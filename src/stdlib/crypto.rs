//! Simple crypto utilities: hashing, HMAC, and Base64 helpers.

use crate::OmniError;
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use hmac::{Hmac, Mac};
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

/// SHA-256 hash of UTF-8 input, hex-encoded.
pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let out = hasher.finalize();
    hex_lower(&out)
}

/// HMAC-SHA256 of UTF-8 message with UTF-8 key, hex-encoded.
pub fn hmac_sha256(key: &str, message: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes())
        .expect("HMAC key construction should not fail");
    mac.update(message.as_bytes());
    let out = mac.finalize().into_bytes();
    hex_lower(&out)
}

/// Base64-encode UTF-8 string using standard alphabet (no padding changes).
pub fn base64_encode(input: &str) -> String {
    B64.encode(input.as_bytes())
}

/// Base64-decode into UTF-8 string.
pub fn base64_decode(input: &str) -> Result<String, OmniError> {
    let bytes = B64
        .decode(input)
        .map_err(|e| OmniError::InvalidInput(format!("base64 decode: {}", e)))?;
    String::from_utf8(bytes).map_err(|e| OmniError::InvalidInput(format!("utf8 decode: {}", e)))
}

/// Generate cryptographically secure random bytes with OS RNG.
pub fn random_bytes(len: usize) -> Result<Vec<u8>, OmniError> {
    if len == 0 {
        return Err(OmniError::InvalidInput("random_bytes: len must be > 0".to_string()));
    }
    let mut buf = vec![0u8; len];
    OsRng.fill_bytes(&mut buf);
    Ok(buf)
}

/// Generate hex-encoded nonce of the given byte length.
pub fn random_hex(len: usize) -> Result<String, OmniError> {
    let bytes = random_bytes(len)?;
    Ok(hex_lower(&bytes))
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha_and_hmac_work() {
        assert_eq!(
            hash_sha256("hello"),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );

        assert_eq!(
            hmac_sha256("key", "message"),
            "6e9ef29b75fffc5b7abae527d58fdadb2fe42e7219011976917343065f58ed4a"
        );
    }

    #[test]
    fn base64_encode_decode_roundtrip() {
        let input = "OmniLang-test";
        let encoded = base64_encode(input);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn random_hex_has_length_and_is_different() {
        let a = random_hex(16).unwrap();
        let b = random_hex(16).unwrap();
        assert_eq!(a.len(), 32);
        assert_eq!(b.len(), 32);
        // Very small chance of collision; acceptable for a sanity check.
        assert_ne!(a, b);
    }
}
