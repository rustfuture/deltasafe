use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;
use std::net::TcpStream;
use std::io::{Write};
use blake3;
use aes::Aes256;
use cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use rand::Rng;


const CHUNK_SIZE: usize = 4096; // 4 KB

/// Bir dosyayı CHUNK_SIZE boyutunda parçalara ayır ve her bir parçanın hash'ini döndür.
pub fn chunk_file_hashes(path: &Path) -> Vec<String> {
    let mut chunk_hashes = Vec::new();

    let file = File::open(path).expect("Dosya açılamadı");
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0u8; CHUNK_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }

        let chunk_data = &buffer[..bytes_read];
        let hash = blake3::hash(chunk_data);
        chunk_hashes.push(hash.to_hex().to_string());
    }

    chunk_hashes
}

/// TCP üzerinden chunk'ı hedef IP'ye gönderir.
pub fn send_chunk_to_server(target: &str, chunk_data: &[u8], key: &[u8; 32]) {
    let (encrypted_chunk, iv) = encrypt_chunk(chunk_data, key);
    let mut payload = Vec::new();
    payload.extend_from_slice(&iv); // IV başa ekleniyor
    payload.extend_from_slice(&encrypted_chunk); // Şifreli veri

    match TcpStream::connect(target) {
        Ok(mut stream) => {
            println!("[📡] Bağlantı kuruldu: {}", target);
            // Şifreli chunk'ı gönder
            if let Err(e) = stream.write(&payload) {
                println!("[⚠️] Chunk gönderilemedi: {}", e);
            } else {
                println!("[📤] Chunk başarıyla gönderildi.");
            }
        },
        Err(e) => {
            println!("[⚠️] Bağlantı hatası: {}", e);
        }
    }
}

pub fn start_sync(source: &str, target: &str, key: &[u8; 32]) {
    println!("[🔍] Kaynak klasör taranıyor: {}", source);

    let path = Path::new(source);
    if !path.exists() || !path.is_dir() {
        eprintln!("Hata: '{}' bir klasör değil veya bulunamadı.", source);
        return;
    }

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();

        if file_path.is_file() {
            let _chunk_hashes = chunk_file_hashes(&file_path);
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
                send_chunk_to_server(target, chunk_data, key);
            }
        }
    }

    println!("[🚀] Chunk'lar gönderildi.");
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


