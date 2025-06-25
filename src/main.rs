mod cli;
mod sync;
mod server;

use cli::{Cli, Commands};
use clap::Parser;
use sync::start_sync;
use server::start_server;
use std::io::{self, Write};

fn main() {
    loop {
        println!("\n=== Deltasafe CLI Menü ===");
        println!("1. Sunucu başlat");
        println!("2. Klasör senkronize et");
        println!("3. Çıkış");
        print!("Seçiminizi girin: ");
        io::stdout().flush().unwrap();

        let mut secim = String::new();
        io::stdin().read_line(&mut secim).unwrap();
        let secim = secim.trim();

        match secim {
            "1" => {
                print!("Sunucu adresini girin (örn: 0.0.0.0:12345): ");
                io::stdout().flush().unwrap();
                let mut address = String::new();
                io::stdin().read_line(&mut address).unwrap();
                let address = address.trim();
                println!("Sunucu başlatılıyor: {}", address);
                start_server(address);
            }
            "2" => {
                print!("Kaynak klasör yolunu girin: ");
                io::stdout().flush().unwrap();
                let mut source = String::new();
                io::stdin().read_line(&mut source).unwrap();
                let source = source.trim();
                print!("Hedef IP:PORT adresini girin: ");
                io::stdout().flush().unwrap();
                let mut target = String::new();
                io::stdin().read_line(&mut target).unwrap();
                let target = target.trim();
                println!("Sync başlatılıyor: {} -> {}", source, target);
                start_sync(source, target);
            }
            "3" => {
                println!("Çıkılıyor...");
                break;
            }
            _ => {
                println!("Geçersiz seçim! Lütfen tekrar deneyin.");
            }
        }
    }
}
