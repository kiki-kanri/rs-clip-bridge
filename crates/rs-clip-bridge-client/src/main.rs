#[cfg(unix)]
use std::env::set_var;
use std::{
    sync::{
        Arc,
        LazyLock,
        OnceLock,
    },
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

use anyhow::{
    Result,
    anyhow,
};
use arboard::Clipboard;
use clap::Parser;
use confique::Config;
use kikiutils::{
    signal::wait_for_shutdown_signal,
    task::manager::TaskManager,
    tracing::{
        init_tracing_with_layer,
        make_tracing_fmt_layer_with_local_time,
    },
};
use rs_clip_bridge_types as types;
use tokio::{
    select,
    sync::mpsc::{
        UnboundedReceiver,
        unbounded_channel,
    },
};
use tokio_util::sync::CancellationToken;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::Layer;
use wsio_client::{
    WsIoClient,
    core::packet::codecs::WsIoPacketCodec,
    session::WsIoClientSession,
};

mod cli;
mod config;
mod monitor;
mod state;

use self::{
    cli::Cli,
    config::{
        ClientConfig,
        confique_client_config_layer::ClientConfigLayer,
    },
    monitor::spawn_clipboard_monitor,
    state::LAST_CONTENT,
    types::{
        ClipboardContent,
        ClipboardEventData,
    },
};

// ================================================================================================
// Application State
// ================================================================================================

static APP_SHUTDOWN_TOKEN: LazyLock<CancellationToken> = LazyLock::new(CancellationToken::new);
static APP_TASK_MANAGER: LazyLock<TaskManager> = LazyLock::new(TaskManager::new);
static CLIENT_CONFIG: OnceLock<ClientConfig> = OnceLock::new();
pub static WS_IO_CLIENT: OnceLock<WsIoClient> = OnceLock::new();

// ================================================================================================
// Initialization
// ================================================================================================

fn init_tracing() -> Result<()> {
    init_tracing_with_layer(
        make_tracing_fmt_layer_with_local_time()?
            .with_target(false)
            .with_filter(LevelFilter::INFO),
    )
}

fn load_config() -> Result<ClientConfig> {
    let cli = Cli::parse();
    let layer = ClientConfigLayer {
        auth_key: cli.auth_key,
        channel_id: cli.channel_id,
        #[cfg(unix)]
        display: cli.display,
        server_url: cli.server_url,
    };

    let mut builder = ClientConfig::builder().preloaded(layer);
    if let Some(path) = cli.config {
        builder = builder.file(path);
    }

    let config = builder.load().map_err(|e| anyhow!("Config load failed: {e}"))?;

    CLIENT_CONFIG
        .set(config.clone())
        .ok()
        .ok_or_else(|| anyhow!("Failed to set client config"))?;

    Ok(config)
}

fn setup_ws_client(config: ClientConfig) -> Result<WsIoClient> {
    let client = WsIoClient::builder(config.server_url.as_ref())?
        .packet_codec(WsIoPacketCodec::Postcard)
        .with_init_handler(move |_, _: Option<()>| {
            let cfg = config.clone();
            async move { Ok(Some((cfg.auth_key, cfg.channel_id))) }
        })
        .build();

    WS_IO_CLIENT.set(client.clone()).ok();

    Ok(client)
}

#[cfg(unix)]
fn setup_display(display: &Option<String>) {
    if let Some(d) = display {
        // SAFETY: Safe before async runtime starts.
        unsafe { set_var("DISPLAY", d) };
    }
}

// ================================================================================================
// Event Handlers
// ================================================================================================

async fn handle_server_event(_: Arc<WsIoClientSession>, data: Arc<ClipboardEventData>) -> Result<()> {
    if let ClipboardContent::Text(text) = &data.content {
        // Update LAST_CONTENT before writing to local clipboard
        *LAST_CONTENT.write().await = text.clone();

        let mut clipboard = Clipboard::new().map_err(|e| anyhow!("Clipboard init error: {e}"))?;
        clipboard
            .set_text(text)
            .map_err(|e| anyhow!("Clipboard write error: {e}"))?;

        tracing::info!("Synced clipboard from server ({} bytes)", text.len());
    }

    Ok(())
}

async fn run_clipboard_sender(mut rx: UnboundedReceiver<String>, client: WsIoClient) {
    loop {
        select! {
            _ = APP_SHUTDOWN_TOKEN.cancelled() => break,
            Some(content) = rx.recv() => {
                // Skip if content matches LAST_CONTENT (circular write prevention)
                if *LAST_CONTENT.read().await == content {
                    continue;
                }

                let data = ClipboardEventData {
                    device_name: None,
                    content: ClipboardContent::Text(content.clone()),
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.as_millis() as u64)
                        .unwrap_or(0),
                };

                // Update LAST_CONTENT before sending
                *LAST_CONTENT.write().await = content;

                if let Err(e) = client.emit::<ClipboardEventData>("event", Some(&data)).await {
                    tracing::error!("Failed to emit clipboard event: {e}");
                }
            }
        }
    }
}

// ================================================================================================
// Runtime
// ================================================================================================

async fn run_signal_handler() {
    select! {
        _ = wait_for_shutdown_signal() => {},
        _ = APP_SHUTDOWN_TOKEN.cancelled() => {},
    }

    shutdown();
}

fn shutdown() {
    APP_SHUTDOWN_TOKEN.cancel();
}

// ================================================================================================
// Entry Point
// ================================================================================================

#[tokio::main]
async fn main() -> Result<()> {
    // --- Init ---
    init_tracing()?;

    // --- Setup ---
    let config = load_config()?;

    #[cfg(unix)]
    setup_display(&config.display);

    let ws_client = setup_ws_client(config)?;

    // Register event handler before connecting
    ws_client.on("event", handle_server_event);
    ws_client.connect().await;

    // Spawn clipboard monitor
    let (tx, rx) = unbounded_channel();
    spawn_clipboard_monitor(tx);

    // --- Runtime: spawn tasks ---
    let ws_client_clone = ws_client.clone();
    APP_TASK_MANAGER.spawn_with_token(|_| async move {
        run_clipboard_sender(rx, ws_client_clone).await;
    });

    APP_TASK_MANAGER.spawn(run_signal_handler());

    // --- Wait for shutdown ---
    APP_SHUTDOWN_TOKEN.cancelled().await;
    tracing::info!("Shutting down...");

    // --- Cleanup ---
    ws_client.disconnect().await;
    APP_TASK_MANAGER.cancel_and_join_existing().await;

    tracing::info!("Shutdown complete");
    Ok(())
}
