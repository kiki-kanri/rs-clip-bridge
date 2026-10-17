use tokio::sync::RwLock;

/// The last serialized clipboard content used to suppress repeated outgoing events.
///
/// Updated before emitting an outgoing event or writing received content to the local clipboard,
/// even if the subsequent operation fails.
pub(crate) static LAST_CONTENT_BYTES: RwLock<Vec<u8>> = RwLock::const_new(Vec::new());
