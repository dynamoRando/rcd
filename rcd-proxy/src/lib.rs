use config::Config;
use log::{error, info};
use proxy_db::ProxyDb;
use rcd_enum::database_type::DatabaseType;
use simple_logger::SimpleLogger;
use std::{env, path::Path};
use thiserror::Error;

const SETTINGS: &str = "Settings.toml";
const PROXY_DB: &str = "Proxy.db";

mod proxy_db;
mod proxy_db_sqlite;
mod proxy_grpc;
mod sql_text;

#[derive(Error, Debug)]
pub enum RcdProxyErr {
    #[error("Could not find Settings.toml in dir: `{0}`")]
    SettingsNotFound(String),
    #[error("User already exists: `{0}`")]
    UserAlreadyExists(String),
}

#[derive(Debug, Clone)]
pub struct RcdProxy {
    settings: RcdProxySettings,
    db: ProxyDb,
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

        let result_db_name = settings.get_string("backing_database_name");

        let db_name: String = match result_db_name {
            Ok(name) => name,
            Err(_) => {
                error!("missing setting: 'backing_database_name', using default {PROXY_DB}");
                PROXY_DB.to_string()
            }
        };

        let result_client_addr = settings.get_string("client_addr_port");

        let client_addr = match result_client_addr {
            Ok(addr) => addr,
            Err(_) => {
                error!("missing setting: 'client_addr_port', using default 127.0.0.1:50051");
                "127.0.0.1:50051".to_string()
            }
        };

        let result_db_addr = settings.get_string("db_addr_port");

        let db_addr = match result_db_addr {
            Ok(addr) => addr,
            Err(_) => {
                error!("missing setting: 'db_addr_port', using default 127.0.0.1:50052");
                "127.0.0.1:50052".to_string()
            }
        };

        let result_http_addr = settings.get_string("http_addr");

        let http_addr = match result_http_addr {
            Ok(addr) => addr,
            Err(_) => {
                error!("missing setting: 'http_addr', using default 127.0.0.1");
                "127.0.0.1".to_string()
            }
        };

        let result_http_port = settings.get_string("http_port");

        let http_port = match result_http_port {
            Ok(port) => port,
            Err(_) => {
                error!("missing setting: 'http_port',  using default 50055");
                "50055".to_string()
            }
        };

        let result_db_type = settings.get_string("database_type");

        let db_type = match result_db_type {
            Ok(x) => x,
            Err(_) => {
                error!("missing setting: 'database_type', using default 1 ");
                "1".to_string()
            }
        };

        let db_type = DatabaseType::from_u32(db_type.parse().unwrap());

        let settings = RcdProxySettings {
            use_grpc: true,
            use_http: true,
            grpc_client_addr_port: client_addr,
            grpc_db_addr_port: db_addr,
            http_ip: http_addr,
            http_port: http_port.parse().unwrap(),
            root_dir: dir.clone(),
            database_type: db_type,
            database_name: db_name.clone(),
        };

        let db = match db_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => ProxyDb::new_with_sqlite(db_name.clone(), dir.clone()),
            DatabaseType::Mysql => todo!(),
            DatabaseType::Postgres => todo!(),
            DatabaseType::Sqlserver => todo!(),
        };

        let service = RcdProxy { settings, db };

        Ok(service)
    }

    pub fn output_settings(&self) {
        let settings = &self.settings.clone();
        info!("{settings:?}");
    }

    /// initalizes the backing database
    pub fn start(&self) {
        self.db.config();

        todo!();
    }

    /// starts the grpc proxy
    pub fn start_grpc(&self) {
        todo!();
    }

    /// checks to see if the specified user name already exists
    /// if not, it will save the un, hash the pw, and init
    /// a new `rcd` directory for the account and init the `rcd` instance
    pub fn register_user(un: String, pw: String) -> Result<(), RcdProxyErr> {
        todo!();
    }
}

#[test]
fn test_output_settings() {
    SimpleLogger::new().env().init().unwrap();

    let cwd = env::current_dir().unwrap().to_str().unwrap().to_string();
    let proxy = RcdProxy::get_proxy_from_config(cwd).unwrap();
    proxy.output_settings();
}
