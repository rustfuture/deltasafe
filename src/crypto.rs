//! Cryptographic helper functions
//!
//! This module contains encryption key derivation and validation operations.

use anyhow::{Context, Result};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

/// PBKDF2 iteration count (sufficient for security)
const PBKDF2_ITERATIONS: u32 = 100_000;

/// Salt length (128 bits)
const SALT_LENGTH: usize = 16;

/// Default salt (should be random in production; fixed for now)
const DEFAULT_SALT: &[u8] = b"deltasafe_salt16";

/// Derives an AES-256 key from a password
///
/// # Arguments
/// * `password` - User password
/// * `salt` - Optional salt (the default is used when `None`)
///
/// # Returns
/// A 32-byte AES key
pub fn derive_key_from_password(password: &str, salt: Option<&[u8]>) -> Result<[u8; 32]> {
    if password.len() < 8 {
        anyhow::bail!("Password must be at least 8 characters long");
    }

    let salt = salt.unwrap_or(DEFAULT_SALT);
    if salt.len() != SALT_LENGTH {
        anyhow::bail!("Salt must be {} bytes long", SALT_LENGTH);
    }

    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key);

    Ok(key)
}

/// Converts a hex string into a 32-byte key
pub fn parse_hex_key(hex_key: &str) -> Result<[u8; 32]> {
    if hex_key.len() != 64 {
        anyhow::bail!("Hex key must be 64 characters long (32 bytes)");
    }

    let decoded = hex::decode(hex_key).context("Invalid hex format")?;

    let key: [u8; 32] = decoded
        .try_into()
        .map_err(|_| anyhow::anyhow!("Key must be 32 bytes long"))?;

    Ok(key)
}

/// Checks password strength
pub fn validate_password_strength(password: &str) -> Result<()> {
    if password.len() < 8 {
        anyhow::bail!("Password must be at least 8 characters long");
    }

    if password.len() > 128 {
        anyhow::bail!("Password must be at most 128 characters long");
    }

    let has_letter = password.chars().any(|c| c.is_alphabetic());
    let has_digit = password.chars().any(|c| c.is_numeric());

    if !has_letter || !has_digit {
        println!("⚠️  Security tip: Use both letters and digits in your password");
    }

    Ok(())
}

/// Generates a random PBKDF2 salt
pub fn generate_random_salt() -> [u8; SALT_LENGTH] {
    use rand::Rng;
    let mut salt = [0u8; SALT_LENGTH];
    rand::thread_rng().fill(&mut salt);
    salt
}

/// Generates a random hex key
pub fn generate_random_hex_key() -> String {
    use rand::Rng;
    let mut key = [0u8; 32];
    rand::thread_rng().fill(&mut key);
    hex::encode(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_to_key_derivation() {
        let password = "test_password_123";
        let key1 = derive_key_from_password(password, None).unwrap();
        let key2 = derive_key_from_password(password, None).unwrap();

        // The same password must produce the same key
        assert_eq!(key1, key2);
        assert_eq!(key1.len(), 32);
    }

    #[test]
    fn test_different_passwords_different_keys() {
        let key1 = derive_key_from_password("password1", None).unwrap();
        let key2 = derive_key_from_password("password2", None).unwrap();

        // Different passwords must produce different keys
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_password_validation() {
        // Too short password
        assert!(derive_key_from_password("123", None).is_err());

        // Valid password
        assert!(derive_key_from_password("password123", None).is_ok());

        // Password strength check
        assert!(validate_password_strength("password123").is_ok());
        assert!(validate_password_strength("123").is_err());
    }

    #[test]
    fn test_hex_key_parsing() {
        let hex_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let key = parse_hex_key(hex_key).unwrap();
        assert_eq!(key.len(), 32);

        // Invalid hex
        assert!(parse_hex_key("invalid_hex").is_err());

        // Wrong length
        assert!(parse_hex_key("0123456789abcdef").is_err());
    }

    #[test]
    fn test_random_key_generation() {
        let key1 = generate_random_hex_key();
        let key2 = generate_random_hex_key();

        assert_eq!(key1.len(), 64);
        assert_eq!(key2.len(), 64);
        assert_ne!(key1, key2); // Random keys must differ

        // Check that the generated key is parseable
        assert!(parse_hex_key(&key1).is_ok());
    }
}
