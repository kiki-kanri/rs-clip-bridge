//! Shared types for rs-clip-bridge
//!
//! This crate contains common data structures used by both the client and server.

use serde::{
    Deserialize,
    Serialize,
};

/// Represents the content of a clipboard operation.
#[derive(Clone, Debug, Deserialize, Serialize)]
// #[serde(tag = "type", content = "payload")]
pub enum ClipboardContent {
    /// Image data as raw bytes
    Image(Vec<u8>),

    /// Plain text content
    Text(String),

    /// Raw data with explicit format
    Raw { data: Vec<u8>, format: String },
}

/// Event data sent when clipboard content changes.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClipboardEventData {
    /// Optional device name that originated this event
    pub device_name: Option<String>,

    /// The clipboard content
    pub content: ClipboardContent,

    /// Unix timestamp in milliseconds
    pub timestamp: u64,
}
