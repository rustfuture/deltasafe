use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "deltasafe")]
#[command(about = "LAN file synchronization tool with AES encryption", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Synchronize a file or directory
    Sync {
        /// Source directory
        #[arg(short, long)]
        source: String,

        /// Target IP address (optional; use --auto for automatic discovery)
        #[arg(short, long, conflicts_with = "auto")]
        target: Option<String>,

        /// Automatic server discovery
        #[arg(long, conflicts_with = "target")]
        auto: bool,

        /// Automatically select when multiple servers are found (without user interaction)
        #[arg(long, requires = "auto")]
        auto_select: bool,

        /// AES key (32 bytes in hex)
        #[arg(short, long, conflicts_with = "password")]
        key: Option<String>,

        /// Password (automatically converted to an AES key)
        #[arg(short, long, conflicts_with = "key")]
        password: Option<String>,
    },

    /// Discover Deltasafe servers on the LAN
    Discover {
        /// Discovery timeout in seconds
        #[arg(short, long, default_value = "5")]
        timeout: u64,
    },

    /// Connect to a peer device
    Connect {
        /// IP address
        #[arg(short, long)]
        ip: String,
    },

    /// Watch the specified folder and sync changes
    Watch {
        /// Folder path
        #[arg(short, long)]
        folder: String,
    },

    /// Start the TCP server
    Server {
        /// Server address (IP and port) - optional, defaults to automatic
        #[arg(short, long)]
        address: Option<String>,

        /// AES key (32 bytes in hex)
        #[arg(short, long, conflicts_with = "password")]
        key: Option<String>,

        /// Password (automatically converted to an AES key)
        #[arg(short, long, conflicts_with = "key")]
        password: Option<String>,
    },
}
