//! # Deltasafe
//!
//! Deltasafe is a Rust library for authenticated file transfer on a trusted LAN.
//!
//! ## Özellikler
//!
//! - **AES-256-GCM**: Data and control frames are authenticated.
//! - **BLAKE3 verification**: A file is published only after its full digest matches.
//! - **Bounded chunk transfer**: Large files are streamed without loading them into memory.
//! - **Safe publication**: Incomplete files are temporary and existing destinations are preserved.
//!
//! ## Kullanım
//!
//! ```rust,no_run
//! use deltasafe::sync::{start_sync, calculate_file_hash};
//! use std::path::Path;
//!
//! // Calculate a file digest.
//! let hash = calculate_file_hash(Path::new("example.txt")).unwrap();
//! println!("file digest: {hash}");
//!
//! // A real transfer returns an error if the receiver does not verify it.
//! # let key = [0u8; 32];
//! # start_sync("./source_folder", "192.168.1.100:12345", &key, None)?;
//! # Ok::<(), anyhow::Error>(())
//! ```

pub mod cli;
pub mod crypto;
pub mod discovery;
mod protocol;
pub mod server;
pub mod sync;
pub mod utils;

pub use cli::{Cli, Commands};
pub use server::start_server;
pub use sync::{calculate_file_hash, start_sync, FileHeader, CHUNK_SIZE};
