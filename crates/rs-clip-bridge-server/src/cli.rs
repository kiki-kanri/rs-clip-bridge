use std::{
    fs::write,
    path::PathBuf,
};

use clap::{
    Parser,
    Subcommand,
};
use confique::toml::template;

use crate::config::ServerConfig;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "rs-clip-bridge: A blazingly fast, cross-platform clipboard synchronizer using WebSockets.",
    long_about = "A secure tool to synchronize clipboard content across multiple devices. \
                  Supports both text and binary data, with server-side grouping via auth keys."
)]
pub struct Cli {
    /// Authentication keys for server access (multiple allowed).
    /// Can be set via the RS_CLIP_AUTH_KEYS environment variable.
    #[arg(short, long, value_delimiter = ',', env = "RS_CLIP_AUTH_KEYS")]
    pub auth_keys: Option<Vec<String>>,

    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Path to the configuration file (TOML format).
    #[arg(short, long, value_name = "FILE", env = "RS_CLIP_SERVER_CONFIG")]
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
    let content = template::<ServerConfig>(Default::default());
    match output {
        Some(path) => write(&path, &content).unwrap(),
        None => print!("{}", content),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_default_host_and_port() {
        let cli = Cli::try_parse_from(["rs-clip-bridge-server"]).unwrap();
        assert_eq!(cli.host, Some("127.0.0.1".into()));
        assert_eq!(cli.port, Some(8000));
    }

    #[test]
    fn cli_override_host_and_port() {
        let cli = Cli::try_parse_from(["rs-clip-bridge-server", "--host", "0.0.0.0", "--port", "9000"]).unwrap();
        assert_eq!(cli.host, Some("0.0.0.0".into()));
        assert_eq!(cli.port, Some(9000));
    }

    #[test]
    fn cli_with_auth_keys() {
        let cli = Cli::try_parse_from(["rs-clip-bridge-server", "--auth-keys", "key1,key2,key3"]).unwrap();
        assert_eq!(cli.auth_keys, Some(vec!["key1".into(), "key2".into(), "key3".into()]));
    }

    #[test]
    fn cli_generate_config_template_command() {
        let cli = Cli::try_parse_from(["rs-clip-bridge-server", "generate-config-template"]).unwrap();
        assert!(matches!(
            cli.command,
            Some(Commands::GenerateConfigTemplate { output: None })
        ));
    }
}
