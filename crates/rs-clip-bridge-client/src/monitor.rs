use std::{
    io::Error as IoError,
    thread::spawn,
};

use anyhow::{
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

use crate::shutdown;

struct ClipboardMonitor {
    ctx: Clipboard,
    last_content: String,
    tx: UnboundedSender<String>,
}

impl ClipboardHandler for ClipboardMonitor {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        match self.ctx.get_text() {
            Ok(text) => {
                if !text.is_empty() && text != self.last_content {
                    self.last_content = text.clone();
                    if let Err(e) = self.tx.send(text) {
                        tracing::error!("Failed to send clipboard content to channel: {e}");
                        return CallbackResult::Stop;
                    }
                }
            }
            Err(e) => tracing::warn!("Failed to read clipboard text: {e}"),
        }

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: IoError) -> CallbackResult {
        tracing::error!("Clipboard monitor error: {error}");
        CallbackResult::Next
    }
}

impl ClipboardMonitor {
    pub fn new(tx: UnboundedSender<String>) -> Result<Self> {
        Ok(Self {
            ctx: Clipboard::new().map_err(|err| anyhow!("Failed to init clipboard: {err}"))?,
            last_content: String::new(),
            tx,
        })
    }
}

pub fn spawn_clipboard_monitor(tx: UnboundedSender<String>) {
    spawn(move || {
        let result = (|| -> Result<()> {
            let monitor = ClipboardMonitor::new(tx)?;
            let mut master = Master::new(monitor).map_err(|_| anyhow!("Master init failed"))?;
            master.run()?;
            Ok(())
        })();

        if let Err(err) = result {
            tracing::error!("Clipboard monitor error: {err}");
            shutdown();
        }
    });
}
