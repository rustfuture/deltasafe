use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "deltasafe")]
#[command(about = "AES şifrelemeli LAN dosya senkronizasyon aracı", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Dosya ya da klasörü senkronize et
    Sync {
        /// Kaynak dizin
        #[arg(short, long)]
        source: String,

        /// Hedef IP adresi
        #[arg(short, long)]
        target: String,

        /// AES anahtarı (hex formatında 32 bayt)
        #[arg(short, long)]
        key: String,
    },

    /// Peer cihazla bağlantı kur
    Connect {
        /// IP adresi
        #[arg(short, long)]
        ip: String,
    },

    /// Belirtilen klasörü izleyerek değişiklikleri sync et
    Watch {
        /// Klasör yolu
        #[arg(short, long)]
        folder: String,
    },

    /// TCP sunucusunu başlat
    Server {
        /// Sunucu adresi (IP ve port)
        #[arg(short, long)]
        address: String,

        /// AES anahtarı (hex formatında 32 bayt)
        #[arg(short, long)]
        key: String,
    },
}
