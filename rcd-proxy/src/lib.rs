use std::{fs, path::Path};

use crate::proxy_grpc::{ProxyClientGrpc, ProxyDbGrpc};
use chrono::{DateTime, Utc};
use config::Config;
use tracing::{warn, instrument};
#[allow(unused_imports)]
use tracing::{debug, error, info, trace};
use proxy_db::ProxyDb;
use rcd_common::{crypt, rcd_settings::RcdSettings};
use rcd_core::{auth, rcd::Rcd, rcd_data::RcdData};
use rcd_enum::database_type::DatabaseType;
use rcd_messages::proxy::server_messages::AuthForTokenReply;
use rcdproto::rcdp::{data_service_server::DataServiceServer, sql_client_server::SqlClientServer};
use rcdx::rcd_service::RcdService;
#[cfg(test)]
use simple_logger::SimpleLogger;
use thiserror::Error;
use tonic::transport::Server;
use triggered::{Listener, Trigger};
use user_info::UserInfo;
use uuid::Uuid;
use stdext::function_name;

const SETTINGS: &str = "Settings.toml";
const PROXY_DB: &str = "Proxy.db";

mod grpc_client;
mod proxy_db;
mod proxy_db_sqlite;
mod proxy_grpc;
pub mod proxy_server;
mod sql_text;
mod user_info;

#[derive(Error, Debug, PartialEq)]
pub enum RcdProxyErr {
    #[error("Could not find Settings.toml in dir: `{0}`")]
    SettingsNotFound(String),
    #[error("User already exists: `{0}`")]
    UserAlreadyExists(String),
    #[error("Db Error: `{0}`")]
    DbError(String),
    #[error("Folder already exists: `{0}`")]
    FolderAlreadyExists(String),
    #[error("User `{0}` not found")]
    UserNotFound(String),
    #[error("Host Id `{0}` not found")]
    HostIdNotFound(String),
    #[error("No rows affected")]
    NoRowsAffected,
    #[error("User `{0}` folder not set")]
    UserFolderNotSet(String),
}

#[derive(Debug, Clone)]
pub struct RcdProxy {
    settings: RcdProxySettings,
    db: ProxyDb,
}

#[derive(Debug, Clone)]
struct GrpcServiceSettings {
    pub root_folder: String,
    pub database_name: String,
    pub addr_port: String,
    pub own_db_addr_port: String,
    pub proxy: RcdProxy,
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
    // for the webpage to talk to this instance
    pub proxy_http_addr: String,
    pub proxy_http_port: usize,
}

impl RcdProxy {
    pub fn get_proxy_with_config(settings: RcdProxySettings) -> Self {
        let db_type = settings.database_type;
        let db_name = settings.database_name.clone();

        let db = match db_type {
            DatabaseType::Unknown => todo!(),
            DatabaseType::Sqlite => {
                ProxyDb::new_with_sqlite(db_name, settings.root_dir.to_string())
            }
            DatabaseType::Mysql => todo!(),
            DatabaseType::Postgres => todo!(),
            DatabaseType::Sqlserver => todo!(),
        };

        RcdProxy { settings, db }
    }

    pub fn http_endpoint_addr(&self) -> String {
        self.settings.proxy_http_addr.clone()
    }

    pub fn http_endpoint_port(&self) -> u16 {
        self.settings.proxy_http_port as u16
    }

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

        debug!("[{}]: {config:?}", function_name!());

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

        let result_proxy_http_addr = settings.get_string("proxy_http_addr");

        let proxy_http_addr = match result_proxy_http_addr {
            Ok(addr) => addr,
            Err(_) => {
                error!("missing setting: 'http_addr', using default 127.0.0.1");
                "127.0.0.1".to_string()
            }
        };

        let result_proxy_http_port = settings.get_string("proxy_http_port");

