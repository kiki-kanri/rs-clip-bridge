use std::sync::Arc;

use anyhow::Result;
use wsio_server::connection::WsIoServerConnection;

use super::internals::extract_channel_id_from_connection;
use crate::clipboard::ClipboardEventData;

// Functions
pub async fn on_event(connection: Arc<WsIoServerConnection>, data: Arc<ClipboardEventData>) -> Result<()> {
    let channel_id = extract_channel_id_from_connection(&connection);
    let _ = connection.to([&channel_id]).emit("event", Some(data.as_ref())).await;
    Ok(())
}
