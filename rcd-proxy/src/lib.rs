use std::path::Path;

use config::Config;
use log::{error, info};
use proxy_db::ProxyDb;
use rcd_enum::database_type::DatabaseType;
#[cfg(test)]
use simple_logger::SimpleLogger;
use thiserror::Error;

const SETTINGS: &str = "Settings.toml";
const PROXY_DB: &str = "Proxy.db";

mod proxy_db;
mod proxy_db_sqlite;
mod proxy_grpc;
mod sql_text;

#[derive(Error, Debug, PartialEq)]
pub enum RcdProxyErr {
    #[error("Could not find Settings.toml in dir: `{0}`")]
    SettingsNotFound(String),
    #[error("User already exists: `{0}`")]
    UserAlreadyExists(String),
    #[error("Db Error: `{0}`")]
    DbError(String),
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
    pub fn get_proxy_from_config_with_dir(
        config_dir: &str,
        root_dir: &str,
    ) -> Result<Self, RcdProxyErr> {
        let result_settings = Self::get_settings(config_dir);

        match result_settings {
            Ok(mut settings) => {
                settings.root_dir = root_dir.to_string();

                let db_type = settings.database_type;
                let db_name = settings.database_name.clone();

                let db = match db_type {
                    DatabaseType::Unknown => todo!(),
                    DatabaseType::Sqlite => ProxyDb::new_with_sqlite(db_name, root_dir.to_string()),
                    DatabaseType::Mysql => todo!(),
                    DatabaseType::Postgres => todo!(),
                    DatabaseType::Sqlserver => todo!(),
                };

                let service = RcdProxy { settings, db };

                return Ok(service);
            }
            Err(e) => return Err(e),
        };
    }

    /// reads the specified directory's Settings.toml and returns
    /// a new instance of a RcdProxy
    pub fn get_proxy_from_config(dir: &str) -> Result<Self, RcdProxyErr> {
        let result_settings = Self::get_settings(dir);
        match result_settings {
            Ok(settings) => {
                let db_type = settings.database_type.clone();
                let db_name = settings.database_name.clone();

                let db = match db_type {
                    DatabaseType::Unknown => todo!(),
                    DatabaseType::Sqlite => ProxyDb::new_with_sqlite(db_name, dir.to_string()),
                    DatabaseType::Mysql => todo!(),
                    DatabaseType::Postgres => todo!(),
                    DatabaseType::Sqlserver => todo!(),
                };

                let service = RcdProxy { settings, db };

                return Ok(service);
            }
            Err(e) => return Err(e),
        };
    }

    fn get_settings(dir: &str) -> Result<RcdProxySettings, RcdProxyErr> {
        let config = Path::new(&dir).join(SETTINGS);

        if !Path::exists(&config) {
            return Err(RcdProxyErr::SettingsNotFound(dir.to_string()));
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

        Ok(RcdProxySettings {
            use_grpc: true,
            use_http: true,
            grpc_client_addr_port: client_addr,
            grpc_db_addr_port: db_addr,
            http_ip: http_addr,
            http_port: http_port.parse().unwrap(),
            root_dir: dir.to_string(),
            database_type: db_type,
            database_name: db_name.clone(),
        })
    }

    pub fn output_settings(&self) {
        let settings = &self.settings.clone();
        info!("{settings:?}");
    }

    /// initalizes the backing database
    pub fn start(&self) {
        self.db.config();
    }

    /// starts the grpc proxy
    pub fn start_grpc(&self) {
        todo!();
    }

    /// checks to see if the specified user name already exists
    /// if not, it will save the un, hash the pw, and init
    /// a new `rcd` directory for the account and init the `rcd` instance
    pub fn register_user(&self, un: &str, pw: &str) -> Result<(), RcdProxyErr> {
        use rcd_common::crypt::hash;
        let hash = hash(pw);
        self.db.register_user(un, &hash.0)
    }
}

#[test]
fn test_output_settings() {
    use ignore_result::Ignore;
    use std::env;
    SimpleLogger::new().env().init().ignore();

    let cwd = env::current_dir().unwrap().to_str().unwrap().to_string();
    let proxy = RcdProxy::get_proxy_from_config(&cwd).unwrap();
    proxy.output_settings();
}

#[test]
pub fn test_new_with_sqlite() {
    use ignore_result::Ignore;
    use rcd_test_harness::get_test_temp_dir;
    use std::env;

    SimpleLogger::new().env().init().ignore();

    let root_dir = get_test_temp_dir("rcd-proxy-db-unit-test-new");
    let config_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let proxy = RcdProxy::get_proxy_from_config_with_dir(&config_dir, &root_dir).unwrap();
    proxy.start();
    let result = proxy.register_user("test", "1234");

    assert_eq!(result, Ok(()));
}

#[test]
pub fn test_register_twice() {
    use ignore_result::Ignore;
    use rcd_test_harness::get_test_temp_dir;
    use std::env;

    SimpleLogger::new().env().init().ignore();

    let root_dir = get_test_temp_dir("rcd-proxy-db-unit-test-register");
    let config_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let proxy = RcdProxy::get_proxy_from_config_with_dir(&config_dir, &root_dir).unwrap();
    proxy.start();
    let first_result = proxy.register_user("test", "1234");
    let second_result = proxy.register_user("test", "1234");

    assert_eq!(first_result, Ok(()));
    assert_eq!(
        second_result,
        Err(RcdProxyErr::UserAlreadyExists(
            "User 'test' already exists".to_string()
        ))
    );
}
