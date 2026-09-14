use deltasafe::crypto::parse_hex_key;
use deltasafe::sync::{calculate_file_hash, FileHeader, CHUNK_SIZE};
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    /// Write `contents` to a uniquely named file inside `dir`.
    fn fixture(dir: &Path, name: &str, contents: &[u8]) -> std::path::PathBuf {
        let path = dir.join(name);
        fs::write(&path, contents).unwrap();
        path
    }

    #[test]
    fn test_file_hash_calculation() {
        let dir = tempfile::tempdir().unwrap();
        let test_file = fixture(dir.path(), "hash.txt", b"Test content for hash");

        let hash = calculate_file_hash(&test_file).unwrap();

        // A BLAKE3 hex string is 64 characters.
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));

        // The same content produces the same hash.
        let hash2 = calculate_file_hash(&test_file).unwrap();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_different_file_contents_different_hashes() {
        let dir = tempfile::tempdir().unwrap();
        let file1 = fixture(dir.path(), "one.txt", b"Content 1");
        let file2 = fixture(dir.path(), "two.txt", b"Content 2");

        assert_ne!(
            calculate_file_hash(&file1).unwrap(),
            calculate_file_hash(&file2).unwrap()
        );
    }

    #[test]
    fn test_hex_key_validation() {
        // Exercise the application's own validator rather than the `hex` crate directly.
        let valid = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        assert_eq!(parse_hex_key(valid).unwrap().len(), 32);

        // Too short.
        assert!(parse_hex_key("0123456789abcdef").is_err());
        // Correct length, but not hexadecimal.
        let non_hex = "g".repeat(64);
        assert!(parse_hex_key(&non_hex).is_err());
        // Empty.
        assert!(parse_hex_key("").is_err());
        // A generated key must round-trip through the validator.
        let generated = deltasafe::crypto::generate_random_hex_key();
        assert!(parse_hex_key(&generated).is_ok());
    }

    #[test]
    fn test_file_header_serialization() {
        use serde_json;
        use std::path::PathBuf;

        let header = FileHeader {
            protocol_version: 1,
            session_id: "00112233445566778899aabbccddeeff".to_string(),
            file_name: "test.txt".to_string(),
            file_size: 1024,
            file_hash: "abcd1234".to_string(),
            relative_path: PathBuf::from("subdir/test.txt"),
            pbkdf2_salt: None,
        };

        let serialized = serde_json::to_string(&header).unwrap();
        assert!(serialized.contains("test.txt"));
        assert!(serialized.contains("1024"));
        assert!(serialized.contains("abcd1234"));

        let deserialized: FileHeader = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.file_name, "test.txt");
        assert_eq!(deserialized.file_size, 1024);
        assert_eq!(deserialized.file_hash, "abcd1234");
    }

    #[test]
    fn test_chunk_size_constant() {
        const EXPECTED_CHUNK_SIZE: usize = 4096;
        assert_eq!(CHUNK_SIZE, EXPECTED_CHUNK_SIZE);
    }
}
