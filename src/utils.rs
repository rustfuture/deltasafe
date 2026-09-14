use crate::crypto::{
    derive_key_from_password, generate_random_hex_key, generate_random_salt, parse_hex_key,
    validate_password_strength,
};
use crate::discovery::{discover_servers, select_best_server_auto, select_server_interactive};
use anyhow::{Context, Result};

/// Resolved AES key and optional PBKDF2 salt
pub struct ResolvedKey {
    pub key: [u8; 32],
    pub pbkdf2_salt: Option<[u8; 16]>,
}

/// Key resolution context
pub enum KeyRole {
    Client,
    Server,
}

/// Resolves the AES key from a key or password
pub fn resolve_key(
    key: Option<&str>,
    password: Option<&str>,
    role: KeyRole,
) -> Result<ResolvedKey> {
    match (key, password) {
        (Some(hex_key), None) => Ok(ResolvedKey {
            key: parse_hex_key(hex_key)?,
            pbkdf2_salt: None,
        }),
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
                }
                KeyRole::Server => {
                    let derived_key = derive_key_from_password(pwd, None)?;
                    Ok(ResolvedKey {
                        key: derived_key,
                        pbkdf2_salt: None,
                    })
                }
            }
        }
        (None, None) => {
            let temp_key = generate_random_hex_key();
            println!("🔑 Generated temporary key: {}", temp_key);
            println!("💡 Share this key with the other side or use --password");
            Ok(ResolvedKey {
                key: parse_hex_key(&temp_key)?,
                pbkdf2_salt: None,
            })
        }
        (Some(_), Some(_)) => {
            anyhow::bail!("Cannot specify both --key and --password; choose one")
        }
    }
}

/// Resolves the target address (for sync)
pub async fn resolve_target_address(
    target: Option<&str>,
    auto_discover: bool,
    auto_select: bool,
) -> Result<String> {
    match (target, auto_discover) {
        (Some(addr), false) => Ok(addr.to_string()),
        (None, true) => {
            println!("[🔍] Starting automatic server discovery...");
            let servers = discover_servers(5).await?;

            if servers.is_empty() {
                anyhow::bail!(
                    "No servers found. Specify an IP:port manually or start a server first."
                );
            }

            let selected_server = if auto_select {
                select_best_server_auto(&servers).context("Automatic server selection failed")?
            } else {
                select_server_interactive(&servers).context("Server selection was cancelled")?
            };

            println!("[✅] Selected server: {}", selected_server.address);
            Ok(selected_server.address.to_string())
        }
        (None, false) => {
            anyhow::bail!(
                "A target address must be specified (--target) or automatic discovery must be used (--auto)"
            )
        }
        (Some(_), true) => {
            anyhow::bail!("Cannot specify both --target and --auto; choose one")
        }
    }
}

/// Resolves the server address
pub fn resolve_server_address(address: Option<&str>) -> Result<String> {
    match address {
        Some(addr) => Ok(addr.to_string()),
        None => {
            let local_ip = get_local_ip()?;
            let default_port = 12345;
            let server_address = format!("{}:{}", local_ip, default_port);
            println!("🌐 Automatic address: {}", server_address);
            Ok(server_address)
        }
    }
}

/// Finds the local IP address
fn get_local_ip() -> Result<String> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").context("Could not create UDP socket")?;
    socket
        .connect("8.8.8.8:80")
        .context("Could not establish a test connection")?;

    let local_addr = socket
        .local_addr()
        .context("Could not get the local address")?;

    Ok(local_addr.ip().to_string())
}
