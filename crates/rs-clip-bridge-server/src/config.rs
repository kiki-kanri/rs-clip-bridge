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
