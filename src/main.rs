mod cli;
mod sync;
mod server;
mod crypto;
mod discovery;
mod utils; // Yeni eklenen modül

use cli::{Cli, Commands};
use clap::Parser;
use sync::start_sync;
use server::start_server;
use anyhow::Result; // crypto ve discovery use'ları utils'e taşındı

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
            let target_address = utils::resolve_target_address(target.as_deref(), *auto, *auto_select).await?;
            println!("Sync başlatılıyor: {} -> {}", source, target_address);
            
            let key_bytes = utils::resolve_key(key.as_deref(), password.as_deref())?;
            start_sync(source, &target_address, &key_bytes);
        },
        Commands::Discover { timeout } => {
            let servers = discovery::discover_servers(*timeout).await?;
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
            let server_address = utils::resolve_server_address(address.as_deref())?;
            println!("Sunucu başlatılıyor: {}", server_address);
            
            let key_bytes = utils::resolve_key(key.as_deref(), password.as_deref())?;
            start_server(&server_address, &key_bytes);
        },
    }
    Ok(())
}
