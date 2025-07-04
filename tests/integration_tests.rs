use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[test]
fn test_basic_sync() {
    // Test klasörlerini oluştur
    let test_dir = "test_data";
    let source_dir = format!("{}/source", test_dir);
    let received_dir = "received_files";
    
    // Temizlik
    let _ = fs::remove_dir_all(test_dir);
    let _ = fs::remove_dir_all(received_dir);
    
    // Test dosyası oluştur
    fs::create_dir_all(&source_dir).unwrap();
    fs::write(format!("{}/test.txt", source_dir), "Hello, Deltasafe!").unwrap();
    
    // Test anahtarı (32 bayt = 64 hex karakter)
    let test_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    
    // Sunucuyu başlat (arka planda)
    let server_handle = thread::spawn(move || {
        Command::new("cargo")
            .args(&["run", "--", "server", "--address", "127.0.0.1:12346", "--key", test_key])
            .output()
            .expect("Sunucu başlatılamadı");
    });
    
    // Sunucunun başlamasını bekle
    thread::sleep(Duration::from_secs(2));
    
    // Sync işlemini başlat
    let output = Command::new("cargo")
        .args(&["run", "--", "sync", "--source", &source_dir, "--target", "127.0.0.1:12346", "--key", test_key])
        .output()
        .expect("Sync komutu çalıştırılamadı");
    
    println!("Sync output: {}", String::from_utf8_lossy(&output.stdout));
    println!("Sync error: {}", String::from_utf8_lossy(&output.stderr));
    
    // Dosyanın alındığını kontrol et
    thread::sleep(Duration::from_secs(1));
    assert!(Path::new("received_files/test.txt").exists());
    
    let received_content = fs::read_to_string("received_files/test.txt").unwrap();
    assert_eq!(received_content, "Hello, Deltasafe!");
    
    // Temizlik
    let _ = fs::remove_dir_all(test_dir);
    let _ = fs::remove_dir_all(received_dir);
}

#[test]
fn test_file_operations() {
    use deltasafe::sync::{calculate_file_hash, CHUNK_SIZE};
    use std::fs;
    
    // Test dosyası oluştur
    let test_file = "tmp_rovodev_integration_test.txt";
    fs::write(test_file, "Integration test content").unwrap();
    
    // Hash hesapla
    let hash = calculate_file_hash(std::path::Path::new(test_file)).unwrap();
    assert_eq!(hash.len(), 64);
    
    // Chunk size kontrolü
    assert_eq!(CHUNK_SIZE, 4096);
    
    // Temizlik
    fs::remove_file(test_file).unwrap();
}