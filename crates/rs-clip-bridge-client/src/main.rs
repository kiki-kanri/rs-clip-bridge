#[cfg(unix)]
use std::env::set_var;
use std::{
    borrow::Cow,
    process::exit,
    sync::{
        Arc,
        LazyLock,
        OnceLock,
    },
};

use anyhow::{
    Context,
    Result,
    anyhow,
};
use arboard::{
    Clipboard,
    ImageData,
};
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
use postcard::from_bytes;
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
mod crypto;
mod monitor;
mod sender;
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
        decompress,
        decrypt,
        parse_key,
    },
    monitor::spawn_clipboard_monitor,
    sender::run_clipboard_sender,
    state::LAST_CONTENT_BYTES,
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

pub fn init_rustls_crypto_provider() -> Result<()> {
    #[cfg(feature = "rustls-aws-lc-rs")]
    let install_result = rustls::crypto::aws_lc_rs::default_provider().install_default();

    #[cfg(all(not(feature = "rustls-aws-lc-rs"), feature = "rustls-ring"))]
    let install_result = rustls::crypto::ring::default_provider().install_default();

    #[cfg(any(feature = "rustls-aws-lc-rs", feature = "rustls-ring"))]
    install_result.map_err(|e| anyhow!("Failed to install rustls crypto provider: {e:?}"))?;

    #[cfg(not(any(feature = "rustls-aws-lc-rs", feature = "rustls-ring")))]
    anyhow::bail!("No rustls crypto provider selected. Enable 'rustls-ring' or 'rustls-aws-lc-rs'");

    Ok(())
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
        max_image_size_bytes: cli.max_image_size_bytes,
        min_compress_size_bytes: cli.min_compress_size_bytes,
        server_url: cli.server_url,
    };

    let mut builder = ClientConfig::builder().preloaded(layer);
    if let Some(path) = cli.config {
        builder = builder.file(path);
    }

    let config = builder.load().context("Config load failed")?;

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

async fn handle_server_event(_: Arc<WsIoClientSession>, data_bytes: Arc<Vec<u8>>) -> Result<()> {
    let key = CRYPTO_KEY.get().context("Crypto key not initialized")?;
    let data = from_bytes::<ClipboardEventData>(&data_bytes)?;

    // Decrypt content
    let plaintext = decrypt(key, &data.nonce, &data.content).context("Decryption failed")?;

    // Decompress if needed (magic byte: 0x01 = zstd, 0x00 = uncompressed)
    let decompressed = if plaintext.first() == Some(&0x01) {
        decompress(&plaintext[1..]).context("Decompression failed")?
    } else {
        plaintext[1..].to_vec()
    };

    // Deserialize ClipboardContent
    let content = from_bytes(&decompressed).context("Deserialize clipboard content failed")?;

    // Update LAST_CONTENT_BYTES before writing to local clipboard
    *LAST_CONTENT_BYTES.write().await = decompressed.clone();

    // Write to local clipboard based on type
    let mut clipboard = Clipboard::new().context("Clipboard init failed")?;
    match &content {
        ClipboardContent::Text(text) => {
            clipboard.set_text(text).context("Clipboard write failed")?;

            tracing::info!("Received clipboard from server: {} bytes", decompressed.len());
        }
        ClipboardContent::Image { bytes, height, width } => {
            clipboard
                .set_image(ImageData {
                    bytes: Cow::Borrowed(bytes),
                    height: *height,
                    width: *width,
                })
                .context("Image clipboard write failed")?;

            tracing::info!(
                "Received image from server: {}x{}, {} bytes",
                width,
                height,
                bytes.len()
            );
        }
        ClipboardContent::Raw(_) => tracing::warn!("Raw clipboard not supported"),
    }

    Ok(())
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
    init_rustls_crypto_provider()?;
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

    let ws_client = setup_ws_client(config.clone())?;

    // Register event handler before connecting
    ws_client.on("event", handle_server_event);
    ws_client.connect().await;

    // Spawn clipboard monitor
    let (tx, rx) = unbounded_channel();
    spawn_clipboard_monitor(tx, config.max_image_size_bytes);

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
