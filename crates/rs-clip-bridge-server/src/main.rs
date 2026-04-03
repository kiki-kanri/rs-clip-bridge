use std::{
    process::exit,
    sync::{
        LazyLock,
        OnceLock,
    },
};

use anyhow::{
    Context,
    Result,
    anyhow,
};
use axum::{
    Router,
    http::StatusCode,
    routing::get,
};
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
use tokio::{
    net::TcpListener,
    select,
};
use tokio_util::sync::CancellationToken;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::Layer;
use wsio_server::{
    WsIoServer,
    core::packet::codecs::WsIoPacketCodec,
    request_adapters::tower::layer::WsIoServerLayer,
};

mod cli;
mod config;
mod namespaces;

use self::{
    cli::{
        Cli,
        Commands,
        run_generate_config_template,
    },
    config::{
        ServerConfig,
        confique_server_config_layer::ServerConfigLayer,
    },
};

// ================================================================================================
// Application State
// ================================================================================================

static APP_SHUTDOWN_TOKEN: LazyLock<CancellationToken> = LazyLock::new(CancellationToken::new);
static APP_TASK_MANAGER: LazyLock<TaskManager> = LazyLock::new(TaskManager::new);
static SERVER_CONFIG: OnceLock<ServerConfig> = OnceLock::new();
pub static WS_IO_SERVER: LazyLock<WsIoServer> =
    LazyLock::new(|| WsIoServer::builder().packet_codec(WsIoPacketCodec::Postcard).build());

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

const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"));

fn load_config() -> Result<ServerConfig> {
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

    let layer = ServerConfigLayer {
        auth_keys: cli.auth_keys,
        host: cli.host,
        port: cli.port,
    };

    let mut builder = ServerConfig::builder().preloaded(layer);
    if let Some(path) = cli.config {
        builder = builder.file(path);
    }

    let config = builder.load().context("Config load failed")?;

    SERVER_CONFIG
        .set(config.clone())
        .ok()
        .ok_or_else(|| anyhow!("Failed to set server config"))?;

    Ok(config)
}

fn setup_server_layer() -> WsIoServerLayer {
    namespaces::main::MAIN.path();
    WS_IO_SERVER.layer()
}

async fn health_handler() -> StatusCode {
    StatusCode::OK
}

// ================================================================================================
// Runtime
// ================================================================================================

async fn run_server(cancel: CancellationToken) -> Result<()> {
    let config = SERVER_CONFIG.get().context("Server config not initialized")?;

    let addr = format!("{}:{}", config.host, config.port);

    tracing::info!("Starting server on {addr}");

    let app = Router::new()
        .route("/health", get(health_handler))
        .layer(setup_server_layer());

    let listener = TcpListener::bind(&addr).await.context("Failed to bind TCP listener")?;

    tracing::info!(
        "Listening on {}",
        listener.local_addr().context("Failed to get local address")?
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(async move { cancel.cancelled().await })
        .await
        .context("Server error")?;

    tracing::info!("Server stopped");
    Ok(())
}

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
    tracing::info!(version = VERSION, "Starting rs-clip-bridge-server");

    // --- Setup ---
    load_config()?;

    // --- Runtime: spawn tasks ---
    APP_TASK_MANAGER.spawn_with_token(|token| async {
        if run_server(token).await.is_err() {
            shutdown();
        }
    });

    APP_TASK_MANAGER.spawn(run_signal_handler());

    // --- Wait for shutdown ---
    APP_SHUTDOWN_TOKEN.cancelled().await;
    tracing::info!("Shutting down...");

    // --- Cleanup ---
    WS_IO_SERVER.shutdown().await;
    APP_TASK_MANAGER.cancel_and_join_existing().await;

    tracing::info!("Shutdown complete");
    Ok(())
}
