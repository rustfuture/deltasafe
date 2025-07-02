use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::net::TcpStream;
use std::io::{Write};
use blake3;
use aes::Aes256;
use cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use rand::Rng;
use serde::{Serialize, Deserialize};
use serde_json;
use walkdir::WalkDir;


const CHUNK_SIZE: usize = 4096; // 4 KB

#[derive(Serialize, Deserialize, Debug)]
struct FileHeader {
    file_name: String,
    file_size: u64,
    file_hash: String,
    relative_path: PathBuf,
}

/// Bir dosyanın tamamının BLAKE3 hash'ini hesaplar.
fn calculate_file_hash(path: &Path) -> Result<String, std::io::Error> {
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

/// TCP üzerinden chunk'ı hedef IP'ye gönderir.
pub fn send_chunk_to_server(stream: &mut TcpStream, chunk_data: &[u8], key: &[u8; 32]) -> Result<(), std::io::Error> {
    let (encrypted_chunk, iv) = encrypt_chunk(chunk_data, key);
    let mut payload = Vec::new();
    payload.extend_from_slice(&iv); // IV başa ekleniyor
    payload.extend_from_slice(&encrypted_chunk); // Şifreli veri

    // Şifreli chunk'ı gönder
    if let Err(e) = stream.write_all(&payload) {
        println!("[⚠️] Chunk gönderilemedi: {}", e);
        Err(e)
    } else {
        println!("[📤] Chunk başarıyla gönderildi.");
        Ok(())
    }
}

pub fn start_sync(source: &str, target: &str, key: &[u8; 32]) {
    println!("[🔍] Kaynak klasör taranıyor: {}", source);

    let path = Path::new(source);
    if !path.exists() || !path.is_dir() {
        eprintln!("Hata: '{}' bir klasör değil veya bulunamadı.", source);
        return;
    }

    match TcpStream::connect(target) {
        Ok(mut stream) => {
            println!("[📡] Bağlantı kuruldu: {}", target);

            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                let file_path = entry.path();

                if file_path.is_file() {
                    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
                    let file_metadata = fs::metadata(&file_path).unwrap();
                    let file_size = file_metadata.len();
                    let relative_path = file_path.strip_prefix(path).unwrap().to_path_buf();

                    let file_hash = calculate_file_hash(&file_path).expect("Dosya hash'i hesaplanamadı.");

                    let header = FileHeader {
                        file_name,
                        file_size,
                        file_hash,
                        relative_path,
                    };

                    let serialized_header = serde_json::to_string(&header).unwrap();
                    let header_len = serialized_header.len() as u32;

                    // Önce başlık uzunluğunu, sonra başlığı gönder
                    stream.write_all(&header_len.to_be_bytes()).unwrap();
                    stream.write_all(serialized_header.as_bytes()).unwrap();

                    // Sunucudan onay bekle (şimdilik basit bir okuma, sonra geliştirilecek)
                    let mut response_buffer = [0; 1];
                    stream.read_exact(&mut response_buffer).unwrap();
                    if response_buffer[0] != 1 {
                        println!("[⚠️] Sunucudan onay alınamadı, dosya gönderimi iptal edildi.");
                        continue;
                    }


                    
                    println!("[📦] Dosya: {}", file_path.display());

                    let file = File::open(&file_path).expect("Dosya açılamadı");
                    let mut reader = BufReader::new(file);
                    let mut buffer = vec![0u8; CHUNK_SIZE];

                    loop {
                        let bytes_read = reader.read(&mut buffer).unwrap();
                        if bytes_read == 0 {
                            break;
                        }
                        let chunk_data = &buffer[..bytes_read];
                        send_chunk_to_server(&mut stream, chunk_data, key).unwrap();
                    }
                }
            }
            println!("[🚀] Chunk'lar gönderildi.");
        },
        Err(e) => {
            println!("[⚠️] Bağlantı hatası: {}", e);
        }
    }
}

// Şifreleme ve çözme için yeni CBC tipleri
// Encryptor ve Decryptor ayrı ayrı

type Aes256CbcEnc = cbc::Encryptor<Aes256>;




fn encrypt_chunk(chunk: &[u8], key: &[u8; 32]) -> (Vec<u8>, Vec<u8>) {
    let mut iv = [0u8; 16];
    rand::thread_rng().fill(&mut iv);
    let mut buf = chunk.to_vec();
    let cipher = Aes256CbcEnc::new(key.into(), &iv.into());
    let ciphertext = cipher.encrypt_padded_mut::<Pkcs7>(&mut buf, chunk.len()).unwrap().to_vec();
    (ciphertext, iv.to_vec())
}


