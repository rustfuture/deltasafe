use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::net::{Shutdown, TcpStream};
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{bail, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::protocol::{
    decrypt_frame, derive_session_key, encrypt_frame, read_frame, write_frame, write_header,
    CLIENT_DIRECTION, KIND_COMPLETE, KIND_DATA, KIND_ERROR, KIND_FINISH, KIND_READY, MAX_FILE_SIZE,
    MAX_HEADER_SIZE, PROTOCOL_VERSION, SERVER_DIRECTION,
};

pub const CHUNK_SIZE: usize = 4096;
const IO_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileHeader {
    pub protocol_version: u16,
    pub session_id: String,
    pub file_name: String,
    pub file_size: u64,
    pub file_hash: String,
    pub relative_path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pbkdf2_salt: Option<String>,
}

/// Calculates the BLAKE3 digest of a file without loading it into memory.
pub fn calculate_file_hash(path: &Path) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

/// Sends every regular file under `source` and returns only after the receiver
/// authenticates, verifies, and publishes every file.
pub fn start_sync(
    source: &str,
    target: &str,
    key: &[u8; 32],
    pbkdf2_salt: Option<[u8; 16]>,
) -> Result<()> {
    let source_root = Path::new(source);
    if !source_root.is_dir() {
        bail!("'{source}' is not an existing directory");
    }

    let mut files = collect_files(source_root)?;
    files.sort();
    let total_size = files.iter().try_fold(0u64, |total, path| {
        let size = fs::metadata(path)
            .with_context(|| format!("Could not read metadata for {}", path.display()))?
            .len();
        if size > MAX_FILE_SIZE {
            bail!(
                "{} exceeds the per-file limit of {MAX_FILE_SIZE} bytes",
                path.display()
            );
        }
        total
            .checked_add(size)
            .context("Total source size exceeds the supported range")
    })?;

    let progress = ProgressBar::new(total_size);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .context("Could not configure progress display")?
            .progress_chars("##-"),
    );

    let mut stream = TcpStream::connect(target)
        .with_context(|| format!("Could not connect to receiver at {target}"))?;
    configure_stream(&stream)?;

    let salt_hex = pbkdf2_salt.map(hex::encode);
    for file_path in files {
        send_file(
            &mut stream,
            source_root,
            &file_path,
            key,
            salt_hex.as_deref(),
            &progress,
        )?;
    }

    stream
        .shutdown(Shutdown::Write)
        .context("Could not finish the transfer connection")?;
    progress.finish_with_message("All files were verified by the receiver");
    Ok(())
}

fn collect_files(source_root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(source_root) {
        let entry = entry.with_context(|| {
            format!(
                "Could not traverse source directory {}",
                source_root.display()
            )
        })?;
        if entry.file_type().is_file() {
            files.push(entry.into_path());
        }
    }
    Ok(files)
}

fn send_file(
    stream: &mut TcpStream,
    source_root: &Path,
    file_path: &Path,
    base_key: &[u8; 32],
    salt_hex: Option<&str>,
    progress: &ProgressBar,
) -> Result<()> {
    let relative_path = file_path
        .strip_prefix(source_root)
        .with_context(|| format!("Could not make {} relative", file_path.display()))?
        .to_path_buf();
    let file_name = relative_path
        .file_name()
        .and_then(|name| name.to_str())
        .context("Source filename is not valid UTF-8")?
        .to_owned();
    let file_size = fs::metadata(file_path)
        .with_context(|| format!("Could not read metadata for {}", file_path.display()))?
        .len();
    let file_hash = calculate_file_hash(file_path)
        .with_context(|| format!("Could not hash {}", file_path.display()))?;

    let mut session_id = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut session_id);
    let header = FileHeader {
        protocol_version: PROTOCOL_VERSION,
        session_id: hex::encode(session_id),
        file_name,
        file_size,
        file_hash,
        relative_path,
        pbkdf2_salt: salt_hex.map(str::to_owned),
    };
    let header_bytes = serde_json::to_vec(&header).context("Could not serialize file header")?;
    if header_bytes.len() > MAX_HEADER_SIZE {
        bail!("File header exceeds the {MAX_HEADER_SIZE}-byte limit");
    }
    let session_key = derive_session_key(base_key, &session_id)?;

    progress.set_message(format!("Sending {}", header.relative_path.display()));
    write_header(stream, &header_bytes)?;
    expect_status(stream, &header_bytes, &session_key, 0, KIND_READY)?;

    let mut reader = BufReader::new(
        File::open(file_path).with_context(|| format!("Could not open {}", file_path.display()))?,
    );
    let mut buffer = [0u8; CHUNK_SIZE];
    let mut index = 0u64;
    loop {
        let bytes_read = reader
            .read(&mut buffer)
            .with_context(|| format!("Could not read {}", file_path.display()))?;
        if bytes_read == 0 {
            break;
        }
        let frame = encrypt_frame(
            &header_bytes,
            &session_key,
            CLIENT_DIRECTION,
            KIND_DATA,
            index,
            &buffer[..bytes_read],
        )?;
        write_frame(stream, &frame)?;
        progress.inc(bytes_read as u64);
        index = index.checked_add(1).context("Frame index overflow")?;
    }

    let finish = encrypt_frame(
        &header_bytes,
        &session_key,
        CLIENT_DIRECTION,
        KIND_FINISH,
        index,
        &[],
    )?;
    write_frame(stream, &finish)?;
    expect_status(stream, &header_bytes, &session_key, 1, KIND_COMPLETE)
        .with_context(|| format!("Receiver rejected {}", header.relative_path.display()))?;
    Ok(())
}

fn expect_status(
    stream: &mut TcpStream,
    header_bytes: &[u8],
    session_key: &[u8; 32],
    expected_index: u64,
    expected_kind: u8,
) -> Result<()> {
    let frame = read_frame(stream)?.context("Receiver closed before sending transfer status")?;
    if frame.index != expected_index {
        bail!(
            "Unexpected receiver status index: expected {expected_index}, got {}",
            frame.index
        );
    }
    let plaintext = decrypt_frame(header_bytes, session_key, SERVER_DIRECTION, &frame)?;
    if !plaintext.is_empty() {
        bail!("Receiver status contained unexpected data");
    }
    match frame.kind {
        kind if kind == expected_kind => Ok(()),
        KIND_ERROR => bail!("Receiver reported a transfer error"),
        kind => bail!("Unexpected receiver status kind: {kind}"),
    }
}

fn configure_stream(stream: &TcpStream) -> Result<()> {
    stream
        .set_read_timeout(Some(IO_TIMEOUT))
        .context("Could not set read timeout")?;
    stream
        .set_write_timeout(Some(IO_TIMEOUT))
        .context("Could not set write timeout")?;
    stream
        .set_nodelay(true)
        .context("Could not enable TCP_NODELAY")?;
    Ok(())
}
