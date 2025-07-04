//! # Deltasafe
//! 
//! Deltasafe, yerel ağ (LAN) üzerinde dosyaları güvenli ve verimli bir şekilde 
//! senkronize etmek için tasarlanmış bir Rust kütüphanesidir.
//! 
//! ## Özellikler
//! 
//! - **AES-256-CBC Şifreleme**: Tüm dosya parçaları endüstri standardı şifreleme ile korunur
//! - **BLAKE3 Hash Doğrulaması**: Dosya bütünlüğü garantisi
//! - **Chunk-based Transfer**: Büyük dosyalar için verimli parça parça aktarım
//! - **Progress Tracking**: Transfer ilerlemesi takibi
//! - **Robust Error Handling**: Kapsamlı hata yönetimi
//! 
//! ## Kullanım
//! 
//! ```rust,no_run
//! use deltasafe::sync::{start_sync, calculate_file_hash};
//! use std::path::Path;
//! 
//! // Dosya hash'i hesaplama
//! let hash = calculate_file_hash(Path::new("example.txt")).unwrap();
//! println!("Dosya hash'i: {}", hash);
//! 
//! // Senkronizasyon başlatma
//! let key = [0u8; 32]; // 32 baytlık AES anahtarı
//! start_sync("./source_folder", "192.168.1.100:12345", &key);
//! ```

pub mod cli;
pub mod sync;
pub mod server;
pub mod crypto;
pub mod discovery;

pub use cli::{Cli, Commands};
pub use sync::{start_sync, calculate_file_hash, FileHeader, CHUNK_SIZE};
pub use server::start_server;