use std::sync::{
    Arc,
    LazyLock,
};

use anyhow::{
    Result,
    bail,
};
use wsio_server::{
    connection::WsIoServerConnection,
    namespace::WsIoServerNamespace,
};

mod handlers;

use super::super::WS_IO_SERVER;
use crate::APP_CONFIG;

// Constants/Statics
pub static MAIN: LazyLock<Arc<WsIoServerNamespace>> = LazyLock::new(|| {
    WS_IO_SERVER
        .new_namespace_builder("/")
        .on_connect(on_connect)
        .with_init_response(init_response_handler)
        .register()
        .unwrap()
});

// Functions
async fn init_response_handler(connection: Arc<WsIoServerConnection>, auth_key: Option<String>) -> Result<()> {
    let Some(config) = APP_CONFIG.get() else {
        bail!("Failed to get app config");
    };

    let Some(config_auth_key) = config.auth_key.clone() else {
        return Ok(());
    };

    if let Some(auth_key) = auth_key
        && auth_key == config_auth_key
    {
        return Ok(());
    }

    connection.disconnect().await;
    bail!("Unauthorized");
}

async fn on_connect(connection: Arc<WsIoServerConnection>) -> Result<()> {
    tracing::info!("Connected with connection id {}", connection.id());

    // Register events

    Ok(())
}
