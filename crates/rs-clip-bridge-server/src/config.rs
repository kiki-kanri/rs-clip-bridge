use confique::Config;

#[derive(Clone, Config, Debug)]
pub(crate) struct ServerConfig {
    /// Accepted authentication keys. Defaults to an empty list, which allows unauthenticated access.
    #[config(default = [])]
    pub auth_keys: Vec<String>,

    /// Server bind address.
    pub host: String,

    /// Server TCP port.
    pub port: u16,
}

#[cfg(test)]
mod tests {
    use self::confique_server_config_layer::ServerConfigLayer;
    use super::*;

    #[test]
    fn server_config_default_port() {
        let config = ServerConfig::builder()
            .preloaded(ServerConfigLayer {
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
            .preloaded(ServerConfigLayer {
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
            .preloaded(ServerConfigLayer {
                auth_keys: None,
                host: Some("127.0.0.1".into()),
                port: Some(8000),
            })
            .load()
            .unwrap();

        assert_eq!(config.auth_keys, [] as [std::string::String; 0]);
    }
}
