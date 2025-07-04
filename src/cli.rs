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

        /// Hedef IP adresi (opsiyonel, --auto ile otomatik keşif)
        #[arg(short, long, conflicts_with = "auto")]
        target: Option<String>,

        /// Otomatik sunucu keşfi
        #[arg(long, conflicts_with = "target")]
        auto: bool,

        /// Birden fazla sunucu bulunursa otomatik seç (kullanıcı etkileşimi olmadan)
        #[arg(long, requires = "auto")]
        auto_select: bool,

        /// AES anahtarı (hex formatında 32 bayt)
        #[arg(short, long, conflicts_with = "password")]
        key: Option<String>,

        /// Şifre (otomatik olarak AES anahtarına dönüştürülür)
        #[arg(short, long, conflicts_with = "key")]
        password: Option<String>,
    },

    /// LAN'daki Deltasafe sunucularını keşfet
    Discover {
        /// Keşif timeout süresi (saniye)
        #[arg(short, long, default_value = "5")]
        timeout: u64,
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
        /// Sunucu adresi (IP ve port) - opsiyonel, varsayılan: otomatik
        #[arg(short, long)]
        address: Option<String>,

        /// AES anahtarı (hex formatında 32 bayt)
        #[arg(short, long, conflicts_with = "password")]
        key: Option<String>,

        /// Şifre (otomatik olarak AES anahtarına dönüştürülür)
        #[arg(short, long, conflicts_with = "key")]
        password: Option<String>,
    },
}
