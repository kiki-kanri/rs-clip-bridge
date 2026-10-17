use confique::Config;

#[derive(Clone, Config)]
pub(crate) struct ClientConfig {
    /// Optional authentication key for server access.
    pub auth_key: Option<String>,

    /// Required channel ID for clipboard isolation.
    pub channel_id: String,

    /// X11 display name on Unix, for example `:0`. Leaves `DISPLAY` unchanged when absent.
    #[cfg(unix)]
    pub display: Option<String>,

    /// Required end-to-end encryption key as 64 hexadecimal characters (32 bytes).
    pub encrypt_key: String,

    /// Maximum outgoing image size in uncompressed bytes. Defaults to 10485760 bytes (10 MiB).
    #[config(default = 10_485_760)]
    pub max_image_size_bytes: usize,

    /// Minimum serialized clipboard size in bytes to attempt compression. Defaults to 1024 bytes.
    #[config(default = 1024)]
    pub min_compress_size_bytes: usize,

    /// WebSocket server URL, for example `ws://localhost:8080`.
    pub server_url: String,
}
