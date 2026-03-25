//! E2E Encryption using ChaCha20-Poly1305
//!
//! The encrypt_key is a 64-character hex string (32 bytes).
//! A random 12-byte nonce is generated for each encryption.

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
        .map_err(|e| anyhow!("Encryption failed: {}", e))?;

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
        .map_err(|e| anyhow!("Decryption failed: {}", e))
}
