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

async fn send_clipboard(client: &WsIoClient, key: &chacha20poly1305::Key, content: ClipboardContent) -> Result<()> {
    // Serialize ClipboardContent to bytes
    let serialized = to_allocvec(&content).map_err(|e| anyhow!("Serialize failed: {e}"))?;

    // Skip if content matches LAST_CONTENT_BYTES (circular write prevention)
    if *LAST_CONTENT_BYTES.read().await == serialized {
        return Ok(());
    }

    // Compress if content exceeds minimum size threshold
    let min_size = CLIENT_CONFIG
        .get()
        .expect("CLIENT_CONFIG must be initialized")
        .min_compress_size_bytes;

    let to_encrypt = if serialized.len() >= min_size {
        let compressed = compress(&serialized).map_err(|e| anyhow!("Compression failed: {e}"))?;

        let mut data = vec![0x01u8]; // magic: zstd compressed
        data.extend(compressed);
        data
    } else {
        let mut data = vec![0x00u8]; // magic: uncompressed
        data.extend(serialized.clone());
        data
    };

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

    match client.emit::<ClipboardEventData>("event", Some(&event_data)).await {
        Ok(_) => tracing::info!("Sent clipboard: {serialized_size} bytes"),
        Err(e) => tracing::error!("Failed to emit clipboard event: {e}"),
    }

    Ok(())
}
