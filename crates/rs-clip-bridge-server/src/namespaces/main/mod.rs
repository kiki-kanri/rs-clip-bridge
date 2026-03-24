use std::sync::{
    Arc,
    LazyLock,
};

use anyhow::{
    Result,
    anyhow,
    bail,
};
use wsio_server::{
    connection::WsIoServerConnection,
    namespace::WsIoServerNamespace,
};

mod handlers;
mod internals;

use self::internals::{
    ChannelId,
    extract_channel_id_from_connection,
};
use super::super::WS_IO_SERVER;
use crate::SERVER_CONFIG;

// Constants/Statics
pub static MAIN: LazyLock<Arc<WsIoServerNamespace>> = LazyLock::new(|| {
    WS_IO_SERVER
        .new_namespace_builder("/")
        .on_connect(on_connect)
        .on_ready(on_ready)
        .with_init_response(init_response_handler)
        .register()
        .unwrap()
});

// Functions
async fn init_response_handler(
    connection: Arc<WsIoServerConnection>,
    data: Option<(Option<String>, String)>,
) -> Result<()> {
    let (auth_key, channel_id) = data.ok_or_else(|| anyhow!("Invalid init response data"))?;
    let config = SERVER_CONFIG
        .get()
        .ok_or_else(|| anyhow!("Server config not initialized"))?;

    if let Some(ref required_key) = config.auth_key {
        if auth_key.as_ref() != Some(required_key) {
            let _ = connection.disconnect().await;
            bail!("Unauthorized: Auth key mismatch");
        }
    }

    // Set channel id
    connection.extensions().insert(ChannelId(channel_id));

    Ok(())
}

async fn on_connect(connection: Arc<WsIoServerConnection>) -> Result<()> {
    tracing::info!("Connected with connection id {}", connection.id());

    // Register events
    connection.on("event", handlers::on_event);

    Ok(())
}

async fn on_ready(connection: Arc<WsIoServerConnection>) -> Result<()> {
    connection.join([extract_channel_id_from_connection(&connection)]);

    Ok(())
}
