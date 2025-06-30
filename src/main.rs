mod cli;
mod sync;
mod server;

use cli::{Cli, Commands};
use clap::Parser;
use sync::start_sync;
use server::start_server;


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Sync { source, target, key } => {
            println!("Sync başlatılıyor: {} -> {}", source, target);
            let key_bytes: [u8; 32] = hex::decode(key).expect("Anahtar hex formatında olmalı ve 32 bayt uzunluğunda olmalı").try_into().expect("Anahtar 32 bayt uzunluğunda olmalı");
            start_sync(source, target, &key_bytes);
        },
        Commands::Connect { ip } => {
            println!("Peer cihaza bağlanılıyor: {}", ip);
            // Connect komutu için henüz bir fonksiyon yok, buraya eklenecek.
        },
        Commands::Watch { folder } => {
            println!("Klasör izleniyor: {}", folder);
            // Watch komutu için henüz bir fonksiyon yok, buraya eklenecek.
        },
        Commands::Server { address, key } => {
            println!("Sunucu başlatılıyor: {}", address);
            let key_bytes: [u8; 32] = hex::decode(key).expect("Anahtar hex formatında olmalı ve 32 bayt uzunluğunda olmalı").try_into().expect("Anahtar 32 bayt uzunluğunda olmalı");
            start_server(address, &key_bytes);
        },
    }
}