        let proxy_http_port = match result_proxy_http_port {
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
            proxy_http_addr: proxy_http_addr,
            proxy_http_port: proxy_http_port.parse().unwrap(),
        })
    }

    pub fn output_settings(&self) {
        let settings = &self.settings.clone();
        info!("{settings:?}");
    }

    /// initalizes the backing database
    pub fn start(&self) {
        let version = env!("CARGO_PKG_VERSION");
        info!("rcd-proxy version: {}", version);

        self.db.config();
    }

    pub fn start_server() {
        todo!()
    }

    pub async fn start_grpc_data_at_addr(&self, addr: &str) {
        let (_, client_listener) = triggered::trigger();

        let client = ProxyDbGrpc::new(
            self.settings.root_dir.clone(),
            self.settings.database_name.clone(),
            addr.to_string(),
            self.clone(),
        );

        let service = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        info!("Data Proxy Service Starting At: {addr}");

        let addr = addr.parse().unwrap();

        Server::builder()
            .add_service(DataServiceServer::new(client))
            .add_service(service)
            .serve_with_shutdown(addr, client_listener)
            .await
            .unwrap();

        info!("Data Proxy Service Ending...");
    }

    async fn start_grpc_data_with_settings(settings: GrpcServiceSettings, listener: Listener) {
        let client = ProxyDbGrpc::new(
            settings.root_folder.clone(),
            settings.database_name.clone(),
            settings.addr_port.clone(),
            settings.proxy.clone(),
        );

        let addr = settings.addr_port.clone();

        let service = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        info!("Data Proxy Service Starting At: {addr}");

        let addr = addr.parse().unwrap();

        Server::builder()
            .add_service(DataServiceServer::new(client))
            .add_service(service)
            .serve_with_shutdown(addr, listener)
            .await
            .unwrap();

        info!("Data Proxy Service Ending...");
    }

    async fn start_grpc_client_with_settings(settings: GrpcServiceSettings, listener: Listener) {
        let client = ProxyClientGrpc::new(
            settings.root_folder.clone(),
            settings.database_name.clone(),
            settings.addr_port.clone(),
            settings.own_db_addr_port.to_string(),
            settings.proxy.clone(),
        );

        let addr = settings.addr_port.clone();

        let service = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        info!("Client Proxy Service Starting At: {addr}");

        let addr = addr.parse().unwrap();

        Server::builder()
            .add_service(SqlClientServer::new(client))
            .add_service(service)
            .serve_with_shutdown(addr, listener)
            .await
            .unwrap();

        info!("Cient Proxy Service Ending...");
    }

    pub async fn start_grpc_client_with_trigger(&self) -> Trigger {
        let (trigger, listener) = triggered::trigger();

        let settings = GrpcServiceSettings {
            root_folder: self.settings.root_dir.clone(),
            database_name: self.settings.database_name.clone(),
            addr_port: self.settings.grpc_client_addr_port.clone(),
            own_db_addr_port: self.settings.grpc_db_addr_port.clone(),
            proxy: self.clone(),
        };

        tokio::spawn(
            async move { Self::start_grpc_client_with_settings(settings, listener).await },
        );

        trigger
    }

    pub async fn start_grpc_data_with_trigger(&self) -> Trigger {
        let (trigger, listener) = triggered::trigger();

        let settings = GrpcServiceSettings {
            root_folder: self.settings.root_dir.clone(),
            database_name: self.settings.database_name.clone(),
            addr_port: self.settings.grpc_db_addr_port.clone(),
            own_db_addr_port: self.settings.grpc_db_addr_port.clone(),
            proxy: self.clone(),
        };

        tokio::spawn(async move { Self::start_grpc_data_with_settings(settings, listener).await });

        trigger
    }

    pub async fn start_grpc_client(&self) -> Trigger {
        self.start_grpc_client_with_trigger().await
    }

    pub async fn start_grpc_data(&self) -> Trigger {
        self.start_grpc_data_with_trigger().await
    }

    pub fn revoke_tokens_for_login(&self, un: &str) {
        self.db.revoke_tokens_for_login(un);
    }

    pub fn auth_for_token(&self, un: &str, pw: &str) -> Result<AuthForTokenReply, RcdProxyErr> {
        if self.verify_login(un, pw)? {
            if self.db.login_has_token(un) {
                self.db.revoke_tokens_for_login(un);
                debug!("revoked existing tokens for login {un}");
            }

            let token_data = self.create_token_for_login(un);
            let jwt = token_data.0;
            let expiration_utc = token_data.1.to_string();

            let u = self.db.get_user(un)?;

            return Ok(AuthForTokenReply {
                is_successful: true,
                expiration_utc: Some(expiration_utc),
                jwt: Some(jwt),
                id: u.id,
            });
        }

        Ok(AuthForTokenReply {
            is_successful: false,
            expiration_utc: None,
            jwt: None,
            id: None,
        })
    }

    fn create_token_for_login(&self, login: &str) -> (String, DateTime<Utc>) {
        let token_data = auth::create_jwt("rcd-proxy", login);
        self.db
            .save_token(login, &token_data.0, token_data.1)
            .unwrap();
        token_data
    }

    pub fn verify_login(&self, un: &str, pw: &str) -> Result<bool, RcdProxyErr> {
        if self.db.has_user(un) {
            let u = self.db.get_user(un)?;

            let mut padded = [0u8; 128];
            u.hash.iter().enumerate().for_each(|(i, val)| {
                padded[i] = *val;
            });

            return Ok(crypt::verify(padded, pw));
        }
        Ok(false)
    }

    pub fn verify_token(&self, jwt: &str) -> Result<bool, RcdProxyErr> {
        Ok(self.db.verify_token(jwt))
    }

    /// checks to see if the specified user name already exists
    /// if not, it will save the un, hash the pw, and init
    /// a new `rcd` directory for the account and init the `rcd` instance
    pub fn register_user(&self, un: &str, pw: &str) -> Result<(), RcdProxyErr> {
        use rcd_common::crypt::hash;
        let hash = hash(pw);
        self.db.register_user(un, &hash.0)
    }

    pub fn get_host_id_for_token(&self, token: &str) -> Result<Option<String>, RcdProxyErr> {
        if self.db.verify_token(token) {
            let u = self.db.get_user_with_token(token)?;
            return Ok(u.id);
        }

        return Ok(None);
    }

    pub fn get_host_id_for_user(&self, un: &str) -> Result<Option<String>, RcdProxyErr> {
        let u = self.db.get_user(un)?;
        return Ok(u.id);
    }

    /// sets up the rcd instnce for the user. intended to be called after `register_user`
    pub fn create_rcd_instance(
        &self,
        un: &str,
        overwrite_existing: bool,
    ) -> Result<String, RcdProxyErr> {
        let full_folder_path = self.setup_user_folder(overwrite_existing)?;
        let host_id = self.setup_rcd_service(un, &full_folder_path)?;

        let mut u = self.db.get_user(un)?;
        u.id = Some(host_id.clone());

        if u.folder.is_none() {
            u.folder = Some(full_folder_path);
        }

        self.db.update_user(&u)?;

        trace!("create_rcd_instance: {u:?}");

        Ok(host_id)
    }

    /// sets up a brand new rcd service for the specified user and updates the rcd folder for this user
    /// intended to be called after a user is registered
    pub fn setup_rcd_service(
        &self,
        un: &str,
        full_folder_path: &str,
    ) -> Result<String, RcdProxyErr> {
        trace!("un: {} dir: {}", un, full_folder_path);

        let settings = self.get_default_rcd_setings(un);
        let mut u = self.db.get_user(un)?;

        if u.folder.is_none() {
            u.folder = Some(full_folder_path.to_string());
            self.db.update_user(&u).unwrap();
        }

        let mut service = rcdx::rcd_service::get_service_from_config(settings, full_folder_path);

        trace!("{service:?}");

        service.init_at_dir(&full_folder_path, un, u.hash.clone());
        service.warn_init_host_info();

        let host_id = service.get_host_id();

        if u.id.is_none() {
            u.id = Some(host_id.clone());
            self.db.update_user(&u).unwrap();
        }

        Ok(host_id)
    }

    pub fn get_rcd_service_for_existing_user(&self, un: &str) -> Result<RcdService, RcdProxyErr> {
        let u = self.db.get_user(un)?;
        let full_folder_path = self.get_user_root_dir(&u)?;

        let settings = self.get_default_rcd_setings(un);

        let mut service = rcdx::rcd_service::get_service_from_config(settings, &full_folder_path);
        service.start_at_dir(&full_folder_path);
        Ok(service)
    }

    pub fn get_rcd_core_for_existing_host(&self, id: &str) -> Result<Rcd, RcdProxyErr> {
        let mut service = self.get_rcd_service_for_existing_host(id)?;
        service.with_core_grpc(&self.settings.grpc_db_addr_port, 60);
        Ok(service.core().clone())
    }

    pub fn get_rcd_core_data_for_existing_host_grpc(
        &self,
        id: &str,
        proxy_grpc_addr_port: &str,
        proxy_grpc_timeout_in_sec: u32,
    ) -> Result<RcdData, RcdProxyErr> {
        let mut service = self.get_rcd_service_for_existing_host(id)?;
        service.with_core_grpc(proxy_grpc_addr_port, proxy_grpc_timeout_in_sec);
        Ok(service.core_data().clone())
    }

    pub fn get_rcd_core_for_existing_host_grpc(
        &self,
        id: &str,
        proxy_grpc_addr_port: &str,
        proxy_grpc_timeout_in_sec: u32,
    ) -> Result<Rcd, RcdProxyErr> {
        let mut service = self.get_rcd_service_for_existing_host(id)?;
        service.with_core_grpc(proxy_grpc_addr_port, proxy_grpc_timeout_in_sec);
        Ok(service.core().clone())
    }

    pub fn get_rcd_service_for_existing_host(&self, id: &str) -> Result<RcdService, RcdProxyErr> {
        let u = self.db.get_host(id)?;
        let full_folder_path = self.get_user_root_dir(&u)?;
        let settings = self.get_default_rcd_setings(&u.username);

        let mut service = rcdx::rcd_service::get_service_from_config(settings, &full_folder_path);
        service.start_at_dir(&full_folder_path);
        Ok(service)
    }

    fn get_default_rcd_setings(&self, un: &str) -> RcdSettings {
        RcdSettings {
            admin_un: un.to_string(),
            admin_pw: "".to_string(),
            database_type: DatabaseType::Sqlite,
            backing_database_name: "rcd.db".to_string(),
            grpc_client_service_addr_port: self.settings.grpc_client_addr_port.clone(),
            grpc_data_service_addr_port: self.settings.grpc_db_addr_port.clone(),
            client_grpc_timeout_in_seconds: 60,
            data_grpc_timeout_in_seconds: 60,
            http_addr: self.settings.http_ip.clone(),
            http_port: self.settings.http_port as u16,
        }
    }

    fn get_user_root_dir(&self, u: &UserInfo) -> Result<String, RcdProxyErr> {
        trace!("[{}]: {u:?}", function_name!());

        if u.folder.as_ref().is_none() {
            return Err(RcdProxyErr::UserFolderNotSet(u.username.clone()));
        }

        return Ok(Path::new(&self.settings.root_dir)
            .join(u.folder.as_ref().unwrap())
            .to_str()
            .unwrap()
            .to_string());
    }

    pub fn setup_user_folder(&self, overwrite_existing: bool) -> Result<String, RcdProxyErr> {
        let folder_id = Uuid::new_v4().to_string();
        let folder_path = Path::new(&self.settings.root_dir).join(folder_id);

        if Path::exists(&folder_path) && !overwrite_existing {
            return Err(RcdProxyErr::FolderAlreadyExists(
                folder_path.to_str().unwrap().to_string(),
            ));
        }

        if folder_path.exists() && overwrite_existing {
            fs::remove_dir_all(&folder_path).unwrap();
        }

        fs::create_dir_all(&folder_path).unwrap();

        Ok(folder_path.to_str().unwrap().to_string())
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
    let proxy = test_setup("rcd-proxy-db-unit-test-new");
    let result = proxy.register_user("test", "1234");
    assert_eq!(result, Ok(()));
}

