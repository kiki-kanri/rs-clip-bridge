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
    last_text: String,
    tx: UnboundedSender<ClipboardContent>,
}

impl ClipboardHandler for ClipboardMonitor {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        match self.ctx.get_text() {
            Ok(text) if !text.is_empty() && text != self.last_text => {
                self.last_text = text.clone();
                tracing::info!("Detected clipboard change: {} chars", text.len());
                if let Err(e) = self.tx.send(ClipboardContent::Text(text)) {
                    tracing::error!("Failed to send clipboard to channel: {e}");
                    return CallbackResult::Stop;
                }
            }
            Err(e) => tracing::warn!("Failed to read clipboard text: {e}"),
            _ => {}
        }

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: IoError) -> CallbackResult {
        tracing::error!("Clipboard monitor error: {error}");
        CallbackResult::Next
    }
}

impl ClipboardMonitor {
    pub fn new(tx: UnboundedSender<ClipboardContent>) -> Result<Self> {
        Ok(Self {
            ctx: Clipboard::new().map_err(|e| anyhow!("Clipboard init error: {e}"))?,
            last_text: String::new(),
            tx,
        })
    }
}

pub fn spawn_clipboard_monitor(tx: UnboundedSender<ClipboardContent>) {
    spawn(move || {
        if let Err(e) = (|| -> Result<()> {
            let monitor = ClipboardMonitor::new(tx)?;
            Master::new(monitor).context("Master init failed")?.run()?;
            Ok(())
        })() {
            tracing::error!("Clipboard monitor error: {e}");
            shutdown();
        }
    });
}
