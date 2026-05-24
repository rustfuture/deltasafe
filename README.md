# Deltasafe: Güvenli LAN Dosya Senkronizasyon Aracı 🚀

## Genel Bakış

**Deltasafe**, yerel ağ (LAN) üzerinde dosyaları güvenli ve **kullanıcı dostu** bir şekilde senkronize etmek için tasarlanmış, Rust ile geliştirilmiş modern bir komut satırı aracıdır. 

🎯 **Artık karmaşık hex anahtarlar yok!** Basit şifreler kullanın: `--password "MyPassword123"`  
🔍 **Otomatik sunucu keşfi!** Manuel IP girmeye gerek yok: `--auto`  
🤖 **Akıllı varsayılanlar!** Minimal parametre ile çalışır: `deltasafe server`

AES-256 şifrelemesi ile verilerinizin gizliliğini ve bütünlüğünü sağlarken, kullanım kolaylığından ödün vermez.

> ⚠️ **Minimum Rust Versiyonu:** Bu proje `edition = "2024"` kullandığı için **Rust 1.85 veya üzeri** gerektirir.

## ✨ Özellikler

### 🔒 Güvenlik
*   **AES-256-CBC Şifreleme:** Endüstri standardı şifreleme ile maksimum güvenlik
*   **PBKDF2 Anahtar Türetme:** Basit şifrelerden güvenli anahtarlar üretir (100.000 iterasyon)
*   **BLAKE3 Hash Doğrulaması:** Dosya bütünlüğü garantisi
*   **Rastgele IV:** Her chunk için benzersiz initialization vector

### 🚀 Kullanıcı Dostu
*   **Basit Şifre Sistemi:** Karmaşık hex anahtarlar yerine "MyPassword123" 
*   **Otomatik Sunucu Keşfi:** LAN'da sunucuları otomatik bulur (port tarama: 12340-12350)
*   **Akıllı Varsayılanlar:** Minimal parametre ile çalışır (varsayılan port: 12345)
*   **Progress Tracking:** Gerçek zamanlı transfer ilerlemesi

### ⚡ Performans
*   **Chunk-based Transfer:** 4KB parçalar ile optimal aktarım
*   **Paralel Bağlantı:** Sunucu birden fazla istemciyi destekler
*   **Async/Await:** Modern Rust async programlama (Tokio runtime)
*   **Dizin Yapısı Korunur:** Klasör hiyerarşisi aynen aktarılır

## 🛠️ Kurulum

Deltasafe'i kullanabilmek için sisteminizde [Rust](https://www.rust-lang.org/tools/install) **1.85 veya üzeri** kurulu olması gerekmektedir.

1.  **Rust Kurulumu:**
    Eğer Rust kurulu değilse, aşağıdaki komut ile `rustup`'ı kurabilirsiniz:
    ```bash
    curl --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    Kurulumdan sonra, Rust araç zincirini PATH'inize eklemek için terminalinizi yeniden başlatmanız veya aşağıdaki komutu çalıştırmanız gerekebilir:
    ```bash
    source $HOME/.cargo/env
    ```
    
    Mevcut Rust sürümünüzü güncellemek için:
    ```bash
    rustup update stable
    ```

2.  **Projeyi Klonlama:**
    ```bash
    git clone https://github.com/rustfuture/deltasafe.git
    cd deltasafe
    ```

3.  **Bağımlılıkları Yükleme ve Derleme:**
    ```bash
    cargo build --release
    ```
    Bu komut, projenin bağımlılıklarını indirir ve optimize edilmiş bir çalıştırılabilir dosya oluşturur. Çalıştırılabilir dosya `target/release/deltasafe` konumunda bulunacaktır.

## 🚀 Kullanım

Deltasafe artık **kullanıcı dostu** hale geldi! Karmaşık hex anahtarlar yerine basit şifreler kullanabilir, sunucuları otomatik keşfedebilirsiniz.

### 🔑 Şifreleme Seçenekleri

**Seçenek 1: Basit Şifre (Önerilen)**
```bash
--password "MySecretPassword123"
```

**Seçenek 2: Manuel Hex Anahtar (İleri Seviye)**
```bash
--key 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
```

**Seçenek 3: Otomatik Geçici Anahtar**
```bash
# Hiç parametre vermezseniz otomatik anahtar üretilir
deltasafe server  # Geçici anahtar gösterilir
```

### 🔍 Sunucu Keşfi (Yeni!)

LAN'daki mevcut Deltasafe sunucularını otomatik olarak keşfedin:

```bash
./target/release/deltasafe discover
```

### 🖥️ Sunucu Modu

**Basit Kullanım (Önerilen):**
```bash
./target/release/deltasafe server --password "MyPassword123"
```

**Gelişmiş Kullanım:**
```bash
./target/release/deltasafe server --address 0.0.0.0:12345 --password "MyPassword123"
```

**Otomatik Mod:**
```bash
./target/release/deltasafe server
# Otomatik IP, port ve geçici anahtar üretir
```

### 📤 İstemci Modu (Sync)

**Otomatik Sunucu Keşfi (Önerilen):**
```bash
# Kullanıcı seçimi ile (birden fazla sunucu varsa)
./target/release/deltasafe sync --source ./my_folder --auto --password "MyPassword123"

