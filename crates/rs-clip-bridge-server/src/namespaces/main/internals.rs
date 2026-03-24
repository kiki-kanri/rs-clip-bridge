use std::sync::Arc;

use wsio_server::connection::WsIoServerConnection;

// Structs
pub struct ChannelId(pub String);

// Functions
pub fn extract_channel_id_from_connection(connection: &Arc<WsIoServerConnection>) -> String {
    connection.extensions().get::<ChannelId>().unwrap().0.clone()
}
