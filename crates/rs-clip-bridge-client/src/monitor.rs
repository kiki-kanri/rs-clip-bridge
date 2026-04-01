use std::{
    io::Error as IoError,
    thread::spawn,
};

use anyhow::{
    Context,
    Result,
    anyhow,
};
use arboard::Clipboard;
use clipboard_master::{
    CallbackResult,
    ClipboardHandler,
    Master,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    shutdown,
    types::ClipboardContent,
};

struct ClipboardMonitor {
    ctx: Clipboard,
    last_image: Option<(Vec<u8>, usize, usize)>, // (bytes, height, width)
    last_text: String,
    max_image_size_bytes: usize,
    tx: UnboundedSender<ClipboardContent>,
}

impl ClipboardHandler for ClipboardMonitor {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        // Try text first
        match self.ctx.get_text() {
            Ok(text) if !text.is_empty() && text != self.last_text => {
                self.last_text = text.clone();
                tracing::info!("Detected text clipboard change: {} chars", text.len());
                if let Err(e) = self.tx.send(ClipboardContent::Text(text)) {
                    tracing::error!("Failed to send clipboard to channel: {e}");
                    return CallbackResult::Stop;
                }

                CallbackResult::Next
            }
            _ => self.try_get_and_send_image(),
        }
    }

    fn on_clipboard_error(&mut self, error: IoError) -> CallbackResult {
        tracing::error!("Clipboard monitor error: {error}");
        CallbackResult::Next
    }
}

impl ClipboardMonitor {
    pub fn new(tx: UnboundedSender<ClipboardContent>, max_image_size_bytes: usize) -> Result<Self> {
        Ok(Self {
            ctx: Clipboard::new().map_err(|e| anyhow!("Clipboard init error: {e}"))?,
            last_text: String::new(),
            last_image: None,
            max_image_size_bytes: max_image_size_bytes,
            tx,
        })
    }

    fn try_get_and_send_image(&mut self) -> CallbackResult {
        match self.ctx.get_image() {
            Ok(image_data) => {
                let bytes = image_data.bytes.into_owned();
                let height = image_data.height;
                let width = image_data.width;

                // Skip if exceeds max size
                if bytes.len() > self.max_image_size_bytes {
                    tracing::warn!(
                        "Image too large ({} bytes), skipping. Max: {} bytes",
                        bytes.len(),
                        self.max_image_size_bytes
                    );

                    return CallbackResult::Next;
                }

                // Quick size check first — different size = definitely different
                let same_as_last = self
                    .last_image
                    .as_ref()
                    .map_or(false, |(b, h, w)| b.len() == bytes.len() && *h == height && *w == width);

                if same_as_last {
                    // Same dimensions — do full byte comparison only if size also matches
                    let identical = self.last_image.as_ref().map_or(false, |(b, _, _)| *b == bytes);
                    if identical {
                        return CallbackResult::Next;
                    }
                }

                tracing::info!(
                    "Detected image clipboard change: {}x{}, {} bytes",
                    width,
                    height,
                    bytes.len()
                );

                self.last_image = Some((bytes.clone(), height, width));
                if let Err(e) = self.tx.send(ClipboardContent::Image { bytes, height, width }) {
                    tracing::error!("Failed to send clipboard to channel: {e}");
                    return CallbackResult::Stop;
                }
            }
            Err(e) => tracing::debug!("No image on clipboard: {e}"),
        }

        CallbackResult::Next
    }
}

pub fn spawn_clipboard_monitor(tx: UnboundedSender<ClipboardContent>, max_image_size_bytes: usize) {
    spawn(move || {
        if let Err(e) = (|| -> Result<()> {
            let monitor = ClipboardMonitor::new(tx, max_image_size_bytes)?;
            Master::new(monitor).context("Master init failed")?.run()?;
            Ok(())
        })() {
            tracing::error!("Clipboard monitor error: {e}");
            shutdown();
        }
    });
}
