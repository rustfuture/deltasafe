mod cli;
mod sync;
mod server;
mod crypto;
mod discovery;

use cli::{Cli, Commands};
use clap::Parser;
use sync::start_sync;
use server::start_server;
use crypto::{derive_key_from_password, parse_hex_key, validate_password_strength, generate_random_hex_key};
use discovery::{discover_servers, select_server_interactive, select_best_server_auto};
use anyhow::{Result, Context};


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run_command(&cli.command).await {
        eprintln!("[❌] Hata: {}", e);
        std::process::exit(1);
    }
}

async fn run_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Sync { source, target, auto, auto_select, key, password } => {
            let target_address = resolve_target_address(target.as_deref(), *auto, *auto_select).await?;
            println!("Sync başlatılıyor: {} -> {}", source, target_address);
            
            let key_bytes = resolve_key(key.as_deref(), password.as_deref())?;
            start_sync(source, &target_address, &key_bytes);
        },
        Commands::Discover { timeout } => {
            let servers = discover_servers(*timeout).await?;
            if servers.is_empty() {
                println!("[ℹ️] Hiç sunucu bulunamadı.");
            } else {
                println!("[✅] Bulunan sunucular:");
                for (i, server) in servers.iter().enumerate() {
                    println!("  {}. {} ({:?})", i + 1, server.address, server.discovery_method);
                    if let Some(name) = &server.name {
                        println!("     Servis adı: {}", name);
                    }
                }
            }
        },
        Commands::Connect { ip } => {
            println!("Peer cihaza bağlanılıyor: {}", ip);
            println!("⚠️ Bu özellik henüz geliştirilme aşamasındadır.");
        },
        Commands::Watch { folder } => {
            println!("Klasör izleniyor: {}", folder);
            println!("⚠️ Bu özellik henüz geliştirilme aşamasındadır.");
        },
        Commands::Server { address, key, password } => {
            let server_address = resolve_server_address(address.as_deref())?;
            println!("Sunucu başlatılıyor: {}", server_address);
            
            let key_bytes = resolve_key(key.as_deref(), password.as_deref())?;
            start_server(&server_address, &key_bytes);
        },
    }
    Ok(())
}

/// Anahtar veya şifreden AES anahtarı çözümler
fn resolve_key(key: Option<&str>, password: Option<&str>) -> Result<[u8; 32]> {
    match (key, password) {
        (Some(hex_key), None) => {
            parse_hex_key(hex_key)
        },
        (None, Some(pwd)) => {
            validate_password_strength(pwd)?;
            derive_key_from_password(pwd, None)
        },
        (None, None) => {
            // Geçici anahtar üret ve kullanıcıya göster
            let temp_key = generate_random_hex_key();
            println!("🔑 Geçici anahtar oluşturuldu: {}", temp_key);
            println!("💡 Bu anahtarı karşı tarafa da verin veya --password kullanın");
            parse_hex_key(&temp_key)
        },
        (Some(_), Some(_)) => {
            anyhow::bail!("Hem --key hem --password belirtilemez, birini seçin")
        }
    }
}

/// Hedef adresini çözümler (sync için)
async fn resolve_target_address(target: Option<&str>, auto_discover: bool, auto_select: bool) -> Result<String> {
    match (target, auto_discover) {
        (Some(addr), false) => Ok(addr.to_string()),
        (None, true) => {
            println!("[🔍] Otomatik sunucu keşfi başlatılıyor...");
            let servers = discover_servers(5).await?;
            
            if servers.is_empty() {
                anyhow::bail!("Hiç sunucu bulunamadı. Manuel IP:port belirtin veya önce sunucu başlatın.");
            }
            
            let selected_server = if auto_select {
                // Otomatik seçim (kullanıcı etkileşimi olmadan)
                select_best_server_auto(&servers)
                    .context("Otomatik sunucu seçimi başarısız")?
            } else {
                // Kullanıcıya seçim yaptır
                select_server_interactive(&servers)
                    .context("Sunucu seçimi iptal edildi")?
            };
            
            println!("[✅] Sunucu seçildi: {} ({:?})", selected_server.address, selected_server.discovery_method);
            Ok(selected_server.address.to_string())
        },
        (None, false) => {
            anyhow::bail!("Hedef adres belirtilmeli (--target) veya otomatik keşif kullanılmalı (--auto)")
        },
        (Some(_), true) => {
            anyhow::bail!("Hem --target hem --auto belirtilemez, birini seçin")
        }
    }
}

/// Sunucu adresini çözümler
fn resolve_server_address(address: Option<&str>) -> Result<String> {
    match address {
        Some(addr) => Ok(addr.to_string()),
        None => {
            // Otomatik IP detection
            let local_ip = get_local_ip()?;
            let default_port = 12345;
            let server_address = format!("{}:{}", local_ip, default_port);
            println!("🌐 Otomatik adres: {}", server_address);
            Ok(server_address)
        }
    }
}

/// Yerel IP adresini bulur
fn get_local_ip() -> Result<String> {
    use std::net::{TcpStream, SocketAddr};
    
    // Google DNS'e bağlanarak yerel IP'yi öğren (gerçekte bağlanmaz)
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")
        .context("UDP socket oluşturulamadı")?;
    socket.connect("8.8.8.8:80")
        .context("Test bağlantısı kurulamadı")?;
    
    let local_addr = socket.local_addr()
        .context("Yerel adres alınamadı")?;
    
    Ok(local_addr.ip().to_string())
}