#[test]
pub fn test_register_twice() {
    let proxy = test_setup("rcd-proxy-db-unit-test-register");

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

#[test]
pub fn test_register_and_setup_user_and_host() {
    let proxy = test_rcd_common_setup("rcd-proxy-unit-test-reg-setup-run-host").unwrap();
    let service = proxy.get_rcd_service_for_existing_user("test").unwrap();
    let host_id = service.get_host_id();
    debug!("{host_id:?}");
    assert!(host_id.len() > 0);

    let service = proxy.get_rcd_service_for_existing_host(&host_id).unwrap();
    let id = service.get_host_id();
    assert_eq!(host_id, id);
}

#[test]
pub fn test_register_and_setup_user() {
    let proxy = test_rcd_common_setup("rcd-proxy-unit-test-reg-setup-run").unwrap();
    let service = proxy.get_rcd_service_for_existing_user("test").unwrap();
    let host_id = service.get_host_id();
    debug!("{host_id:?}");
    assert!(host_id.len() > 0);
}

#[test]
pub fn test_get_rcd_for_user() {
    test_rcd_common_setup("rcd-proxy-unit-test-get-rcd-for-user").unwrap();
    assert!(true);
}

#[cfg(test)]
/// common test code - sets up a test folder and returns a rcd proxy
pub fn test_setup(test_name: &str) -> RcdProxy {
    use ignore_result::Ignore;
    use rcd_test_harness_common::get_test_temp_dir;
    use std::env;

    SimpleLogger::new().env().init().ignore();

    let root_dir = get_test_temp_dir(test_name);
    let config_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let proxy = RcdProxy::get_proxy_from_config_with_dir(&config_dir, &root_dir).unwrap();
    proxy.start();
    proxy
}

#[cfg(test)]
/// common setup code - sets up the proxy instance and then returns an rcd service for the "test" user
pub fn test_rcd_common_setup(test_name: &str) -> Option<RcdProxy> {
    let proxy = test_setup(test_name);
    let result_register = proxy.register_user("test", "1234");

    if result_register.is_err() {
        assert!(false);
    }

    let result_setup = proxy.setup_user_folder(false);

    match result_setup {
        Ok(root_dir) => {
            let result_setup_rcd = proxy.setup_rcd_service("test", &root_dir);

            match result_setup_rcd {
                Ok(host_id) => {
                    debug!("{host_id:?}");
                    assert!(host_id.len() > 0);
                    return Some(proxy);
                }
                Err(_) => assert!(false),
            }
        }
        Err(_) => assert!(false),
    }

    None
}
