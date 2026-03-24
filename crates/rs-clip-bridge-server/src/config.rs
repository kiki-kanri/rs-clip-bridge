use confique::Config;

#[derive(Clone, Config, Debug)]
pub struct AppConfig {
    pub auth_key: Option<String>,
    pub host: String,
    pub port: u16,
}
