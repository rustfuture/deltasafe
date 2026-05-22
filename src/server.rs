use std::fs::{self, OpenOptions};
use std::io::{Write, Read};
use std::path::Path;
use std::net::{TcpListener, TcpStream};
use aes::Aes256;
use cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use anyhow::{Result, Context, anyhow};

use crate::sync::{FileHeader, calculate_file_hash};
use crate::crypto::derive_key_from_password;

type Aes256CbcDec = cbc::Decryptor<Aes256>;

fn read_exact_or_eof(stream: &mut TcpStream, buf: &mut [u8]) -> Result<bool> {
    match stream.read_exact(buf) {
        Ok(()) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(false),
        Err(e) => Err(e.into()),
    }
}

fn read_file_header(stream: &mut TcpStream) -> Result<Option<FileHeader>> {
    let mut header_len_bytes = [0u8; 4];
    if !read_exact_or_eof(stream, &mut header_len_bytes)? {
        return Ok(None);
    }

    let header_len = u32::from_be_bytes(header_len_bytes) as usize;
    let mut header_buffer = vec![0u8; header_len];
    stream.read_exact(&mut header_buffer)
        .context("Başlık okunamadı")?;

    let header: FileHeader = serde_json::from_slice(&header_buffer)
        .context("Başlık deserialize edilemedi")?;

    Ok(Some(header))
}

fn resolve_transfer_key(
    header: &FileHeader,
    default_key: &[u8; 32],
    password: Option<&str>,
) -> Result<[u8; 32]> {
    match (&header.pbkdf2_salt, password) {
        (Some(salt_hex), Some(pwd)) => {
            let salt = hex::decode(salt_hex)
                .context("Geçersiz PBKDF2 salt formatı")?;
            let salt: [u8; 16] = salt.try_into()
                .map_err(|_| anyhow!("PBKDF2 salt 16 bayt olmalıdır"))?;
            derive_key_from_password(pwd, Some(&salt))
        },
        (Some(_), None) => {
            anyhow::bail!("İstemci oturum salt gönderdi ancak sunucu şifre modunda başlatılmadı")
        },
        (None, _) => Ok(*default_key),
    }
}

fn decrypt_chunk(ciphertext: &[u8], key: &[u8; 32], iv: &[u8]) -> Result<Vec<u8>> {
    let mut buf = ciphertext.to_vec();
    let cipher = Aes256CbcDec::new(key.into(), iv.into());
    let decrypted = cipher.decrypt_padded_mut::<Pkcs7>(&mut buf)
        .map_err(|_| anyhow!("Chunk çözülemedi"))?;
    Ok(decrypted.to_vec())
}

fn read_framed_payload(stream: &mut TcpStream, buffer: &mut Vec<u8>) -> Result<Option<()>> {
    let mut len_bytes = [0u8; 4];
    if !read_exact_or_eof(stream, &mut len_bytes)? {
        return Ok(None);
    }

    let payload_len = u32::from_be_bytes(len_bytes) as usize;
    if payload_len < 16 {
        anyhow::bail!("Geçersiz chunk boyutu: {}", payload_len);
    }

    buffer.resize(payload_len, 0);
    stream.read_exact(buffer)
        .context("Chunk okunamadı")?;

    Ok(Some(()))
}

fn receive_file(stream: &mut TcpStream, header: &FileHeader, key: &[u8; 32]) -> Result<()> {
    let received_dir = Path::new("received_files");
    fs::create_dir_all(received_dir).context("Ana dizin oluşturulamadı")?;

    let full_path = received_dir.join(&header.relative_path);
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).context("Dizin oluşturulamadı")?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&full_path)
        .context("Dosya oluşturulamadı")?;

    let mut total_bytes_written = 0u64;
    let mut payload_buffer = Vec::new();

    while total_bytes_written < header.file_size {
        if read_framed_payload(stream, &mut payload_buffer)?.is_none() {
            anyhow::bail!(
                "Bağlantı erken kapandı: {} ({} / {} bayt alındı)",
                header.file_name,
                total_bytes_written,
                header.file_size
            );
        }

        let iv = &payload_buffer[..16];
        let ciphertext = &payload_buffer[16..];
        let decrypted = decrypt_chunk(ciphertext, key, iv)
            .context("Chunk çözülemedi")?;

        file.write_all(&decrypted)
            .context("Veri dosyaya yazılamadı")?;
        total_bytes_written += decrypted.len() as u64;
    }

    println!(
        "[📂] Dosya '{}' başarıyla alındı. Toplam {} bayt.",
        header.file_name, total_bytes_written
    );

    match calculate_file_hash(&full_path) {
        Ok(calculated_hash) => {
            if calculated_hash == header.file_hash {
                println!("[✅] Dosya hash doğrulaması başarılı: {}", calculated_hash);
            } else {
                println!(
                    "[❌] Dosya hash doğrulaması BAŞARISIZ! Beklenen: {}, Hesaplanan: {}",
                    header.file_hash, calculated_hash
                );
            }
        }
        Err(e) => {
            println!("[⚠️] Kaydedilen dosyanın hash'i hesaplanamadı: {}", e);
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, default_key: [u8; 32], password: Option<String>) {
    println!("[📥] Bağlantı alındı.");

    loop {
        let header = match read_file_header(&mut stream) {
            Ok(Some(header)) => header,
            Ok(None) => break,
            Err(e) => {
                println!("[⚠️] Başlık okunamadı: {}", e);
                break;
            }
        };

        println!("[📄] Alınan dosya başlığı: {:?}", header);

        let transfer_key = match resolve_transfer_key(
            &header,
            &default_key,
            password.as_deref(),
        ) {
            Ok(key) => key,
            Err(e) => {
                println!("[⚠️] Anahtar çözümlenemedi: {}", e);
                let _ = stream.write_all(&[0]);
                break;
            }
        };

        if stream.write_all(&[1]).is_err() {
            println!("[⚠️] İstemciye onay gönderilemedi.");
            break;
        }

        if let Err(e) = receive_file(&mut stream, &header, &transfer_key) {
            println!("[⚠️] Dosya alınamadı: {}", e);
            break;
        }
    }

    println!("[📥] Bağlantı kapatıldı.");
}

pub fn start_server(address: &str, key: &[u8; 32], password: Option<String>) {
    let listener = TcpListener::bind(address).expect("Sunucu başlatılamadı");

    println!("[📡] Sunucu başlatıldı: {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let key_clone = *key;
                let password_clone = password.clone();
                std::thread::spawn(move || {
                    handle_client(stream, key_clone, password_clone);
                });
            }
            Err(e) => {
                println!("[⚠️] Bağlantı hatası: {}", e);
            }
        }
    }
}
