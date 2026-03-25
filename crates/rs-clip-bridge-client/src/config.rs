use confique::Config;

#[derive(Clone, Config)]
pub struct ClientConfig {
    /// Authentication key for access and grouping
    pub auth_key: Option<String>,

    /// Channel ID for clipboard isolation (required)
    pub channel_id: String,

    /// Linux-specific: X11 Display name (e.g., ":0")
    #[cfg(unix)]
    pub display: Option<String>,

    /// Server connection URL (e.g., ws://localhost:8080)
    pub server_url: String,
}
