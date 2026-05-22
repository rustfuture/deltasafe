use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::net::TcpStream;
use blake3;
use aes::Aes256;
use cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use rand::Rng;
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::{Result, Context};

pub const CHUNK_SIZE: usize = 4096;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileHeader {
    pub file_name: String,
    pub file_size: u64,
    pub file_hash: String,
    pub relative_path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pbkdf2_salt: Option<String>,
}

/// Bir dosyanın tamamının BLAKE3 hash'ini hesaplar.
pub fn calculate_file_hash(path: &Path) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0; 4096];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

fn write_framed_payload(stream: &mut TcpStream, payload: &[u8]) -> Result<()> {
    let len = payload.len() as u32;
    stream.write_all(&len.to_be_bytes())
        .context("Chunk uzunluğu gönderilemedi")?;
    stream.write_all(payload)
        .context("Chunk gönderilemedi")?;
    Ok(())
}

/// TCP üzerinden chunk'ı hedef IP'ye gönderir.
pub fn send_chunk_to_server(
    stream: &mut TcpStream,
    chunk_data: &[u8],
    key: &[u8; 32],
    progress: &ProgressBar,
) -> Result<()> {
    let (encrypted_chunk, iv) = encrypt_chunk(chunk_data, key);
    let mut payload = Vec::with_capacity(iv.len() + encrypted_chunk.len());
    payload.extend_from_slice(&iv);
    payload.extend_from_slice(&encrypted_chunk);

    write_framed_payload(stream, &payload)?;
    progress.inc(chunk_data.len() as u64);
    Ok(())
}

pub fn start_sync(source: &str, target: &str, key: &[u8; 32], pbkdf2_salt: Option<[u8; 16]>) {
    if let Err(e) = sync_files(source, target, key, pbkdf2_salt) {
        eprintln!("[❌] Senkronizasyon hatası: {}", e);
    }
}

fn sync_files(source: &str, target: &str, key: &[u8; 32], pbkdf2_salt: Option<[u8; 16]>) -> Result<()> {
    println!("[🔍] Kaynak klasör taranıyor: {}", source);

    let path = Path::new(source);
    if !path.exists() || !path.is_dir() {
        anyhow::bail!("'{}' bir klasör değil veya bulunamadı.", source);
    }

    let salt_hex = pbkdf2_salt.map(hex::encode);

    let mut files = Vec::new();
    let mut total_size = 0u64;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let file_path = entry.path();
        if file_path.is_file() {
            let metadata = fs::metadata(&file_path)
                .context("Dosya metadata'sı okunamadı")?;
            total_size += metadata.len();
            files.push(file_path.to_path_buf());
        }
    }

    println!("[📊] {} dosya bulundu, toplam boyut: {} bayt", files.len(), total_size);

    let progress = ProgressBar::new(total_size);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-")
    );

    println!("[🔗] Sunucuya bağlanılıyor: {}", target);
    let mut stream = TcpStream::connect(target)
        .context("Sunucuya bağlanılamadı")?;

    println!("[📡] Bağlantı kuruldu: {}", target);

    for file_path in files {
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .context("Geçersiz dosya adı")?
            .to_string();

        let file_metadata = fs::metadata(&file_path)
            .context("Dosya metadata'sı okunamadı")?;
        let file_size = file_metadata.len();

        let relative_path = file_path.strip_prefix(path)
            .context("Relative path hesaplanamadı")?
            .to_path_buf();

        let file_hash = calculate_file_hash(&file_path)
            .context("Dosya hash'i hesaplanamadı")?;

        progress.set_message(format!("Gönderiliyor: {}", file_name));

        let header = FileHeader {
            file_name: file_name.clone(),
            file_size,
            file_hash,
            relative_path,
            pbkdf2_salt: salt_hex.clone(),
        };

        let serialized_header = serde_json::to_string(&header)
            .context("Header serialize edilemedi")?;
        let header_len = serialized_header.len() as u32;

        stream.write_all(&header_len.to_be_bytes())
            .context("Header uzunluğu gönderilemedi")?;
        stream.write_all(serialized_header.as_bytes())
            .context("Header gönderilemedi")?;

        let mut response_buffer = [0; 1];
        stream.read_exact(&mut response_buffer)
            .context("Sunucudan yanıt alınamadı")?;

        if response_buffer[0] != 1 {
            anyhow::bail!("Sunucudan onay alınamadı: {}", file_name);
        }

        let file = File::open(&file_path)
            .context("Dosya açılamadı")?;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0u8; CHUNK_SIZE];

        loop {
            let bytes_read = reader.read(&mut buffer)
                .context("Dosya okunamadı")?;
            if bytes_read == 0 {
                break;
            }
            let chunk_data = &buffer[..bytes_read];
            send_chunk_to_server(&mut stream, chunk_data, key, &progress)
                .context("Chunk gönderilemedi")?;
        }
    }

    progress.finish_with_message("Tüm dosyalar başarıyla gönderildi!");
    println!("[🚀] Senkronizasyon tamamlandı.");
    Ok(())
}

type Aes256CbcEnc = cbc::Encryptor<Aes256>;

fn encrypt_chunk(chunk: &[u8], key: &[u8; 32]) -> (Vec<u8>, Vec<u8>) {
    let mut iv = [0u8; 16];
    rand::thread_rng().fill(&mut iv);

    let mut buf = chunk.to_vec();
    buf.resize(chunk.len() + 16, 0);

    let cipher = Aes256CbcEnc::new(key.into(), &iv.into());
    let ciphertext = cipher.encrypt_padded_mut::<Pkcs7>(&mut buf, chunk.len())
        .expect("Şifreleme hatası")
        .to_vec();
    (ciphertext, iv.to_vec())
}
