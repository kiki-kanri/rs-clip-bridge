use std::{
    fs::write,
    path::PathBuf,
};

use clap::{
    Parser,
    Subcommand,
};
use confique::toml::template;

use crate::config::ClientConfig;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "rs-clip-bridge-client: Cross-platform clipboard sync client with E2E encryption.",
    long_about = "A client that monitors local clipboard changes and synchronizes them with a remote server via WebSockets. All clipboard data is encrypted using ChaCha20-Poly1305."
)]
pub struct Cli {
    /// Authentication key for server access (optional)
    #[arg(short, long, env = "RS_CLIP_AUTH_KEY")]
    pub auth_key: Option<String>,

    /// Channel ID for clipboard isolation (required)
    #[arg(long, env = "RS_CLIP_CHANNEL_ID")]
    pub channel_id: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Path to the configuration file (TOML format)
    #[arg(short, long, value_name = "FILE", env = "RS_CLIP_CLIENT_CONFIG")]
    pub config: Option<PathBuf>,

    /// Linux/X11 display name (e.g., :0). Defaults to $DISPLAY if empty
    #[cfg(unix)]
    #[arg(short = 'D', long, value_name = "NAME", env = "RS_CLIP_DISPLAY")]
    pub display: Option<String>,

    /// Encryption key for E2E encryption (64 hex chars / 32 bytes, required)
    #[arg(short = 'K', long, env = "RS_CLIP_ENCRYPT_KEY")]
    pub encrypt_key: Option<String>,

    /// Maximum image size in bytes to sync (e.g., 5242880 for 5 MB, default: 10485760 for 10 MB)
    #[arg(long, env = "RS_CLIP_MAX_IMAGE_SIZE_BYTES")]
    pub max_image_size_bytes: Option<usize>,

    /// Minimum size in bytes to trigger compression (default: 1024)
    #[arg(long, env = "RS_CLIP_MIN_COMPRESS_SIZE_BYTES")]
    pub min_compress_size_bytes: Option<usize>,

    /// WebSocket server URL (e.g., ws://127.0.0.1:8080 or wss://example.com)
    #[arg(short, long, value_name = "URL", env = "RS_CLIP_SERVER_URL")]
    pub server_url: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a configuration file template
    GenerateConfigTemplate {
        /// Output path (default: stdout)
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
}

pub fn run_generate_config_template(output: Option<PathBuf>) {
    let content = template::<ClientConfig>(Default::default());
    match output {
        Some(path) => write(&path, &content).unwrap(),
        None => print!("{}", content),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_parse_required_args() {
        let cli = Cli::try_parse_from([
            "rs-clip-bridge-client",
            "--server-url",
            "ws://localhost:8080",
            "--channel-id",
            "test-channel",
            "--encrypt-key",
            &"0".repeat(64),
        ])
        .unwrap();
        assert_eq!(cli.channel_id, Some("test-channel".into()));
        assert_eq!(cli.server_url, Some("ws://localhost:8080".into()));
        assert_eq!(cli.encrypt_key, Some("0".repeat(64)));
    }

    #[test]
    fn cli_parse_all_optional_args() {
        let cli = Cli::try_parse_from([
            "rs-clip-bridge-client",
            "--server-url",
            "ws://localhost:8080",
            "--channel-id",
            "my-channel",
            "--encrypt-key",
            &"a".repeat(64),
            "--auth-key",
            "my-secret-key",
            "--max-image-size-bytes",
            "5242880",
            "--min-compress-size-bytes",
            "2048",
        ])
        .unwrap();
        assert_eq!(cli.auth_key, Some("my-secret-key".into()));
        assert_eq!(cli.max_image_size_bytes, Some(5242880));
        assert_eq!(cli.min_compress_size_bytes, Some(2048));
    }

    #[test]
    fn cli_generate_config_template_command() {
        let cli = Cli::try_parse_from(["rs-clip-bridge-client", "generate-config-template"]).unwrap();
        assert!(matches!(
            cli.command,
            Some(Commands::GenerateConfigTemplate { output: None })
        ));
    }
}
