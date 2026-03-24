use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "rs-clip-bridge: A blazingly fast, cross-platform clipboard synchronizer using WebSockets.",
    long_about = "A secure tool to synchronize clipboard content across multiple devices. \
                  Supports both text and binary data, with server-side grouping via auth keys."
)]
pub struct Cli {
    /// Authentication key for server access and clipboard grouping.
    /// Can be set via the RS_CLIP_AUTH_KEY environment variable.
    #[arg(short, long, env = "RS_CLIP_AUTH_KEY")]
    pub auth_key: Option<String>,

    /// Path to the configuration file (TOML format).
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Server host address (e.g., 127.0.0.1 or example.com).
    /// Can be set via the RS_CLIP_SERVER_HOST environment variable.
    #[arg(short = 'H', long, env = "RS_CLIP_SERVER_HOST", default_value = "127.0.0.1")]
    pub host: Option<String>,

    /// Server port number.
    /// Can be set via the RS_CLIP_SERVER_PORT environment variable.
    #[arg(short, long, env = "RS_CLIP_SERVER_PORT", default_value = "8000")]
    pub port: Option<u16>,
}
