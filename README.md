# Deltasafe: Güvenli LAN Dosya Senkronizasyon Aracı 🚀

## Genel Bakış

**Deltasafe**, yerel ağ (LAN) üzerinde dosyaları güvenli ve verimli bir şekilde senkronize etmek için tasarlanmış, Rust ile geliştirilmiş bir komut satırı aracıdır. Özellikle hassas verilerin güvenli bir şekilde aktarılması gerektiği durumlarda, AES-256 şifrelemesi kullanarak verilerinizin gizliliğini ve bütünlüğünü sağlar.

## ✨ Özellikler

*   **AES-256 Şifreleme:** Tüm dosya parçaları, endüstri standardı AES-256 algoritması ile şifrelenerek aktarılır.
*   **Parça Tabanlı Senkronizasyon:** Büyük dosyalar küçük parçalara bölünerek daha verimli ve hataya dayanıklı bir aktarım sağlanır.
*   **BLAKE3 Hash Doğrulaması:** Her dosya parçasının bütünlüğü BLAKE3 hash algoritması ile doğrulanır.
*   **TCP/IP Üzerinden Güvenli Aktarım:** Veriler, güvenilir TCP/IP protokolü üzerinden aktarılır.
*   **Basit CLI Arayüzü:** Kolay kullanımlı komut satırı arayüzü ile sunucu ve istemci işlemleri yönetilir.

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

Deltasafe, sunucu ve istemci modları ile çalışır. Her iki tarafın da aynı AES anahtarını kullanması gerekmektedir. Anahtar, 32 bayt uzunluğunda (64 karakterlik hex string) olmalıdır.

### 🔑 AES Anahtarı Oluşturma

Güvenli bir AES anahtarı oluşturmak için aşağıdaki Python kodunu kullanabilirsiniz:

```python
import os
import binascii

key = os.urandom(32) # 32 bayt rastgele anahtar
hex_key = binascii.hexlify(key).decode('utf-8')
print(hex_key)
```
Bu kod size 64 karakterlik bir hex string verecektir. Bu anahtarı hem sunucu hem de istemci için kullanın.

### Sunucu Modu

Belirtilen IP ve port üzerinde gelen bağlantıları dinler ve şifrelenmiş dosyaları alır.

```bash
./target/release/deltasafe server --address 0.0.0.0:12345 --key <64-karakterli-hex-anahtarınız>
```
*   `--address`: Sunucunun dinleyeceği IP adresi ve port (örn: `0.0.0.0:12345`).
*   `--key`: Kullanılacak 32 baytlık AES anahtarının 64 karakterlik hex string temsili.

### İstemci Modu

Belirtilen kaynak klasördeki dosyaları şifreler ve hedef sunucuya gönderir.

```bash
./target/release/deltasafe sync --source /path/to/your/folder --target 192.168.1.100:12345 --key <64-karakterli-hex-anahtarınız>
```
*   `--source`: Senkronize edilecek kaynak klasörün yolu (örn: `./my_documents`).
*   `--target`: Hedef sunucunun IP adresi ve portu (örn: `192.168.1.100:12345`).
*   `--key`: Kullanılacak 32 baytlık AES anahtarının 64 karakterlik hex string temsili.

## 🚧 Bilinen Sınırlamalar ve Gelecek Geliştirmeler

*   **Sunucu Tarafında Dosya Adı Yönetimi:** Şu anda sunucu, gelen tüm dosyaları `received_file` adıyla kaydetmektedir. Bu, birden fazla dosya senkronize edildiğinde veya aynı dosya tekrar gönderildiğinde eski dosyaların üzerine yazılmasına neden olur. Gelecekte bu durumun iyileştirilmesi planlanmaktadır.
*   **Hata Yönetimi:** Uygulama içinde daha sağlam hata yönetimi mekanizmalarının eklenmesi gerekmektedir.
*   **`Connect` ve `Watch` Komutları:** `cli.rs`'de tanımlı olan `connect` ve `watch` komutları henüz işlevsel değildir.

## 🤝 Katkıda Bulunma

Projenin geliştirilmesine katkıda bulunmak isterseniz, lütfen bir pull request açmaktan çekinmeyin. Her türlü katkı memnuniyetle karşılanır!

## 📄 Lisans

Bu proje MIT Lisansı altında lisanslanmıştır. Daha fazla bilgi için `LICENSE` dosyasına bakınız.