use config::Config;
use rcd_enum::database_type::DatabaseType;
use std::path::Path;
use thiserror::Error;

const SETTINGS: &str = "Settings.toml";
const PROXY_DB: &str = "Proxy.db";

#[derive(Error, Debug)]
pub enum RcdProxyErr {
    #[error("Could not find Settings.toml in dir: `{0}`")]
    SettingsNotFound(String),
}

#[derive(Debug, Clone)]
pub struct RcdProxy {
    settings: RcdProxySettings,
}

#[derive(Debug, Clone)]
pub struct RcdProxySettings {
    pub use_grpc: bool,
    pub use_http: bool,
    pub grpc_ip: String,
    pub http_ip: String,
    pub client_port: usize,
    pub db_port: usize,
    pub http_port: usize,
    pub root_dir: String,
    pub database_type: DatabaseType,
}

impl RcdProxy {
    /// reads the specified directory's Settings.toml and returns
    /// a new instance of a RcdProxy
    pub fn get_proxy_from_config(dir: String) -> Result<Self, RcdProxyErr> {
        let config = Path::new(&dir).join(SETTINGS);

        if !Path::exists(&config) {
            return Err(RcdProxyErr::SettingsNotFound(dir));
        }

        let config = config.to_str().unwrap();

        let settings = Config::builder()
            .add_source(config::File::with_name(config))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .expect("Could not find config file in {dir}");

        todo!();
    }
}
