use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

use anyhow::{
    Result,
    anyhow,
};
use postcard::to_allocvec;
use tokio::{
    select,
    sync::mpsc::UnboundedReceiver,
};
use wsio_client::WsIoClient;

use crate::{
    APP_SHUTDOWN_TOKEN,
    CLIENT_CONFIG,
    CRYPTO_KEY,
    crypto::{
        compress,
        encrypt,
    },
    state::LAST_CONTENT_BYTES,
    types::{
        ClipboardContent,
        ClipboardEventData,
        ClipboardPayloadEnvelope,
    },
};

pub(crate) async fn run_clipboard_sender(mut rx: UnboundedReceiver<ClipboardContent>, client: WsIoClient) {
    let key = match CRYPTO_KEY.get() {
        Some(k) => k,
        None => {
            tracing::error!("Crypto key not initialized");
            return;
        }
    };

    loop {
        select! {
            _ = APP_SHUTDOWN_TOKEN.cancelled() => break,
            Some(content) = rx.recv() => {
                if let Err(e) = send_clipboard(&client, key, content).await {
                    tracing::error!("Send clipboard failed: {e}");
                }
            }
        }
    }
}

fn build_payload_envelope(serialized: &[u8], min_compress_size_bytes: usize) -> Result<ClipboardPayloadEnvelope> {
    if serialized.len() < min_compress_size_bytes {
        return Ok(ClipboardPayloadEnvelope::Uncompressed(serialized.to_vec()));
    }

    let compressed = compress(serialized).map_err(|e| anyhow!("Compression failed: {e}"))?;
    if compressed.len() < serialized.len() {
        Ok(ClipboardPayloadEnvelope::Zstd(compressed))
    } else {
        Ok(ClipboardPayloadEnvelope::Uncompressed(serialized.to_vec()))
    }
}

async fn send_clipboard(client: &WsIoClient, key: &chacha20poly1305::Key, content: ClipboardContent) -> Result<()> {
    // Serialize ClipboardContent to bytes
    let serialized = to_allocvec(&content).map_err(|e| anyhow!("Serialize failed: {e}"))?;

    // Skip if content matches LAST_CONTENT_BYTES (circular write prevention)
    if *LAST_CONTENT_BYTES.read().await == serialized {
        return Ok(());
    }

    // Compress only when content exceeds the threshold and compression actually saves bytes.
    let min_size = CLIENT_CONFIG
        .get()
        .expect("CLIENT_CONFIG must be initialized")
        .min_compress_size_bytes;

    let envelope = build_payload_envelope(&serialized, min_size)?;

    let to_encrypt = to_allocvec(&envelope).map_err(|e| anyhow!("Serialize payload envelope failed: {e}"))?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    // Encrypt serialized + compressed content
    let (nonce, encrypted) = encrypt(key, &to_encrypt).map_err(|e| anyhow!("Encryption failed: {e}"))?;

    let event_data = ClipboardEventData {
        device_name: None,
        content: encrypted,
        nonce,
        timestamp,
    };

    // Update LAST_CONTENT_BYTES before sending
    let serialized_size = serialized.len();
    *LAST_CONTENT_BYTES.write().await = serialized;

    match client.emit::<Vec<u8>>("event", Some(&to_allocvec(&event_data)?)).await {
        Ok(_) => tracing::info!("Sent clipboard: {serialized_size} bytes"),
        Err(e) => tracing::error!("Failed to emit clipboard event: {e}"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_envelope_skips_compression_below_threshold() {
        let serialized = vec![0u8; 128];

        let envelope = build_payload_envelope(&serialized, 1024).unwrap();

        assert_eq!(envelope, ClipboardPayloadEnvelope::Uncompressed(serialized));
    }

    #[test]
    fn payload_envelope_uses_compression_when_smaller() {
        let serialized = vec![0u8; 4096];

        let envelope = build_payload_envelope(&serialized, 0).unwrap();

        match envelope {
            ClipboardPayloadEnvelope::Zstd(compressed) => assert!(compressed.len() < serialized.len()),
            ClipboardPayloadEnvelope::Uncompressed(_) => panic!("expected compressible payload to use zstd"),
        }
    }

    #[test]
    fn payload_envelope_skips_compression_when_not_smaller() {
        let serialized = vec![1u8, 2, 3, 4];

        let envelope = build_payload_envelope(&serialized, 0).unwrap();

        assert_eq!(envelope, ClipboardPayloadEnvelope::Uncompressed(serialized));
    }
}
