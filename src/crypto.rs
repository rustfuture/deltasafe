//! Kriptografik yardımcı fonksiyonlar
//! 
//! Bu modül şifreleme anahtarı türetme ve doğrulama işlemlerini içerir.

use anyhow::{Result, Context};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

/// PBKDF2 iterasyon sayısı (güvenlik için yeterli)
const PBKDF2_ITERATIONS: u32 = 100_000;

/// Salt uzunluğu (128 bit)
const SALT_LENGTH: usize = 16;

/// Varsayılan salt (production'da rastgele olmalı, şimdilik sabit)
const DEFAULT_SALT: &[u8] = b"deltasafe_salt16";

/// Şifreden AES-256 anahtarı türetir
/// 
/// # Arguments
/// * `password` - Kullanıcı şifresi
/// * `salt` - Opsiyonel salt (None ise varsayılan kullanılır)
/// 
/// # Returns
/// 32 baytlık AES anahtarı
pub fn derive_key_from_password(password: &str, salt: Option<&[u8]>) -> Result<[u8; 32]> {
    if password.len() < 8 {
        anyhow::bail!("Şifre en az 8 karakter olmalıdır");
    }
    
    let salt = salt.unwrap_or(DEFAULT_SALT);
    if salt.len() != SALT_LENGTH {
        anyhow::bail!("Salt {} bayt uzunluğunda olmalıdır", SALT_LENGTH);
    }
    
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key);
    
    Ok(key)
}

/// Hex string'i 32 baytlık anahtara çevirir
pub fn parse_hex_key(hex_key: &str) -> Result<[u8; 32]> {
    if hex_key.len() != 64 {
        anyhow::bail!("Hex anahtar 64 karakter uzunluğunda olmalıdır (32 bayt)");
    }
    
    let decoded = hex::decode(hex_key)
        .context("Geçersiz hex formatı")?;
    
    let key: [u8; 32] = decoded.try_into()
        .map_err(|_| anyhow::anyhow!("Anahtar 32 bayt uzunluğunda olmalıdır"))?;
    
    Ok(key)
}

/// Şifre güçlülüğünü kontrol eder
pub fn validate_password_strength(password: &str) -> Result<()> {
    if password.len() < 8 {
        anyhow::bail!("Şifre en az 8 karakter olmalıdır");
    }
    
    if password.len() > 128 {
        anyhow::bail!("Şifre en fazla 128 karakter olabilir");
    }
    
    let has_letter = password.chars().any(|c| c.is_alphabetic());
    let has_digit = password.chars().any(|c| c.is_numeric());
    
    if !has_letter || !has_digit {
        println!("⚠️  Güvenlik önerisi: Şifrenizde hem harf hem rakam bulunması önerilir");
    }
    
    Ok(())
}

/// Rastgele hex anahtar üretir
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
        
        // Aynı şifre aynı anahtarı üretmeli
        assert_eq!(key1, key2);
        assert_eq!(key1.len(), 32);
    }
    
    #[test]
    fn test_different_passwords_different_keys() {
        let key1 = derive_key_from_password("password1", None).unwrap();
        let key2 = derive_key_from_password("password2", None).unwrap();
        
        // Farklı şifreler farklı anahtarlar üretmeli
        assert_ne!(key1, key2);
    }
    
    #[test]
    fn test_password_validation() {
        // Çok kısa şifre
        assert!(derive_key_from_password("123", None).is_err());
        
        // Geçerli şifre
        assert!(derive_key_from_password("password123", None).is_ok());
        
        // Şifre güçlülük kontrolü
        assert!(validate_password_strength("password123").is_ok());
        assert!(validate_password_strength("123").is_err());
    }
    
    #[test]
    fn test_hex_key_parsing() {
        let hex_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let key = parse_hex_key(hex_key).unwrap();
        assert_eq!(key.len(), 32);
        
        // Geçersiz hex
        assert!(parse_hex_key("invalid_hex").is_err());
        
        // Yanlış uzunluk
        assert!(parse_hex_key("0123456789abcdef").is_err());
    }
    
    #[test]
    fn test_random_key_generation() {
        let key1 = generate_random_hex_key();
        let key2 = generate_random_hex_key();
        
        assert_eq!(key1.len(), 64);
        assert_eq!(key2.len(), 64);
        assert_ne!(key1, key2); // Rastgele anahtarlar farklı olmalı
        
        // Üretilen anahtarın parse edilebilir olduğunu kontrol et
        assert!(parse_hex_key(&key1).is_ok());
    }
}