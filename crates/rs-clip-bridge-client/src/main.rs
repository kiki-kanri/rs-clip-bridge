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
    bail,
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
    sync::mpsc::unbounded_channel,
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

// Constants/Statics
static APP_SHUTDOWN_TOKEN: LazyLock<CancellationToken> = LazyLock::new(CancellationToken::new);
static APP_TASK_MANAGER: LazyLock<TaskManager> = LazyLock::new(TaskManager::new);
static CLIENT_CONFIG: OnceLock<ClientConfig> = OnceLock::new();
pub static WS_IO_CLIENT: OnceLock<WsIoClient> = OnceLock::new();

// Functions
async fn handle_ws_io_server_event(_: Arc<WsIoClientSession>, data: Arc<ClipboardEventData>) -> Result<()> {
    let mut clipboard = Clipboard::new().map_err(|err| anyhow!("Clipboard init error: {err}"))?;

    match &data.content {
        ClipboardContent::Text(text) => {
            // Update LAST_CONTENT before writing to prevent circular detection
            *LAST_CONTENT.write().await = text.clone();

            if let Err(err) = clipboard.set_text(text) {
                tracing::error!("Failed to set local clipboard: {err}");
                bail!("Clipboard write error");
            }

            tracing::info!("Successfully synced clipboard from server ({} bytes)", text.len());
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn parse_cli_and_set_config() -> Result<ClientConfig> {
    let cli = Cli::parse();
    let cli_config_layer = ClientConfigLayer {
        auth_key: cli.auth_key,
        channel_id: cli.channel_id,
        display: cli.display,
        server_url: cli.server_url,
    };

    let mut config_builder = ClientConfig::builder().preloaded(cli_config_layer);

    if let Some(path) = cli.config {
        config_builder = config_builder.file(path);
    }

    let config = config_builder
        .load()
        .map_err(|e| anyhow!("Configuration load failed: {}", e))?;

    CLIENT_CONFIG
        .set(config.clone())
        .map_err(|_| anyhow!("Failed to set client config"))?;

    Ok(config)
}

pub fn shutdown() {
    APP_SHUTDOWN_TOKEN.cancel();
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing_with_layer(
        make_tracing_fmt_layer_with_local_time()?
            .with_target(false)
            .with_filter(LevelFilter::INFO),
    )?;

    // Get config
    let config = parse_cli_and_set_config()?;

    // Create and ws.io client
    let ws_io_client = WsIoClient::builder(config.server_url.as_ref())?
        .packet_codec(WsIoPacketCodec::Postcard)
        .with_init_handler(move |_, _: Option<()>| {
            let config = config.clone();
            async move { Ok(Some((config.auth_key, config.channel_id))) }
        })
        .build();

    WS_IO_CLIENT
        .set(ws_io_client.clone())
        .map_err(|_| anyhow!("Failed to set ws.io client"))?;

    // Register event
    ws_io_client.on("event", handle_ws_io_server_event);

    // Connect
    ws_io_client.connect().await;

    // Create clipboard event channel
    let (clipboard_event_tx, mut clipboard_event_rx) = unbounded_channel::<String>();

    // Spawn clipboard event handler task
    spawn_clipboard_monitor(clipboard_event_tx);

    // Spawn read rx task
    let ws_io_client_clone = ws_io_client.clone();
    APP_TASK_MANAGER.spawn_with_token(|token| async move {
        tracing::info!("Clipboard event handler task started");

        loop {
            select! {
                _ = token.cancelled() => break,
                Some(content) = clipboard_event_rx.recv() => {
                    // Check for circular write: skip if content matches LAST_CONTENT
                    if *LAST_CONTENT.read().await == content {
                        continue;
                    }

                    tracing::info!("Detected clipboard change, sending to server ({} bytes)", content.len());

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

                    if let Err(err) = ws_io_client_clone.emit::<ClipboardEventData>("event", Some(&data)).await {
                        tracing::error!("Failed to emit clipboard event: {err}");
                    }
                }
            }
        }

        tracing::info!("Clipboard event handler task stopped");
    });

    // Wait for shutdown signal
    APP_TASK_MANAGER.spawn_with_token(async |token| {
        select! {
            _ = token.cancelled() => {},
            _ = wait_for_shutdown_signal() => {},
        }

        shutdown();
    });

    // Wait for app shutdown
    APP_SHUTDOWN_TOKEN.cancelled().await;
    tracing::info!("Shutting down...");

    ws_io_client.disconnect().await;
    APP_TASK_MANAGER.cancel_and_join_existing().await;

    tracing::info!("Shutdown complete");
    Ok(())
}
