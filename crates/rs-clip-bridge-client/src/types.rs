use serde::{
    Deserialize,
    Serialize,
};

/// Clipboard content type
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ClipboardContent {
    Image {
        bytes: Vec<u8>,
        height: usize,
        width: usize,
    },
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

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use postcard::{
        from_bytes,
        to_allocvec,
    };

    use super::*;

    #[test]
    fn clipboard_event_data_serde() {
        let data = ClipboardEventData {
            device_name: Some("test-device".to_string()),
            content: vec![0xde, 0xad, 0xbe, 0xef],
            nonce: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c],
            timestamp: 1234567890,
        };

        let encoded = to_allocvec(&data).unwrap();
        let decoded: ClipboardEventData = from_bytes(&encoded).unwrap();
        assert_eq!(decoded.device_name, data.device_name);
        assert_eq!(decoded.content, data.content);
        assert_eq!(decoded.nonce, data.nonce);
        assert_eq!(decoded.timestamp, data.timestamp);
    }

    #[test]
    fn clipboard_content_serde() {
        let content = ClipboardContent::Text("Hello, World!".to_string());
        let encoded = to_allocvec(&content).unwrap();
        let decoded: ClipboardContent = from_bytes(&encoded).unwrap();
        assert_eq!(decoded, content);

        let raw = ClipboardContent::Raw(vec![0xff, 0x00, 0xff]);
        let encoded = to_allocvec(&raw).unwrap();
        let decoded: ClipboardContent = from_bytes(&encoded).unwrap();
        assert_eq!(decoded, raw);

        let image = ClipboardContent::Image {
            bytes: vec![0x89, 0x50, 0x4e],
            height: 1,
            width: 1,
        }; // PNG magic

        let encoded = to_allocvec(&image).unwrap();
        let decoded: ClipboardContent = from_bytes(&encoded).unwrap();
        assert_eq!(decoded, image);
    }

    #[test]
    fn clipboard_event_data_no_device_name() {
        let data = ClipboardEventData {
            device_name: None,
            content: vec![0u8; 32],
            nonce: vec![0u8; 12],
            timestamp: 0,
        };

        let encoded = to_allocvec(&data).unwrap();
        let decoded: ClipboardEventData = from_bytes(&encoded).unwrap();
        assert_eq!(decoded.device_name, None);
    }

    #[test]
    fn clipboard_content_large_image() {
        let large_bytes = vec![0xabu8; 1024 * 1024];
        let image = ClipboardContent::Image {
            bytes: large_bytes.clone(),
            height: 1920,
            width: 1080,
        };

        let encoded = to_allocvec(&image).unwrap();
        let decoded: ClipboardContent = from_bytes(&encoded).unwrap();

        match decoded {
            ClipboardContent::Image { bytes, height, width } => {
                assert_eq!(bytes, large_bytes);
                assert_eq!(height, 1920);
                assert_eq!(width, 1080);
            }
            _ => panic!("Expected Image variant"),
        }
    }

    #[test]
    fn clipboard_content_unicode_text() {
        let texts = [
            "Hello, World!",
            "你好，世界！",
            "🎉 🚀 🌙",
            "Line1\nLine2\nLine3",
            "Tab\there",
        ];

        for text in texts {
            let content = ClipboardContent::Text(text.to_string());
            let encoded = to_allocvec(&content).unwrap();
            let decoded: ClipboardContent = from_bytes(&encoded).unwrap();
            assert_eq!(decoded, content);
        }
    }

    #[test]
    fn clipboard_content_raw_binary() {
        let raw = ClipboardContent::Raw(vec![0x00, 0xff, 0x42, 0x13, 0x37]);
        let encoded = to_allocvec(&raw).unwrap();
        let decoded: ClipboardContent = from_bytes(&encoded).unwrap();
        assert_eq!(decoded, raw);
    }

    #[test]
    fn clipboard_event_data_all_zeros() {
        let data = ClipboardEventData {
            device_name: None,
            content: vec![0u8; 64],
            nonce: vec![0u8; 12],
            timestamp: 0,
        };

        let encoded = to_allocvec(&data).unwrap();
        let decoded: ClipboardEventData = from_bytes(&encoded).unwrap();
        assert_eq!(decoded.content.len(), 64);
        assert!(decoded.device_name.is_none());
    }
}
