use std::fs::{self, OpenOptions};
use std::io::{Write, Read, BufReader};
use std::path::{Path, PathBuf};
use std::net::{TcpListener, TcpStream};
use aes::Aes256;
use cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use serde::{Serialize, Deserialize};
use serde_json;
use blake3;

// Yeni tip tanımı: CBC ile AES256
// Aes256Cbc = Cbc<Aes256, Pkcs7>
type Aes256CbcDec = cbc::Decryptor<Aes256>;

#[derive(Serialize, Deserialize, Debug)]
struct FileHeader {
    file_name: String,
    file_size: u64,
    file_hash: String,
    relative_path: PathBuf,
}

fn decrypt_chunk(ciphertext: &[u8], key: &[u8; 32], iv: &[u8]) -> Vec<u8> {
    let mut buf = ciphertext.to_vec();
    let cipher = Aes256CbcDec::new(key.into(), iv.into());
    let decrypted = cipher.decrypt_padded_mut::<Pkcs7>(&mut buf).unwrap();
    decrypted.to_vec()
}

/// Bir dosyanın tamamının BLAKE3 hash'ini hesaplar.
fn calculate_file_hash(path: &Path) -> Result<String, std::io::Error> {
    let file = fs::File::open(path)?;
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

fn handle_client(mut stream: TcpStream, key: &[u8; 32]) {
    println!("[📥] Bağlantı alındı.");

    // 1. Başlık uzunluğunu oku (4 bayt)
    let mut header_len_bytes = [0; 4];
    if stream.read_exact(&mut header_len_bytes).is_err() {
        println!("[⚠️] Başlık uzunluğu okunamadı.");
        return;
    }
    let header_len = u32::from_be_bytes(header_len_bytes) as usize;

    // 2. Başlığı oku
    let mut header_buffer = vec![0; header_len];
    if stream.read_exact(&mut header_buffer).is_err() {
        println!("[⚠️] Başlık okunamadı.");
        return;
    }

    // 3. Başlığı deserialize et
    let header: FileHeader = match serde_json::from_slice(&header_buffer) {
        Ok(h) => h,
        Err(e) => {
            println!("[⚠️] Başlık deserialize edilemedi: {}", e);
            return;
        }
    };

    println!("[📄] Alınan dosya başlığı: {:?}", header);

    // Hedef yolu oluştur ve dizinleri oluştur
    let received_dir = Path::new("received_files");
    fs::create_dir_all(received_dir).expect("Ana dizin oluşturulamadı");
    
    let full_path = received_dir.join(&header.relative_path);
    if let Some(parent) = full_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            println!("[⚠️] Dizin oluşturulamadı: {}", e);
            return;
        }
    }

    // 4. İstemciye onay gönder
    if stream.write_all(&[1]).is_err() {
        println!("[⚠️] İstemciye onay gönderilemedi.");
        return;
    }

    let path = &full_path;
    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)  // Mevcut dosyayı temizle
        .open(path) {
        Ok(f) => f,
        Err(e) => {
            println!("[⚠️] Dosya oluşturulamadı: {}", e);
            return;
        }
    };

    let mut total_bytes_read = 0;
    let mut buffer = [0; 4096]; // 4KB chunk

    while total_bytes_read < header.file_size {
        let bytes_to_read = std::cmp::min(buffer.len(), (header.file_size - total_bytes_read) as usize);
        let bytes_read = match stream.read_exact(&mut buffer[..bytes_to_read]) {
            Ok(_) => bytes_to_read,
            Err(e) => {
                println!("[⚠️] Chunk okuma hatası: {}", e);
                break;
            }
        };

        // IV ve şifreli veri ayrıştırılıyor
        if bytes_read > 16 {
            let iv = &buffer[..16];
            let ciphertext = &buffer[16..bytes_read];
            let decrypted = decrypt_chunk(ciphertext, key, iv);
            file.write_all(&decrypted).expect("Veri dosyaya yazılamadı");
            println!("[📦] Alınan ve çözülen chunk: {} bayt", decrypted.len());
            total_bytes_read += decrypted.len() as u64;
        } else {
            println!("[⚠️] Chunk boyutu çok küçük, IV ve veri ayrıştırılamadı.");
            break;
        }
    }

    println!("[📂] Dosya '{}' başarıyla alındı ve kaydedildi. Toplam {} bayt.", header.file_name, total_bytes_read);

    // Dosya hash'ini doğrula
    match calculate_file_hash(path) {
        Ok(calculated_hash) => {
            if calculated_hash == header.file_hash {
                println!("[✅] Dosya hash doğrulaması başarılı: {}", calculated_hash);
            } else {
                println!("[❌] Dosya hash doğrulaması BAŞARISIZ! Beklenen: {}, Hesaplanan: {}", header.file_hash, calculated_hash);
            }
        }
        Err(e) => {
            println!("[⚠️] Kaydedilen dosyanın hash'i hesaplanamadı: {}", e);
        }
    }
}

pub fn start_server(address: &str, key: &[u8; 32]) {
    let listener = TcpListener::bind(address).expect("Sunucu başlatılamadı");

    println!("[📡] Sunucu başlatıldı: {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Her bağlantıyı ayrı bir thread'de ele al
                let key_clone = key.clone();
                std::thread::spawn(move || {
                    handle_client(stream, &key_clone);
                });
            }
            Err(e) => {
                println!("[⚠️] Bağlantı hatası: {}", e);
            }
        }
    }
}
