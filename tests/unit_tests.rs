use deltasafe::sync::{calculate_file_hash, FileHeader, CHUNK_SIZE};
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_hash_calculation() {
        // Create a test file
        let test_file = "tmp_rovodev_test_hash.txt";
        fs::write(test_file, "Test content for hash").unwrap();

        // Compute the hash
        let hash = calculate_file_hash(Path::new(test_file)).unwrap();

        // Check that the hash has the correct length
        assert_eq!(hash.len(), 64); // A BLAKE3 hex string is 64 characters

        // Check that the same content produces the same hash
        let hash2 = calculate_file_hash(Path::new(test_file)).unwrap();
        assert_eq!(hash, hash2);

        // Cleanup
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_hex_key_validation() {
        // Valid key
        let valid_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        assert_eq!(valid_key.len(), 64);

        // Hex decode test
        let decoded = hex::decode(valid_key).unwrap();
        assert_eq!(decoded.len(), 32);

        // Invalid key (too short)
        let invalid_key = "0123456789abcdef";
        assert_eq!(invalid_key.len(), 16); // Too short

        // Invalid key (not hex)
        let non_hex_key = "gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg";
        assert!(hex::decode(non_hex_key).is_err());
    }

    #[test]
    fn test_file_header_serialization() {
        use serde_json;
        use std::path::PathBuf;

        // Create and test the FileHeader struct
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

        // Deserialize test
        let deserialized: FileHeader = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.file_name, "test.txt");
        assert_eq!(deserialized.file_size, 1024);
        assert_eq!(deserialized.file_hash, "abcd1234");
    }

    #[test]
    fn test_chunk_size_constant() {
        // Check that the CHUNK_SIZE constant has a reasonable value
        const EXPECTED_CHUNK_SIZE: usize = 4096;
        assert_eq!(CHUNK_SIZE, EXPECTED_CHUNK_SIZE);
    }

    #[test]
    fn test_different_file_contents_different_hashes() {
        // Create two different files
        let test_file1 = "tmp_rovodev_test1.txt";
        let test_file2 = "tmp_rovodev_test2.txt";

        fs::write(test_file1, "Content 1").unwrap();
        fs::write(test_file2, "Content 2").unwrap();

        let hash1 = calculate_file_hash(Path::new(test_file1)).unwrap();
        let hash2 = calculate_file_hash(Path::new(test_file2)).unwrap();

        // Different contents must produce different hashes
        assert_ne!(hash1, hash2);

        // Cleanup
        fs::remove_file(test_file1).unwrap();
        fs::remove_file(test_file2).unwrap();
    }
}
