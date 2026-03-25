use tokio::sync::RwLock;

/// Stores the last clipboard content to prevent circular write conflicts.
/// When we receive a clipboard change from the server, we update this.
/// When we send a clipboard change to the server, we check this first.
pub static LAST_CONTENT: RwLock<String> = RwLock::const_new(String::new());
