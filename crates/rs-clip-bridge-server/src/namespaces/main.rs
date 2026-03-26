use std::sync::{
    Arc,
    LazyLock,
};

use anyhow::{
    Context,
    Result,
    bail,
};
use wsio_server::{
    connection::WsIoServerConnection,
    namespace::WsIoServerNamespace,
};

use crate::{
    SERVER_CONFIG,
    WS_IO_SERVER,
    types::ClipboardEventData,
};

// ================================================================================================
// Channel ID Extension
// ================================================================================================

/// Extension stored on each connection to identify its channel
pub struct ChannelId(pub String);

/// Extract channel ID from a connection's extensions
pub fn extract_channel_id_from_connection(connection: &Arc<WsIoServerConnection>) -> String {
    connection.extensions().get::<ChannelId>().unwrap().0.clone()
}

// ================================================================================================
// Namespace Definition
// ================================================================================================

pub static MAIN: LazyLock<Arc<WsIoServerNamespace>> = LazyLock::new(|| {
    WS_IO_SERVER
        .new_namespace_builder("/")
        .on_connect(on_connect)
        .on_ready(on_ready)
        .with_init_response(init_response_handler)
        .register()
        .unwrap()
});

// ================================================================================================
// Event Handlers
// ================================================================================================

async fn init_response_handler(
    connection: Arc<WsIoServerConnection>,
    data: Option<(Option<String>, String)>,
) -> Result<()> {
    let (auth_key, channel_id) = data.context("Invalid init response data")?;
    let config = SERVER_CONFIG.get().context("Server config not initialized")?;

    if config.auth_keys.is_empty() || !config.auth_keys.iter().any(|k| Some(k) == auth_key.as_ref()) {
        let _ = connection.disconnect().await;
        bail!("Unauthorized: Auth key mismatch");
    }

    // Set channel id
    connection.extensions().insert(ChannelId(channel_id.clone()));

    tracing::info!(channel_id = %channel_id, "Client authorized");

    Ok(())
}

async fn on_connect(connection: Arc<WsIoServerConnection>) -> Result<()> {
    tracing::info!("Connection established: id={}", connection.id());

    // Register events
    connection.on("event", on_event);

    Ok(())
}

async fn on_ready(connection: Arc<WsIoServerConnection>) -> Result<()> {
    let channel_id = extract_channel_id_from_connection(&connection);
    connection.join([channel_id.clone()]);

    tracing::info!(channel_id = %channel_id, "Joined channel");

    Ok(())
}

/// Handle incoming clipboard event from a client
async fn on_event(connection: Arc<WsIoServerConnection>, data: Arc<ClipboardEventData>) -> Result<()> {
    let channel_id = extract_channel_id_from_connection(&connection);
    let _ = connection.to([&channel_id]).emit("event", Some(data.as_ref())).await;
    Ok(())
}
