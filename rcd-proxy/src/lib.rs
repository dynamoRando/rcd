use config::Config;
use log::info;
use rcd_enum::database_type::DatabaseType;
use simple_logger::SimpleLogger;
use std::{env, path::Path};
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
    pub grpc_client_addr_port: String,
    pub grpc_db_addr_port: String,
    pub http_ip: String,
    pub http_port: usize,
    pub root_dir: String,
    pub database_type: DatabaseType,
    pub database_name: String,
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
            .expect("Could not find {SETTINGS} file in {dir}");

        let db_name = settings
            .get_string("backing_database_name")
            .unwrap_or(PROXY_DB.to_string());
        let client_addr = settings
            .get_string("client_addr_port")
            .unwrap_or("127.0.0.1:50051".to_string());
        let db_addr = settings
            .get_string("db_addr_port")
            .unwrap_or("127.0.0.1:50052".to_string());
        let http_addr = settings
            .get_string("http_addr")
            .unwrap_or("127.0.0.1".to_string());
        let http_port = settings
            .get_string("http_port")
            .unwrap_or("50055".to_string());
        let db_type = settings
            .get_string("database_type")
            .unwrap_or("1".to_string());

        let settings = RcdProxySettings {
            use_grpc: true,
            use_http: true,
            grpc_client_addr_port: client_addr,
            grpc_db_addr_port: db_addr,
            http_ip: http_addr,
            http_port: http_port.parse().unwrap(),
            root_dir: dir,
            database_type: DatabaseType::from_u32(db_type.parse().unwrap()),
            database_name: db_name,
        };

        let service = RcdProxy { settings };

        Ok(service)
    }

    pub fn output_settings(&self) {
        let settings = &self.settings.clone();
        info!("{settings:?}");
    }
}

#[test]
fn test_get_settings() {
    SimpleLogger::new().env().init().unwrap();

    let cwd = env::current_dir().unwrap().to_str().unwrap().to_string();
    let proxy = RcdProxy::get_proxy_from_config(cwd).unwrap();
    proxy.output_settings();
}
