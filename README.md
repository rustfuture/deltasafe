# Deltasafe: Güvenli LAN Dosya Senkronizasyon Aracı 🚀

## Genel Bakış

**Deltasafe**, yerel ağ (LAN) üzerinde dosyaları güvenli ve **kullanıcı dostu** bir şekilde senkronize etmek için tasarlanmış, Rust ile geliştirilmiş modern bir komut satırı aracıdır. 

🎯 **Artık karmaşık hex anahtarlar yok!** Basit şifreler kullanın: `--password "MyPassword123"`  
🔍 **Otomatik sunucu keşfi!** Manuel IP girmeye gerek yok: `--auto`  
🤖 **Akıllı varsayılanlar!** Minimal parametre ile çalışır: `deltasafe server`

AES-256 şifrelemesi ile verilerinizin gizliliğini ve bütünlüğünü sağlarken, kullanım kolaylığından ödün vermez.

## ✨ Özellikler

### 🔒 Güvenlik
*   **AES-256-CBC Şifreleme:** Endüstri standardı şifreleme ile maksimum güvenlik
*   **PBKDF2 Anahtar Türetme:** Basit şifrelerden güvenli anahtarlar üretir
*   **BLAKE3 Hash Doğrulaması:** Dosya bütünlüğü garantisi
*   **Rastgele IV:** Her chunk için benzersiz initialization vector

### 🚀 Kullanıcı Dostu
*   **Basit Şifre Sistemi:** Karmaşık hex anahtarlar yerine "MyPassword123" 
*   **Otomatik Sunucu Keşfi:** LAN'da sunucuları otomatik bulur
*   **Akıllı Varsayılanlar:** Minimal parametre ile çalışır
*   **Progress Tracking:** Gerçek zamanlı transfer ilerlemesi

### ⚡ Performans
*   **Chunk-based Transfer:** 4KB parçalar ile optimal aktarım
*   **Paralel Bağlantı:** Sunucu birden fazla istemciyi destekler
*   **Async/Await:** Modern Rust async programlama
*   **Dizin Yapısı Korunur:** Klasör hiyerarşisi aynen aktarılır

## 🛠️ Kurulum

Deltasafe'i kullanabilmek için sisteminizde [Rust](https://www.rust-lang.org/tools/install) kurulu olması gerekmektedir.

1.  **Rust Kurulumu:**
    Eğer Rust kurulu değilse, aşağıdaki komut ile `rustup`'ı kurabilirsiniz:
    ```bash
    curl --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    Kurulumdan sonra, Rust araç zincirini PATH'inize eklemek için terminalinizi yeniden başlatmanız veya aşağıdaki komutu çalıştırmanız gerekebilir:
    ```bash
    source $HOME/.cargo/env
    ```

2.  **Projeyi Klonlama:**
    ```bash
    git clone https://github.com/your-username/deltasafe.git # Kendi repo adresinizi buraya ekleyin
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

### 🔍 Sunucu Keşfi

LAN'daki mevcut Deltasafe sunucularını otomatik olarak keşfedin:

```bash
./target/release/deltasafe discover
```

> **Not:** mDNS (Zeroconf) keşfi henüz tamamlanmamıştır. `--auto` modu şu an yerel ağda **port taraması** (12340–12349) kullanır. Güvenilir keşif için `--target IP:port` ile manuel hedef belirtmeniz önerilir.

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
```

## 📊 Teknik Özellikler

- **AES-256-CBC şifreleme** ile maksimum güvenlik
- **PBKDF2 anahtar türetme** ile basit şifre desteği  
- **BLAKE3 hash doğrulaması** ile dosya bütünlüğü
- **4KB chunk transfer** ile optimal performans
- **Async/await** ile modern Rust mimarisi

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

Projenin geliştirilmesine katkıda bulunmak isterseniz, lütfen bir pull request açmaktan çekinmeyin. Her türlü katkı memnuniyetle karşılanır!

## 📄 Lisans

Bu proje MIT Lisansı altında lisanslanmıştır. Daha fazla bilgi için `LICENSE` dosyasına bakınız.