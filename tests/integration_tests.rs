use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;
use deltasafe::server::start_server;
use deltasafe::sync::start_sync;
use deltasafe::crypto::parse_hex_key;
use deltasafe::utils::{resolve_key, KeyRole};

#[test]
fn test_basic_sync() {
    let test_dir = "test_data";
    let source_dir = format!("{}/source", test_dir);
    let received_dir = "received_files";

    let _ = fs::remove_dir_all(test_dir);
    let _ = fs::remove_dir_all(received_dir);

    fs::create_dir_all(&source_dir).unwrap();
    fs::write(format!("{}/test.txt", source_dir), "Hello, Deltasafe!").unwrap();
    fs::create_dir_all(format!("{}/nested", source_dir)).unwrap();
    fs::write(format!("{}/nested/other.txt", source_dir), "Second file").unwrap();

    let test_key = parse_hex_key(
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    ).unwrap();

    let server_handle = thread::spawn(move || {
        start_server("127.0.0.1:12346", &test_key, None);
    });

    thread::sleep(Duration::from_millis(500));

    start_sync(&source_dir, "127.0.0.1:12346", &test_key, None);

    thread::sleep(Duration::from_millis(500));

    assert!(Path::new("received_files/test.txt").exists());
    assert!(Path::new("received_files/nested/other.txt").exists());

    let received_content = fs::read_to_string("received_files/test.txt").unwrap();
    assert_eq!(received_content, "Hello, Deltasafe!");

    let received_nested = fs::read_to_string("received_files/nested/other.txt").unwrap();
    assert_eq!(received_nested, "Second file");

    drop(server_handle);

    let _ = fs::remove_dir_all(test_dir);
    let _ = fs::remove_dir_all(received_dir);
}

#[test]
fn test_password_sync_with_session_salt() {
    let test_dir = "test_data_password";
    let source_dir = format!("{}/source", test_dir);
    let received_dir = "received_files";

    let _ = fs::remove_dir_all(test_dir);
    let _ = fs::remove_dir_all(received_dir);

    fs::create_dir_all(&source_dir).unwrap();
    fs::write(format!("{}/secret.txt", source_dir), "Password protected").unwrap();

    let password = "testpassword123".to_string();
    let server_password = password.clone();

    let server_handle = thread::spawn(move || {
        let resolved = resolve_key(None, Some(&server_password), KeyRole::Server).unwrap();
        start_server("127.0.0.1:12347", &resolved.key, Some(server_password));
    });

    thread::sleep(Duration::from_millis(500));

    let resolved = resolve_key(None, Some(&password), KeyRole::Client).unwrap();
    start_sync(
        &source_dir,
        "127.0.0.1:12347",
        &resolved.key,
        resolved.pbkdf2_salt,
    );

    thread::sleep(Duration::from_millis(500));

    assert!(Path::new("received_files/secret.txt").exists());
    let content = fs::read_to_string("received_files/secret.txt").unwrap();
    assert_eq!(content, "Password protected");

    drop(server_handle);

    let _ = fs::remove_dir_all(test_dir);
    let _ = fs::remove_dir_all(received_dir);
}

#[test]
fn test_file_operations() {
    use deltasafe::sync::{calculate_file_hash, CHUNK_SIZE};

    let test_file = "tmp_rovodev_integration_test.txt";
    fs::write(test_file, "Integration test content").unwrap();

    let hash = calculate_file_hash(std::path::Path::new(test_file)).unwrap();
    assert_eq!(hash.len(), 64);
    assert_eq!(CHUNK_SIZE, 4096);

    fs::remove_file(test_file).unwrap();
}
