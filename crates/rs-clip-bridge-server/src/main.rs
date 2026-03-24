use std::sync::{
    LazyLock,
    OnceLock,
};

use anyhow::{
    Result,
    anyhow,
    bail,
};
use axum::Router;
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
    cli::Cli,
    config::{
        AppConfig,
        confique_app_config_layer::AppConfigLayer,
    },
};

// Constants/Statics
static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();
static APP_SHUTDOWN_TOKEN: LazyLock<CancellationToken> = LazyLock::new(CancellationToken::new);
static APP_TASK_MANAGER: LazyLock<TaskManager> = LazyLock::new(TaskManager::new);
pub static WS_IO_SERVER: LazyLock<WsIoServer> =
    LazyLock::new(|| WsIoServer::builder().packet_codec(WsIoPacketCodec::Postcard).build());

// Functions
fn init_namespaces_and_get_tower_layer() -> WsIoServerLayer {
    namespaces::main::MAIN.path();
    WS_IO_SERVER.layer()
}

pub fn shutdown() {
    APP_SHUTDOWN_TOKEN.cancel();
}

async fn start_server(cancel_token: CancellationToken) -> Result<()> {
    let Some(config) = APP_CONFIG.get() else {
        bail!("Failed to get app config");
    };

    let server_addr = format!("{}:{}", &config.host, &config.port);
    tracing::info!("Server: starting on {}", &server_addr);

    let app = Router::new().layer(init_namespaces_and_get_tower_layer());
    let listener = TcpListener::bind(server_addr).await?;

    tracing::info!("Server: listening on {}", &listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(async move { cancel_token.cancelled().await })
        .await?;

    tracing::info!("Server: stopped");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing_with_layer(
        make_tracing_fmt_layer_with_local_time()?
            .with_target(false)
            .with_filter(LevelFilter::INFO),
    )?;

    // Parse cli and merge config
    let cli = Cli::parse();
    let cli_config_layer = AppConfigLayer {
        auth_key: cli.auth_key,
        host: cli.host,
        port: cli.port,
    };

    let mut config_builder = AppConfig::builder().preloaded(cli_config_layer);

    if let Some(path) = cli.config {
        config_builder = config_builder.file(path);
    }

    let config = config_builder
        .load()
        .map_err(|e| anyhow!("Configuration load failed: {}", e))?;

    // Set config
    APP_CONFIG
        .set(config)
        .map_err(|_| anyhow!("Failed to set app config"))?;

    // Run server and register signal handler
    APP_TASK_MANAGER.spawn_with_token(move |token| async {
        if start_server(token).await.is_err() {
            shutdown();
        }
    });

    APP_TASK_MANAGER.spawn_with_token(async |token| {
        select! {
            _ = token.cancelled() => {}
            _ = wait_for_shutdown_signal() => {}
        }

        shutdown();
    });

    // Wait for app shutdown
    APP_SHUTDOWN_TOKEN.cancelled().await;
    tracing::info!("Shutting down...");

    WS_IO_SERVER.shutdown().await;
    APP_TASK_MANAGER.cancel_and_join_existing().await;

    tracing::info!("Shutdown complete");
    Ok(())
}
