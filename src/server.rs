use std::fs;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use tempfile::NamedTempFile;

use crate::crypto::derive_key_from_password;
use crate::protocol::{
    decrypt_frame, derive_session_key, encrypt_frame, read_frame, read_header_bytes, write_frame,
    CLIENT_DIRECTION, KIND_COMPLETE, KIND_DATA, KIND_ERROR, KIND_FINISH, KIND_READY, MAX_FILE_SIZE,
    PROTOCOL_VERSION, SERVER_DIRECTION,
};
use crate::sync::{calculate_file_hash, FileHeader, CHUNK_SIZE};

const IO_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_RECEIVE_ROOT: &str = "received_files";

struct Destination {
    root: PathBuf,
    parent: PathBuf,
    final_path: PathBuf,
}

/// Starts the receiver and serves connections until the listener fails.
pub fn start_server(address: &str, key: &[u8; 32], password: Option<String>) -> Result<()> {
    let listener = TcpListener::bind(address)
        .with_context(|| format!("Could not bind receiver to {address}"))?;
    println!("Receiver listening on {address}");

    for incoming in listener.incoming() {
        let stream = incoming.context("Could not accept connection")?;
        let key = *key;
        let password = password.clone();
        std::thread::spawn(move || {
            if let Err(error) = handle_client(
                stream,
                key,
                password.as_deref(),
                Path::new(DEFAULT_RECEIVE_ROOT),
            ) {
                eprintln!("Transfer connection failed: {error:#}");
            }
        });
    }
    Ok(())
}

/// Serves exactly one connection with an already-bound listener.
///
/// This is useful for deterministic embedding and end-to-end tests with an
/// ephemeral port; it does not spawn a detached connection thread.
pub fn serve_once(
    listener: TcpListener,
    key: &[u8; 32],
    password: Option<&str>,
    receive_root: &Path,
) -> Result<()> {
    let (stream, _) = listener.accept().context("Could not accept connection")?;
    handle_client(stream, *key, password, receive_root)
}

fn handle_client(
    mut stream: TcpStream,
    default_key: [u8; 32],
    password: Option<&str>,
    receive_root: &Path,
) -> Result<()> {
    configure_stream(&stream)?;

    loop {
        let Some(header_bytes) = read_header_bytes(&mut stream)? else {
            return Ok(());
        };
        let header: FileHeader =
            serde_json::from_slice(&header_bytes).context("File header is not valid JSON")?;
        let session_id = validate_header(&header)?;
        let base_key = resolve_transfer_key(&header, &default_key, password)?;
        let session_key = derive_session_key(&base_key, &session_id)?;
        let destination = prepare_destination(receive_root, &header.relative_path)?;

        send_status(&mut stream, &header_bytes, &session_key, 0, KIND_READY)?;
        match receive_file(
            &mut stream,
            &header_bytes,
            &header,
            &session_key,
            &destination,
        ) {
            Ok(()) => {
                send_status(&mut stream, &header_bytes, &session_key, 1, KIND_COMPLETE)?;
            }
            Err(error) => {
                let _ = send_status(&mut stream, &header_bytes, &session_key, 1, KIND_ERROR);
                return Err(error);
            }
        }
    }
}

fn validate_header(header: &FileHeader) -> Result<[u8; 16]> {
    if header.protocol_version != PROTOCOL_VERSION {
        bail!(
            "Unsupported protocol version: expected {PROTOCOL_VERSION}, got {}",
            header.protocol_version
        );
    }
    if header.file_size > MAX_FILE_SIZE {
        bail!(
            "Declared file size {} exceeds the {MAX_FILE_SIZE}-byte limit",
            header.file_size
        );
    }
    let session = hex::decode(&header.session_id).context("Session id is not valid hex")?;
    let session_id: [u8; 16] = session
        .try_into()
        .map_err(|_| anyhow!("Session id must be exactly 16 bytes"))?;
    let digest = hex::decode(&header.file_hash).context("File hash is not valid hex")?;
    if digest.len() != 32 {
        bail!("File hash must be a 32-byte BLAKE3 digest");
    }
    validate_relative_path(&header.relative_path)?;
    let path_name = header
        .relative_path
        .file_name()
        .and_then(|name| name.to_str())
        .context("Destination filename is not valid UTF-8")?;
    if path_name != header.file_name {
        bail!("Header filename does not match the relative path");
    }
    Ok(session_id)
}

fn validate_relative_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty() {
        bail!("Destination path is empty");
    }
    let normalized: PathBuf = path.components().collect();
    if normalized != path {
        bail!("Destination path is not lexically normalized");
    }
    for component in path.components() {
        if !matches!(component, Component::Normal(_)) {
            bail!("Destination path contains a forbidden component");
        }
    }
    Ok(())
}

