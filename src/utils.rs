use anyhow::{Result, Context};
use crate::crypto::{derive_key_from_password, parse_hex_key, validate_password_strength, generate_random_hex_key};
use crate::discovery::{discover_servers, select_server_interactive, select_best_server_auto};

/// Anahtar veya şifreden AES anahtarı çözümler
pub fn resolve_key(key: Option<&str>, password: Option<&str>) -> Result<[u8; 32]> {
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
pub async fn resolve_target_address(target: Option<&str>, auto_discover: bool, auto_select: bool) -> Result<String> {
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
pub fn resolve_server_address(address: Option<&str>) -> Result<String> {
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
    // Google DNS'e bağlanarak yerel IP'yi öğren (gerçekte bağlanmaz)
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")
        .context("UDP socket oluşturulamadı")?;
    socket.connect("8.8.8.8:80")
        .context("Test bağlantısı kurulamadı")?;
    
    let local_addr = socket.local_addr()
        .context("Yerel adres alınamadı")?;
    
    Ok(local_addr.ip().to_string())
}
