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

    /// Maximum image size in bytes to sync (e.g., 5242880 for 5 MB, default: 10485760 for 10 MB)
    #[config(default = 10485760)]
    pub max_image_size_bytes: usize,

    /// Minimum size in bytes to trigger compression (default: 1 KB)
    #[config(default = 1024)]
    pub min_compress_size_bytes: usize,

    /// Server connection URL (e.g., ws://localhost:8080)
    pub server_url: String,
}
