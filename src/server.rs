use std::fs::OpenOptions;
use std::io::{Write, Read};
use std::path::Path;
use std::net::{TcpListener};
use aes::Aes256;
use cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};

// Yeni tip tanımı: CBC ile AES256
// Aes256Cbc = Cbc<Aes256, Pkcs7>
type Aes256CbcDec = cbc::Decryptor<Aes256>;


fn decrypt_chunk(ciphertext: &[u8], key: &[u8; 32], iv: &[u8]) -> Vec<u8> {
    let mut buf = ciphertext.to_vec();
    let cipher = Aes256CbcDec::new(key.into(), iv.into());
    let decrypted = cipher.decrypt_padded_mut::<Pkcs7>(&mut buf).unwrap();
    decrypted.to_vec()
}

pub fn start_server(address: &str, key: &[u8; 32]) {
    let listener = TcpListener::bind(address).expect("Sunucu başlatılamadı");

    println!("[📡] Sunucu başlatıldı: {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("[📥] Bağlantı alındı.");
                let mut buffer = [0; 4096]; // 4KB chunk
                let mut file_data: Vec<u8> = Vec::new();

                while let Ok(bytes_read) = stream.read(&mut buffer) {
                    if bytes_read == 0 {
                        break;
                    }
                    // IV ve şifreli veri ayrıştırılıyor
                    if bytes_read > 16 {
                        let iv = &buffer[..16];
                        let ciphertext = &buffer[16..bytes_read];
                        let decrypted = decrypt_chunk(ciphertext, key, iv);
                        file_data.extend_from_slice(&decrypted);
                        println!("[📦] Alınan ve çözülen chunk: {} bayt", decrypted.len());
                    } else {
                        println!("[⚠️] Chunk boyutu çok küçük, IV ve veri ayrıştırılamadı.");
                    }
                }

                // Dosya tamamlandığında, veriyi birleştirip kaydedelim.
                let path = Path::new("received_file");
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(path)
                    .expect("Dosya oluşturulamadı");

                file.write_all(&file_data).expect("Veri dosyaya yazılamadı");

                println!("[📂] Dosya başarıyla alındı ve kaydedildi.");
            }
            Err(e) => {
                println!("[⚠️] Bağlantı hatası: {}", e);
            }
        }
    }
}
