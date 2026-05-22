use anyhow::{Result, Context};
use crate::crypto::{
    derive_key_from_password, parse_hex_key, validate_password_strength,
    generate_random_hex_key, generate_random_salt,
};
use crate::discovery::{discover_servers, select_server_interactive, select_best_server_auto};

/// Çözümlenmiş AES anahtarı ve opsiyonel PBKDF2 salt bilgisi
pub struct ResolvedKey {
    pub key: [u8; 32],
    pub pbkdf2_salt: Option<[u8; 16]>,
}

/// Anahtar çözümleme bağlamı
pub enum KeyRole {
    Client,
    Server,
}

/// Anahtar veya şifreden AES anahtarı çözümler
pub fn resolve_key(key: Option<&str>, password: Option<&str>, role: KeyRole) -> Result<ResolvedKey> {
    match (key, password) {
        (Some(hex_key), None) => {
            Ok(ResolvedKey {
                key: parse_hex_key(hex_key)?,
                pbkdf2_salt: None,
            })
        },
        (None, Some(pwd)) => {
            validate_password_strength(pwd)?;
            match role {
                KeyRole::Client => {
                    let salt = generate_random_salt();
                    let derived_key = derive_key_from_password(pwd, Some(&salt))?;
                    Ok(ResolvedKey {
                        key: derived_key,
                        pbkdf2_salt: Some(salt),
                    })
                },
                KeyRole::Server => {
                    let derived_key = derive_key_from_password(pwd, None)?;
                    Ok(ResolvedKey {
                        key: derived_key,
                        pbkdf2_salt: None,
                    })
                }
            }
        },
        (None, None) => {
            let temp_key = generate_random_hex_key();
            println!("🔑 Geçici anahtar oluşturuldu: {}", temp_key);
            println!("💡 Bu anahtarı karşı tarafa da verin veya --password kullanın");
            Ok(ResolvedKey {
                key: parse_hex_key(&temp_key)?,
                pbkdf2_salt: None,
            })
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
                select_best_server_auto(&servers)
                    .context("Otomatik sunucu seçimi başarısız")?
            } else {
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
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")
        .context("UDP socket oluşturulamadı")?;
    socket.connect("8.8.8.8:80")
        .context("Test bağlantısı kurulamadı")?;

    let local_addr = socket.local_addr()
        .context("Yerel adres alınamadı")?;

    Ok(local_addr.ip().to_string())
}
