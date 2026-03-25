use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "rs-clip-bridge-client: Cross-platform clipboard sync client.",
    long_about = "A client that monitors local clipboard changes and synchronizes them with a remote server via WebSockets."
)]
pub struct Cli {
    /// Authentication key for server access
    #[arg(short, long, env = "RS_CLIP_AUTH_KEY")]
    pub auth_key: Option<String>,

    /// Channel ID for grouping devices (defaults to "default" if not set)
    #[arg(short, long, env = "RS_CLIP_CHANNEL_ID")]
    pub channel_id: Option<String>,

    /// Path to the configuration file (TOML format)
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Linux/X11 display name (e.g., :0). Defaults to $DISPLAY if empty
    #[cfg(unix)]
    #[arg(short = 'D', long, value_name = "NAME", env = "RS_CLIP_DISPLAY")]
    pub display: Option<String>,

    /// WebSocket server URL (e.g., ws://127.0.0.1:8080 or wss://example.com)
    #[arg(short, long, value_name = "URL", env = "RS_CLIP_SERVER_URL")]
    pub server_url: Option<String>,
}