# Otomatik seçim (kullanıcı etkileşimi olmadan)
./target/release/deltasafe sync --source ./my_folder --auto --auto-select --password "MyPassword123"
```

**Manuel Hedef Belirleme:**
```bash
./target/release/deltasafe sync --source ./my_folder --target 192.168.1.100:12345 --password "MyPassword123"
```

**Hex Anahtar ile (İleri Seviye):**
```bash
./target/release/deltasafe sync --source ./my_folder --target 192.168.1.100:12345 --key 0123456789abcdef...
```

### 📋 Parametre Açıklamaları

*   `--source`: Senkronize edilecek kaynak klasör
*   `--target`: Hedef sunucu IP:port (opsiyonel, --auto ile otomatik)
*   `--auto`: Otomatik sunucu keşfi
*   `--auto-select`: Birden fazla sunucu varsa otomatik seç (etkileşim olmadan)
*   `--password`: Basit şifre (önerilen)
*   `--key`: 64 karakterlik hex anahtar (ileri seviye)
*   `--address`: Sunucu adresi (opsiyonel, otomatik tespit)

## 🧪 Test Etme

Projeyi test etmek için:

```bash
# Unit testleri çalıştır
cargo test

# Belirli bir test çalıştır
cargo test test_file_hash_calculation

# Test çıktısını detaylı göster
cargo test -- --nocapture

# Lint kontrolü (clippy)
cargo clippy

# Kod formatlama kontrolü
cargo fmt --check
```

## 📊 Teknik Özellikler

- **AES-256-CBC şifreleme** ile maksimum güvenlik
- **PBKDF2 anahtar türetme** ile basit şifre desteği  
- **BLAKE3 hash doğrulaması** ile dosya bütünlüğü
- **4KB chunk transfer** ile optimal performans
- **Async/await** ile modern Rust mimarisi

## 🔄 Transfer Protokolü

İstemci-sunucu arası iletişim şu adımlarla gerçekleşir:

```
┌──────────┐                          ┌──────────┐
│  Client  │                          │  Server  │
└────┬─────┘                          └────┬─────┘
     │                                     │
     │  [4 byte] Header uzunluğu (BE)      │
     │────────────────────────────────────▶│
     │                                     │
     │  [N byte] JSON FileHeader           │
     │────────────────────────────────────▶│
     │                                     │
     │  [1 byte] ACK (0x01 = başarılı)     │
     │◀────────────────────────────────────│
     │                                     │
     │  [16 byte IV + şifreli veri] × N    │
     │────────────────────────────────────▶│
     │                                     │