fn resolve_transfer_key(
    header: &FileHeader,
    default_key: &[u8; 32],
    password: Option<&str>,
) -> Result<[u8; 32]> {
    match (&header.pbkdf2_salt, password) {
        (Some(salt_hex), Some(password)) => {
            let salt = hex::decode(salt_hex).context("PBKDF2 salt is not valid hex")?;
            let salt: [u8; 16] = salt
                .try_into()
                .map_err(|_| anyhow!("PBKDF2 salt must be exactly 16 bytes"))?;
            derive_key_from_password(password, Some(&salt))
        }
        (None, None) => Ok(*default_key),
        (Some(_), None) => bail!("Sender used password mode but receiver did not"),
        (None, Some(_)) => bail!("Receiver requires password mode but sender did not use it"),
    }
}

fn prepare_destination(receive_root: &Path, relative_path: &Path) -> Result<Destination> {
    validate_relative_path(relative_path)?;
    fs::create_dir_all(receive_root)
        .with_context(|| format!("Could not create receive root {}", receive_root.display()))?;
    let root = receive_root
        .canonicalize()
        .with_context(|| format!("Could not resolve receive root {}", receive_root.display()))?;

    let mut parent = root.clone();
    if let Some(relative_parent) = relative_path.parent() {
        for component in relative_parent.components() {
            let Component::Normal(name) = component else {
                bail!("Destination parent contains a forbidden component");
            };
            parent.push(name);
            match fs::symlink_metadata(&parent) {
                Ok(metadata) if metadata.file_type().is_symlink() => {
                    bail!("Destination parent contains a symbolic link");
                }
                Ok(metadata) if !metadata.is_dir() => {
                    bail!("Destination parent component is not a directory");
                }
                Ok(_) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    fs::create_dir(&parent).with_context(|| {
                        format!(
                            "Could not create destination directory {}",
                            parent.display()
                        )
                    })?;
                }
                Err(error) => return Err(error).context("Could not inspect destination parent"),
            }
        }
    }

    let canonical_parent = parent
        .canonicalize()
        .with_context(|| format!("Could not resolve destination parent {}", parent.display()))?;
    if !canonical_parent.starts_with(&root) {
        bail!("Destination escapes the receive root");
    }
    let file_name = relative_path
        .file_name()
        .context("Destination path has no filename")?;
    let final_path = canonical_parent.join(file_name);
    match fs::symlink_metadata(&final_path) {
        Ok(_) => bail!("Destination already exists: {}", final_path.display()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error).context("Could not inspect destination file"),
    }

    Ok(Destination {
        root,
        parent: canonical_parent,
        final_path,
    })
}

fn receive_file(
    stream: &mut TcpStream,
    header_bytes: &[u8],
    header: &FileHeader,
    session_key: &[u8; 32],
    destination: &Destination,
) -> Result<()> {
    let mut pending = NamedTempFile::new_in(&destination.parent)
        .context("Could not create a temporary destination file")?;
    let mut total_written = 0u64;
    let mut expected_index = 0u64;

    loop {
        let frame = read_frame(stream)?.context("Connection closed before FINISH frame")?;
        if frame.index != expected_index {
            bail!(
                "Unexpected frame index: expected {expected_index}, got {}",
                frame.index
            );
        }
        match frame.kind {
            KIND_DATA => {
                let plaintext = decrypt_frame(header_bytes, session_key, CLIENT_DIRECTION, &frame)?;
                if plaintext.is_empty() || plaintext.len() > CHUNK_SIZE {
                    bail!("Invalid plaintext chunk size: {}", plaintext.len());
                }
                let next_total = total_written
                    .checked_add(plaintext.len() as u64)
                    .context("Received byte count overflow")?;
                if next_total > header.file_size {
                    bail!("Received more data than declared in the file header");
                }
                pending
                    .as_file_mut()
                    .write_all(&plaintext)
                    .context("Could not write temporary file")?;
                total_written = next_total;
                expected_index = expected_index
                    .checked_add(1)
                    .context("Frame index overflow")?;
            }
            KIND_FINISH => {
                let plaintext = decrypt_frame(header_bytes, session_key, CLIENT_DIRECTION, &frame)?;
                if !plaintext.is_empty() {
                    bail!("FINISH frame must not contain file data");
                }
                if total_written != header.file_size {
                    bail!(
                        "File ended at {total_written} bytes; expected {}",
                        header.file_size
                    );
                }
                break;
            }
            kind => bail!("Unexpected sender frame kind: {kind}"),
        }
    }

    pending
        .as_file_mut()
        .flush()
        .context("Could not flush temporary file")?;
    pending
        .as_file()
        .sync_all()
        .context("Could not sync temporary file")?;
    let actual_hash =
        calculate_file_hash(pending.path()).context("Could not hash received file")?;
    if actual_hash != header.file_hash {
        bail!("Received file hash does not match the declared digest");
    }

    let parent_now = destination
        .parent
        .canonicalize()
        .context("Could not revalidate destination parent")?;
    if !parent_now.starts_with(&destination.root) || parent_now != destination.parent {
        bail!("Destination parent changed during transfer");
    }
    pending
        .persist_noclobber(&destination.final_path)
        .map_err(|error| error.error)
        .with_context(|| {
            format!(
                "Could not publish received file at {}",
                destination.final_path.display()
            )
        })?;
    Ok(())
}

