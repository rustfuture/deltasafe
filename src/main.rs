use anyhow::Result;
use clap::Parser;
use deltasafe::cli::{Cli, Commands};
use deltasafe::server::start_server;
use deltasafe::sync::start_sync;
use deltasafe::utils::{self, KeyRole};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run_command(&cli.command).await {
        eprintln!("[❌] Error: {}", e);
        std::process::exit(1);
    }
}

async fn run_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Sync {
            source,
            target,
            auto,
            auto_select,
            key,
            password,
        } => {
            let target_address =
                utils::resolve_target_address(target.as_deref(), *auto, *auto_select).await?;
            println!("Starting sync: {} -> {}", source, target_address);

            let resolved =
                utils::resolve_key(key.as_deref(), password.as_deref(), KeyRole::Client)?;
            start_sync(source, &target_address, &resolved.key, resolved.pbkdf2_salt)?;
        }
        Commands::Discover { timeout } => {
            let servers = deltasafe::discovery::discover_servers(*timeout).await?;
            if servers.is_empty() {
                println!("[ℹ️] No servers found.");
            } else {
                println!("[✅] Discovered servers:");
                for (i, server) in servers.iter().enumerate() {
                    println!(
                        "  {}. {} ({:?})",
                        i + 1,
                        server.address,
                        server.discovery_method
                    );
                    if let Some(name) = &server.name {
                        println!("     Service name: {}", name);
                    }
                }
            }
        }
        Commands::Connect { ip } => {
            println!("Connecting to peer device: {}", ip);
            println!("⚠️ This feature is still under development.");
        }
        Commands::Watch { folder } => {
            println!("Watching folder: {}", folder);
            println!("⚠️ This feature is still under development.");
        }
        Commands::Server {
            address,
            key,
            password,
        } => {
            let server_address = utils::resolve_server_address(address.as_deref())?;
            println!("Starting server: {}", server_address);

            let resolved =
                utils::resolve_key(key.as_deref(), password.as_deref(), KeyRole::Server)?;
            start_server(&server_address, &resolved.key, password.clone())?;
        }
    }
    Ok(())
}
