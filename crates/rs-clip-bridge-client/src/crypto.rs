//! E2E Encryption using ChaCha20-Poly1305
//!
//! The encrypt_key is a 64-character hex string (32 bytes).
//! A random 12-byte nonce is generated for each encryption.

use std::{
    io::copy,
    thread::available_parallelism,
};

use anyhow::{
    Result,
    anyhow,
    bail,
};
pub use chacha20poly1305::Key;
use chacha20poly1305::{
    ChaCha20Poly1305,
    Nonce,
    aead::{
        Aead,
        KeyInit,
    },
};
use hex::decode_to_slice;
use rand::random;
use zstd::stream::{
    Encoder,
    decode_all,
};

/// Parse a 64-char hex string into a 32-byte key
pub fn parse_key(hex_key: &str) -> Result<Key> {
    if hex_key.len() != 64 {
        return Err(anyhow!("encrypt_key must be 64 hex characters (32 bytes)"));
    }

    let mut key = Key::default();
    decode_to_slice(hex_key.as_bytes(), &mut key)?;
    Ok(key)
}

/// Encrypt plaintext with ChaCha20-Poly1305.
/// Returns (nonce, content_with_tag) — content is ciphertext || poly1305_tag.
pub fn encrypt(key: &Key, plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let cipher = ChaCha20Poly1305::new(key);

    // Generate random 12-byte nonce
    let nonce_bytes: [u8; 12] = random();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let content_with_tag = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| anyhow!("Encryption failed: {e}"))?;

    Ok((nonce_bytes.to_vec(), content_with_tag))
}

/// Decrypt content_with_tag (ciphertext || poly1305_tag) using ChaCha20-Poly1305.
pub fn decrypt(key: &Key, nonce: &[u8], content_with_tag: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key);

    if nonce.len() != 12 {
        bail!("Nonce must be 12 bytes");
    }

    let nonce = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce, content_with_tag)
        .map_err(|e| anyhow!("Decryption failed: {e}"))
}

/// Compress data using zstd with multi-threading (zstdmt).
/// Uses level 1 (fast) for near-lz4 speed with better compression.
pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
    let workers = available_parallelism().map(|n| n.get()).unwrap_or(1);
    let mut result = Vec::new();

    let mut encoder = Encoder::new(&mut result, 1)?;
    encoder.multithread(workers as u32)?;
    copy(&mut &data[..], &mut encoder)?;
    encoder.finish()?;

    Ok(result)
}

/// Decompress zstd-compressed data.
pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    decode_all(data).map_err(|e| anyhow!("Decompression failed: {e}"))
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_KEY: &str = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";

    #[test]
    fn parse_key_valid() {
        let key = parse_key(VALID_KEY).unwrap();
        // Key is 32 bytes (256 bits for ChaCha20)
        let bytes: &[u8] = key.as_ref();
        assert_eq!(bytes.len(), 32);
    }

    #[test]
    fn parse_key_too_short() {
        let err = parse_key("abc123").unwrap_err();
        assert!(err.to_string().contains("64 hex characters"));
    }

    #[test]
    fn parse_key_too_long() {
        let err = parse_key(&"a".repeat(65)).unwrap_err();
        assert!(err.to_string().contains("64 hex characters"));
    }

    #[test]
    fn parse_key_invalid_hex() {
        let err = parse_key(&"g".repeat(64)).unwrap_err();
        // hex crate returns "Invalid character 'g'" for non-hex input
        let msg = err.to_string();
        assert!(
            msg.contains("Invalid") || msg.contains("Odd") || msg.contains("Hex"),
            "unexpected error: {}",
            msg
        );
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let key = parse_key(VALID_KEY).unwrap();
        let plaintext = b"Hello, World! \xe4\xbd\xa0\xe5\xa5\xbd"; // "你好" in UTF-8

        let (nonce, ciphertext) = encrypt(&key, plaintext).unwrap();
        assert_eq!(nonce.len(), 12);
        assert!(!ciphertext.is_empty());

        let decrypted = decrypt(&key, &nonce, &ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn encrypt_produces_different_nonces() {
        let key = parse_key(VALID_KEY).unwrap();
        let plaintext = b"Hello";

        let (_, ct1) = encrypt(&key, plaintext).unwrap();
        let (_, ct2) = encrypt(&key, plaintext).unwrap();

        // Same plaintext, different nonces -> different ciphertext
        assert_ne!(ct1, ct2);
    }

    #[test]
    fn decrypt_wrong_key_fails() {
        let key1 = parse_key(VALID_KEY).unwrap();
        let key2 = parse_key("1f1e1d1c1b1a191817161514131211100f0e0d0c0b0a09080706050403020100").unwrap();

        let (nonce, ciphertext) = encrypt(&key1, b"secret").unwrap();

        let err = decrypt(&key2, &nonce, &ciphertext).unwrap_err();
        assert!(err.to_string().contains("Decryption failed"));
    }

    #[test]
    fn decrypt_invalid_tag_fails() {
        let key = parse_key(VALID_KEY).unwrap();
        let (nonce, mut ciphertext) = encrypt(&key, b"secret").unwrap();

        // Corrupt the tag (last 16 bytes of content_with_tag)
        let len = ciphertext.len();
        ciphertext[len - 1] ^= 0xff;

        let err = decrypt(&key, &nonce, &ciphertext).unwrap_err();
        assert!(err.to_string().contains("Decryption failed"));
    }

    #[test]
    fn decrypt_nonce_wrong_length_fails() {
        let key = parse_key(VALID_KEY).unwrap();
        let (_, ciphertext) = encrypt(&key, b"hello").unwrap();

        // Empty nonce
        let err = decrypt(&key, &[], &ciphertext).unwrap_err();
        assert!(err.to_string().contains("12 bytes"));

        // Short nonce
        let err = decrypt(&key, &[1, 2, 3], &ciphertext).unwrap_err();
        assert!(err.to_string().contains("12 bytes"));
    }

    #[test]
    fn encrypt_empty_plaintext() {
        let key = parse_key(VALID_KEY).unwrap();
        let (nonce, ciphertext) = encrypt(&key, b"").unwrap();
        assert_eq!(nonce.len(), 12);
        assert_eq!(ciphertext.len(), 16); // Just the Poly1305 tag

        let decrypted = decrypt(&key, &nonce, &ciphertext).unwrap();
        assert_eq!(decrypted, b"");
    }

    #[test]
    fn encrypt_large_plaintext() {
        let key = parse_key(VALID_KEY).unwrap();
        let plaintext = vec![0u8; 1024 * 1024]; // 1MB

        let (nonce, ciphertext) = encrypt(&key, &plaintext).unwrap();
        let decrypted = decrypt(&key, &nonce, &ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
