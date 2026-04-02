use confique::Config;

#[derive(Clone, Config, Debug)]
pub struct ServerConfig {
    /// List of auth keys
    #[config(default = [])]
    pub auth_keys: Vec<String>,

    /// Server host
    pub host: String,

    /// Server port
    pub port: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_config_default_port() {
        let config = ServerConfig::builder()
            .preloaded(confique_server_config_layer::ServerConfigLayer {
                auth_keys: None,
                host: Some("127.0.0.1".into()),
                port: Some(8000),
            })
            .load()
            .unwrap();
        assert_eq!(config.port, 8000);
    }

    #[test]
    fn server_config_default_host() {
        let config = ServerConfig::builder()
            .preloaded(confique_server_config_layer::ServerConfigLayer {
                auth_keys: None,
                host: Some("0.0.0.0".into()),
                port: Some(8000),
            })
            .load()
            .unwrap();
        assert_eq!(config.host, "0.0.0.0");
    }

    #[test]
    fn server_config_default_auth_keys_empty() {
        let config = ServerConfig::builder()
            .preloaded(confique_server_config_layer::ServerConfigLayer {
                auth_keys: None,
                host: Some("127.0.0.1".into()),
                port: Some(8000),
            })
            .load()
            .unwrap();
        assert!(config.auth_keys.is_empty());
    }
}
