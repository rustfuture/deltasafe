use std::fs;
use std::net::TcpListener;
use std::path::PathBuf;
use std::thread::{self, JoinHandle};

use anyhow::Result;
use deltasafe::crypto::parse_hex_key;
use deltasafe::server::serve_once;
use deltasafe::sync::{calculate_file_hash, start_sync, CHUNK_SIZE};
use deltasafe::utils::{resolve_key, KeyRole};
use tempfile::TempDir;

const TEST_KEY: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

struct Receiver {
    _sandbox: TempDir,
    root: PathBuf,
    address: String,
    handle: JoinHandle<Result<()>>,
}

fn spawn_receiver(key: [u8; 32], password: Option<String>) -> Receiver {
    let sandbox = tempfile::tempdir().unwrap();
    let root = sandbox.path().join("received");
    let thread_root = root.clone();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap().to_string();
    let handle =
        thread::spawn(move || serve_once(listener, &key, password.as_deref(), &thread_root));
    Receiver {
        _sandbox: sandbox,
        root,
        address,
        handle,
    }
}

fn source_tree() -> (TempDir, PathBuf, Vec<u8>) {
    let sandbox = tempfile::tempdir().unwrap();
    let source = sandbox.path().join("source");
    fs::create_dir_all(source.join("nested")).unwrap();
    fs::write(source.join("hello.txt"), b"Hello, Deltasafe!").unwrap();
    fs::write(source.join("empty.bin"), []).unwrap();
    let large = vec![0x5a; CHUNK_SIZE * 3 + 17];
    fs::write(source.join("nested/large.bin"), &large).unwrap();
    (sandbox, source, large)
}

#[test]
fn transfers_multiple_nested_empty_and_multichunk_files() {
    let key = parse_hex_key(TEST_KEY).unwrap();
    let receiver = spawn_receiver(key, None);
    let (_source_sandbox, source, large) = source_tree();

    start_sync(source.to_str().unwrap(), &receiver.address, &key, None).unwrap();
    receiver.handle.join().unwrap().unwrap();

    assert_eq!(
        fs::read(receiver.root.join("hello.txt")).unwrap(),
        b"Hello, Deltasafe!"
    );
    assert_eq!(fs::read(receiver.root.join("empty.bin")).unwrap(), b"");
    assert_eq!(
        fs::read(receiver.root.join("nested/large.bin")).unwrap(),
        large
    );
}

#[test]
fn password_mode_uses_the_transmitted_salt_and_waits_for_final_status() {
    let password = "correct-password-123";
    let server_key = resolve_key(None, Some(password), KeyRole::Server)
        .unwrap()
        .key;
    let receiver = spawn_receiver(server_key, Some(password.to_owned()));
    let (_source_sandbox, source, _) = source_tree();
    let client = resolve_key(None, Some(password), KeyRole::Client).unwrap();

    start_sync(
        source.to_str().unwrap(),
        &receiver.address,
        &client.key,
        client.pbkdf2_salt,
    )
    .unwrap();
    receiver.handle.join().unwrap().unwrap();
    assert_eq!(
        fs::read(receiver.root.join("hello.txt")).unwrap(),
        b"Hello, Deltasafe!"
    );
}

#[test]
fn wrong_password_is_reported_and_publishes_no_file() {
    let server_password = "server-password-123";
    let server_key = resolve_key(None, Some(server_password), KeyRole::Server)
        .unwrap()
        .key;
    let receiver = spawn_receiver(server_key, Some(server_password.to_owned()));
    let (_source_sandbox, source, _) = source_tree();
    let client = resolve_key(None, Some("wrong-password-456"), KeyRole::Client).unwrap();

    let result = start_sync(
        source.to_str().unwrap(),
        &receiver.address,
        &client.key,
        client.pbkdf2_salt,
    );
    assert!(result.is_err());
    assert!(receiver.handle.join().unwrap().is_err());
    assert!(!receiver.root.join("hello.txt").exists());
}

#[test]
fn existing_destination_is_preserved() {
    let key = parse_hex_key(TEST_KEY).unwrap();
    let sandbox = tempfile::tempdir().unwrap();
    let root = sandbox.path().join("received");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("hello.txt"), b"original").unwrap();
    let thread_root = root.clone();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap().to_string();
    let handle = thread::spawn(move || serve_once(listener, &key, None, &thread_root));
    let (_source_sandbox, source, _) = source_tree();

    assert!(start_sync(source.to_str().unwrap(), &address, &key, None).is_err());
    assert!(handle.join().unwrap().is_err());
    assert_eq!(fs::read(root.join("hello.txt")).unwrap(), b"original");
}

#[test]
fn hashes_files_without_loading_them_as_text() {
    let sandbox = tempfile::tempdir().unwrap();
    let path = sandbox.path().join("binary.dat");
    fs::write(&path, [0, 159, 146, 150, 255]).unwrap();
    let hash = calculate_file_hash(&path).unwrap();
    assert_eq!(hash.len(), 64);
}