```

**FileHeader JSON yapısı:**
```json
{
  "file_name": "dosya.txt",
  "file_size": 1024,
  "file_hash": "blake3_hex_hash",
  "relative_path": "alt_klasor/dosya.txt"
}
```

## 🏗️ Proje Yapısı

```
src/
├── main.rs        # Giriş noktası, komut yönlendirme
├── lib.rs         # Kütüphane arayüzü, modül dışa aktarımları
├── cli.rs         # Komut satırı argümanları (Clap derive)
├── crypto.rs      # PBKDF2 anahtar türetme, hex parse, validasyon
├── sync.rs        # Dosya tarama, AES şifreleme, chunk gönderimi
├── server.rs      # TCP dinleme, chunk alma, AES şifre çözme
├── discovery.rs   # mDNS + port tarama ile sunucu keşfi
└── utils.rs       # Anahtar/adres çözümleme yardımcıları
```

## 📦 Bağımlılıklar

| Crate | Versiyon | Açıklama |
|-------|----------|----------|
| `clap` | 4.4 | Komut satırı argüman ayrıştırma (derive mode) |
| `tokio` | 1.0 | Asenkron runtime (full features) |
| `aes` | 0.8 | AES şifreleme algoritması |
| `cbc` | 0.1 | CBC (Cipher Block Chaining) modu |
| `cipher` | 0.4 | Block cipher trait'leri ve padding |
| `pbkdf2` | 0.12 | Şifreden anahtar türetme |
| `sha2` | 0.10 | SHA-256 (PBKDF2 ile kullanılır) |
| `blake3` | 1.5 | Hızlı dosya hash hesaplama |
| `serde` / `serde_json` | 1.0 | JSON serialization/deserialization |
| `walkdir` | 2.5 | Dizin ağacı tarama |
| `indicatif` | 0.17 | Terminal progress bar |
| `mdns-sd` | 0.11 | mDNS sunucu keşfi |
| `rand` | 0.8 | Rastgele IV ve anahtar üretimi |
| `hex` | 0.4 | Hex encoding/decoding |
| `anyhow` | 1.0 | Esnek hata yönetimi |

## 🎯 Kullanım Senaryoları

### 👥 **Yeni Başlayan Kullanıcı**
```bash
# Terminal 1: Sunucu başlat
deltasafe server --password "basit123"

# Terminal 2: Dosya gönder (kullanıcı seçimi ile)
deltasafe sync --source ./documents --auto --password "basit123"

# Veya otomatik seçim (etkileşim olmadan)
deltasafe sync --source ./documents --auto --auto-select --password "basit123"
```

### 🔧 **İleri Seviye Kullanıcı**
```bash
# Önce keşif yap
deltasafe discover --timeout 10

# Manuel hedef ile gönder
deltasafe sync --source ./folder --target 192.168.1.50:12345 --key 0123...cdef
```

### 🏢 **Kurumsal Kullanım**
```bash
# Sabit sunucu adresi
deltasafe server --address 0.0.0.0:12345 --password "CompanySecret2024"

# Toplu dosya transferi
deltasafe sync --source ./shared_files --target server.company.local:12345 --password "CompanySecret2024"
```


## 🤝 Katkıda Bulunma

Projenin geliştirilmesine katkıda bulunmak isterseniz:

1.  Projeyi fork edin
2.  Feature branch oluşturun (`git checkout -b feature/yeni-ozellik`)
3.  Değişikliklerinizi commit edin (`git commit -m 'feat: Yeni özellik ekle'`)
4.  Branch'e push edin (`git push origin feature/yeni-ozellik`)
5.  Pull Request açın

**Geliştirici kontrol listesi:**
```bash
cargo build          # Derleme hatası olmadığını doğrula
cargo test           # Tüm testlerin geçtiğinden emin ol
cargo clippy         # Lint uyarılarını kontrol et
cargo fmt --check    # Kod formatını kontrol et
```

## 📄 Lisans

Bu proje MIT Lisansı altında lisanslanmıştır. Daha fazla bilgi için `LICENSE` dosyasına bakınız.