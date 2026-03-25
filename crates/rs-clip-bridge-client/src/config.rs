use confique::Config;

#[derive(Clone, Config)]
pub struct ClientConfig {
    /// Authentication key for server access and grouping (optional)
    pub auth_key: Option<String>,

    /// Channel ID for clipboard isolation (required)
    pub channel_id: String,

    /// Linux-specific: X11 Display name (e.g., ":0")
    #[cfg(unix)]
    pub display: Option<String>,

    /// Encryption key for E2E encryption (32 bytes / 64 hex chars, required)
    pub encrypt_key: String,

    /// Server connection URL (e.g., ws://localhost:8080)
    pub server_url: String,
}
