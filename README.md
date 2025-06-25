# Deltasafe: AES Şifrelemeli LAN Senkronizasyon Aracı

## Proje Özeti

Deltasafe, LAN üzerinde dosya senkronizasyonu ve veri güvenliğini sağlamak için geliştirilmiş bir araçtır. AES-256 şifreleme kullanarak dosyaları parçalara böler, her parçayı şifreler ve TCP üzerinden güvenli bir şekilde aktarır.

## Kullanılan Teknolojiler

- **Rust:** Performans odaklı sistem programlama dili. Dosya işleme ve şifreleme gibi görevler için kullanılmıştır.
- **AES-256:** Simetrik şifreleme algoritması. Her dosya parçası ayrı ayrı şifrelenir.
- **TCP/IP:** Verilerin güvenli ve kararlı bir şekilde iletilmesi için kullanılır.

## Proje Akışı

### Sunucu
- TCP üzerinden bağlantı kabul eder.
- Alınan dosya parçalarını birleştirir ve hedef dosyayı oluşturur.

### İstemci
- Dosyaları parçalara böler, şifreler ve her parçayı TCP üzerinden gönderir.

## Kullanıcı Kılavuzu

### Sunucu Başlatma

```bash
cargo run -- server --address 0.0.0.0:12345
```
- Belirtilen IP ve port üzerinde gelen bağlantıları dinler.

### İstemci Başlatma

```bash
cargo run -- sync --source ./test-folder --target 0.0.0.0:12345
```
- Kaynak dosyalar belirtilen sunucuya gönderilir.
