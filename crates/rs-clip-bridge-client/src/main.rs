#[cfg(unix)]
use std::env::set_var;
use std::{
    process::exit,
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
    Context,
    Result,
    anyhow,
};
use arboard::Clipboard;
use chacha20poly1305::Key;
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
use postcard::{
    from_bytes,
    to_allocvec,
};
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
mod crypto;
mod monitor;
mod state;
mod types;

use self::{
    cli::{
        Cli,
        Commands,
        run_generate_config_template,
    },
    config::{
        ClientConfig,
        confique_client_config_layer::ClientConfigLayer,
    },
    crypto::{
        decrypt,
        encrypt,
        parse_key,
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
static CRYPTO_KEY: OnceLock<Key> = OnceLock::new();
pub static WS_IO_CLIENT: OnceLock<WsIoClient> = OnceLock::new();

// ================================================================================================
// Initialization
// ================================================================================================

pub fn init_rustls_provider() -> Result<()> {
    #[cfg(feature = "rustls-aws-lc-rs")]
    {
        use rustls::crypto::aws_lc_rs::default_provider;

        let _ = default_provider().install_default();
        return Ok(());
    }

    #[cfg(feature = "rustls-ring")]
    {
        use rustls::crypto::ring::default_provider;

        let _ = default_provider().install_default();
        return Ok(());
    }

    #[cfg(all(not(feature = "rustls-ring"), not(feature = "rustls-aws-lc-rs")))]
    {
        use anyhow::bail;

        bail!("No rustls crypto provider selected. Please enable 'rustls-ring' or 'rustls-aws-lc-rs' feature.");
    }
}

fn init_tracing() -> Result<()> {
    init_tracing_with_layer(
        make_tracing_fmt_layer_with_local_time()?
            .with_target(false)
            .with_filter(LevelFilter::INFO),
    )
}

fn load_config() -> Result<ClientConfig> {
    let cli = Cli::parse();

    // Handle subcommands
    if let Some(cmd) = cli.command {
        match cmd {
            Commands::GenerateConfigTemplate { output } => {
                run_generate_config_template(output);
                exit(0);
            }
        }
    }

    let layer = ClientConfigLayer {
        auth_key: cli.auth_key,
        channel_id: cli.channel_id,
        #[cfg(unix)]
        display: cli.display,
        encrypt_key: cli.encrypt_key,
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
        .on_session_close(|_| async {
            tracing::info!("Disconnected from server");
            Ok(())
        })
        .on_session_ready(|_| async {
            tracing::info!("Connected to server");
            Ok(())
        })
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
        // SAFETY: safe before async runtime starts
        unsafe { set_var("DISPLAY", d) };
    }
}

// ================================================================================================
// Event Handlers
// ================================================================================================

async fn handle_server_event(_: Arc<WsIoClientSession>, data: Arc<ClipboardEventData>) -> Result<()> {
    let key = CRYPTO_KEY.get().context("Crypto key not initialized")?;

    // Decrypt content
    let plaintext = decrypt(key, &data.nonce, &data.content).map_err(|e| anyhow!("Decryption failed: {e}"))?;

    // Deserialize ClipboardContent
    let content: ClipboardContent = from_bytes(&plaintext).map_err(|e| anyhow!("Deserialize failed: {e}"))?;

    // Update LAST_CONTENT before writing to local clipboard
    *LAST_CONTENT.write().await = plaintext.clone();

    // Write to local clipboard based on type
    let mut clipboard = Clipboard::new().map_err(|e| anyhow!("Clipboard init error: {e}"))?;
    match &content {
        ClipboardContent::Text(text) => {
            clipboard
                .set_text(text)
                .map_err(|e| anyhow!("Clipboard write error: {e}"))?;

            tracing::info!("Received clipboard from server: {} bytes", plaintext.len());
        }
        ClipboardContent::Image(_) | ClipboardContent::Raw(_) => {
            tracing::warn!("Image/Raw clipboard not yet supported");
        }
    }

    Ok(())
}

async fn run_clipboard_sender(mut rx: UnboundedReceiver<ClipboardContent>, client: WsIoClient) {
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
                // Serialize ClipboardContent to bytes
                let serialized = match to_allocvec(&content) {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::error!("Serialize failed: {e}");
                        continue;
                    }
                };

                // Skip if content matches LAST_CONTENT (circular write prevention)
                if *LAST_CONTENT.read().await == serialized {
                    continue;
                }

                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0);

                // Encrypt serialized content
                let (nonce, encrypted) = match encrypt(key, &serialized) {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::error!("Encryption failed: {e}");
                        continue;
                    }
                };

                let event_data = ClipboardEventData {
                    device_name: None,
                    content: encrypted,
                    nonce,
                    timestamp,
                };

                // Update LAST_CONTENT before sending
                let serialized_size = serialized.len();
                *LAST_CONTENT.write().await = serialized;

                match client.emit::<ClipboardEventData>("event", Some(&event_data)).await {
                    Ok(_) => tracing::info!("Sent clipboard: {serialized_size} bytes"),
                    Err(e) => tracing::error!("Failed to emit clipboard event: {e}"),
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
    init_rustls_provider()?;
    tracing::info!("Starting rs-clip-bridge-client");

    // --- Setup ---
    let config = load_config()?;
    tracing::info!(
        channel_id = %config.channel_id,
        server_url = %config.server_url,
        "Configuration loaded"
    );

    // Parse and store encryption key
    let key = parse_key(&config.encrypt_key)?;
    CRYPTO_KEY.set(key).ok();

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
