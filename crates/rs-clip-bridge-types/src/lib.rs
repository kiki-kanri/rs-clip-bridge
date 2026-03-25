//! Shared types for rs-clip-bridge
//!
//! This crate contains common data structures used by both the client and server.

use serde::{
    Deserialize,
    Serialize,
};

/// Clipboard content type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ClipboardContent {
    Image(Vec<u8>),
    Text(String),
    Raw(Vec<u8>),
}

/// Event data sent when clipboard content changes.
///
/// Content is encrypted using ChaCha20-Poly1305.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClipboardEventData {
    /// Optional device name that originated this event
    pub device_name: Option<String>,

    /// Encrypted content: `[ciphertext || poly1305_tag]`
    pub content: Vec<u8>,

    /// ChaCha20-Poly1305 nonce (12 bytes)
    pub nonce: Vec<u8>,

    /// Unix timestamp in milliseconds
    pub timestamp: u64,
}