fn send_status(
    stream: &mut TcpStream,
    header_bytes: &[u8],
    session_key: &[u8; 32],
    index: u64,
    kind: u8,
) -> Result<()> {
    let body = encrypt_frame(
        header_bytes,
        session_key,
        SERVER_DIRECTION,
        kind,
        index,
        &[],
    )?;
    write_frame(stream, &body)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Shutdown;

    use crate::protocol::{read_frame, write_header};

    fn test_header(data: &[u8]) -> (FileHeader, Vec<u8>, [u8; 32]) {
        let session_id = [3u8; 16];
        let base_key = [7u8; 32];
        let header = FileHeader {
            protocol_version: PROTOCOL_VERSION,
            session_id: hex::encode(session_id),
            file_name: "payload.bin".to_owned(),
            file_size: data.len() as u64,
            file_hash: blake3::hash(data).to_hex().to_string(),
            relative_path: PathBuf::from("payload.bin"),
            pbkdf2_salt: None,
        };
        let header_bytes = serde_json::to_vec(&header).unwrap();
        let session_key = derive_session_key(&base_key, &session_id).unwrap();
        (header, header_bytes, session_key)
    }

    fn assert_status(
        stream: &mut TcpStream,
        header_bytes: &[u8],
        session_key: &[u8; 32],
        expected_index: u64,
        expected_kind: u8,
    ) {
        let frame = read_frame(stream).unwrap().unwrap();
        assert_eq!(frame.index, expected_index);
        assert_eq!(frame.kind, expected_kind);
        let plaintext = decrypt_frame(header_bytes, session_key, SERVER_DIRECTION, &frame).unwrap();
        assert!(plaintext.is_empty());
    }

    #[test]
    fn rejects_unsafe_relative_paths() {
        assert!(validate_relative_path(Path::new("nested/file.txt")).is_ok());
        assert!(validate_relative_path(Path::new("")).is_err());
        assert!(validate_relative_path(Path::new("../outside.txt")).is_err());
        assert!(validate_relative_path(Path::new("/tmp/outside.txt")).is_err());
    }

    #[test]
    fn rejects_existing_destination() {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("existing.txt"), b"keep me").unwrap();
        assert!(prepare_destination(root.path(), Path::new("existing.txt")).is_err());
        assert_eq!(
            fs::read(root.path().join("existing.txt")).unwrap(),
            b"keep me"
        );
    }

    #[test]
    fn corrupted_frame_is_rejected_without_publishing_a_file() {
        let sandbox = tempfile::tempdir().unwrap();
        let receive_root = sandbox.path().join("received");
        let thread_root = receive_root.clone();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server =
            std::thread::spawn(move || serve_once(listener, &[7u8; 32], None, &thread_root));
        let mut stream = TcpStream::connect(address).unwrap();
        configure_stream(&stream).unwrap();
        let (_, header_bytes, session_key) = test_header(b"hello");

        write_header(&mut stream, &header_bytes).unwrap();
        assert_status(&mut stream, &header_bytes, &session_key, 0, KIND_READY);
        let mut body = encrypt_frame(
            &header_bytes,
            &session_key,
            CLIENT_DIRECTION,
            KIND_DATA,
            0,
            b"hello",
        )
        .unwrap();
        *body.last_mut().unwrap() ^= 1;
        write_frame(&mut stream, &body).unwrap();

        assert_status(&mut stream, &header_bytes, &session_key, 1, KIND_ERROR);
        assert!(server.join().unwrap().is_err());
        assert!(!receive_root.join("payload.bin").exists());
        assert_eq!(fs::read_dir(&receive_root).unwrap().count(), 0);
    }

    #[test]
    fn truncated_frame_is_rejected_without_leaving_a_temp_file() {
        let sandbox = tempfile::tempdir().unwrap();
        let receive_root = sandbox.path().join("received");
        let thread_root = receive_root.clone();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server =
            std::thread::spawn(move || serve_once(listener, &[7u8; 32], None, &thread_root));
        let mut stream = TcpStream::connect(address).unwrap();
        configure_stream(&stream).unwrap();
        let (_, header_bytes, session_key) = test_header(b"hello");

        write_header(&mut stream, &header_bytes).unwrap();
        assert_status(&mut stream, &header_bytes, &session_key, 0, KIND_READY);
        stream.write_all(&30u32.to_be_bytes()).unwrap();
        stream.write_all(&[KIND_DATA, 0, 0, 0]).unwrap();
        stream.shutdown(Shutdown::Write).unwrap();

        assert_status(&mut stream, &header_bytes, &session_key, 1, KIND_ERROR);
        assert!(server.join().unwrap().is_err());
        assert!(!receive_root.join("payload.bin").exists());
        assert_eq!(fs::read_dir(&receive_root).unwrap().count(), 0);
    }

    #[cfg(unix)]
    #[test]
    fn destination_parent_symlink_is_rejected() {
        use std::os::unix::fs::symlink;

        let receive_root = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        symlink(outside.path(), receive_root.path().join("escape")).unwrap();
        assert!(prepare_destination(receive_root.path(), Path::new("escape/file.txt")).is_err());
        assert!(!outside.path().join("file.txt").exists());
    }
}
