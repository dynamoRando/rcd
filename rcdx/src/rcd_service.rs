use config::Config;
use guid_create::GUID;
use log::{error, info, trace, LevelFilter};
use rcd_common::db::DbiConfigSqlite;
use rcd_common::rcd_settings::RcdSettings;
use rcd_core::comm::{RcdCommunication, RcdRemoteDbClient};
use rcd_core::dbi::Dbi;
use rcd_core::rcd::Rcd;
use rcd_core::rcd_data::RcdData;
use rcd_core::remote_grpc::RemoteGrpc;
use rcd_sqlite_log::SqliteLog;
use std::env;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use triggered::Listener;

use rcd_enum::database_type::DatabaseType;

mod grpc;
mod http;

/// Intalizes the backing cds with the specified admin values. Intended to only be called by an rcd-proxy upon inital sign-up
fn init_backing_store_at_dir_with_hash(
    db_type: DatabaseType,
    backing_db_name: &str,
    root_dir: &str,
    admin_un: &str,
    admin_hash: Vec<u8>,
) {
    match db_type {
        DatabaseType::Sqlite => {
            let config = DbiConfigSqlite {
                root_folder: root_dir.to_string(),
                rcd_db_name: backing_db_name.to_string(),
            };

            let dbi = Dbi {
                db_type: DatabaseType::Sqlite,
                mysql_config: None,
                postgres_config: None,
                sqlite_config: Some(config),
            };

            dbi.configure_rcd_db();
            dbi.configure_admin_hash(admin_un, admin_hash);
        }
        DatabaseType::Mysql => todo!(),
        DatabaseType::Postgres => todo!(),
        DatabaseType::Sqlserver => todo!(),
        _ => panic!("Unknown db type"),
    }
}

fn configure_backing_store_at_existing_dir(
    db_type: DatabaseType,
    backing_db_name: &str,
    root_dir: &str,
) {
    match db_type {
        DatabaseType::Sqlite => {
            let config = DbiConfigSqlite {
                root_folder: root_dir.to_string(),
                rcd_db_name: backing_db_name.to_string(),
            };

            let dbi = Dbi {
                db_type: DatabaseType::Sqlite,
                mysql_config: None,
                postgres_config: None,
                sqlite_config: Some(config),
            };

            dbi.configure_rcd_db();
        }
        DatabaseType::Mysql => todo!(),
        DatabaseType::Postgres => todo!(),
        DatabaseType::Sqlserver => todo!(),
        _ => panic!("Unknown db type"),
    }

    todo!()
}

/// Configures the backing cds based on the type in the apps current working directory
fn configure_backing_store_at_dir(
    db_type: DatabaseType,
    backing_db_name: &str,
    admin_un: &str,
    admin_pw: &str,
    root_dir: &str,
) {
    match db_type {
        DatabaseType::Sqlite => {
            let config = DbiConfigSqlite {
                root_folder: root_dir.to_string(),
                rcd_db_name: backing_db_name.to_string(),
            };

            let dbi = Dbi {
                db_type: DatabaseType::Sqlite,
                mysql_config: None,
                postgres_config: None,
                sqlite_config: Some(config),
            };

            dbi.configure_rcd_db();
            dbi.configure_admin(admin_un, admin_pw);
        }
        DatabaseType::Mysql => todo!(),
        DatabaseType::Postgres => todo!(),
        DatabaseType::Sqlserver => todo!(),
        _ => panic!("Unknown db type"),
    }
}

/// Configures the backing cds based on the type in the apps current working directory
fn configure_backing_store(
    db_type: DatabaseType,
    backing_db_name: &str,
    admin_un: &str,
    admin_pw: &str,
) {
    let cwd = env::current_dir().unwrap();

    match db_type {
        DatabaseType::Sqlite => {
            let config = DbiConfigSqlite {
                root_folder: cwd.as_os_str().to_str().unwrap().to_string(),
                rcd_db_name: backing_db_name.to_string(),
            };

            let dbi = Dbi {
                db_type: DatabaseType::Sqlite,
                mysql_config: None,
                postgres_config: None,
                sqlite_config: Some(config),
            };

            dbi.configure_rcd_db();
            dbi.configure_admin(admin_un, admin_pw);
        }
        DatabaseType::Mysql => todo!(),
        DatabaseType::Postgres => todo!(),
        DatabaseType::Sqlserver => todo!(),
        _ => panic!("Unknown db type"),
    }
}

#[derive(Debug)]
pub struct RcdService {
    pub rcd_settings: RcdSettings,
    pub root_dir: String,
    pub db_interface: Option<Dbi>,
    pub sql_client_channel: Option<(Sender<bool>, Receiver<bool>)>,
    pub db_client_channel: Option<(Sender<bool>, Receiver<bool>)>,
    pub core: Option<Rcd>,
    pub core_data: Option<RcdData>,
}

impl RcdService {
    pub fn with_core_grpc(&mut self, db_addr_port: &str, timeout_in_seconds: u32) {
        let grpc = RemoteGrpc {
            db_addr_port: db_addr_port.to_string(),
            timeout_in_seconds: timeout_in_seconds,
        };

        let remote_client = RcdRemoteDbClient {
            comm_type: RcdCommunication::Grpc,
            grpc: Some(grpc),
            http: None,
        };

        let core = Rcd {
            db_interface: Some(self.db_interface.as_ref().unwrap().clone()),
            remote_client: Some(remote_client),
            settings: Some(self.rcd_settings.clone()),
        };

        let core_data = RcdData {
            db_interface: Some(self.db_interface.as_ref().unwrap().clone()),
        };

        self.core = Some(core);
        self.core_data = Some(core_data);
    }

    pub fn cwd(&self) -> String {
        if self.root_dir.is_empty() {
            let wd = env::current_dir().unwrap();
            let cwd = wd.to_str().unwrap();
            cwd.to_string()
        } else {
            self.root_dir.clone()
        }
    }

    pub fn core(&self) -> &Rcd {
        return self.core.as_ref().unwrap();
    }

    pub fn core_data(&self) -> &RcdData {
        return self.core_data.as_ref().unwrap();
    }

    /// initalizes the service at the specified directory with the username and hash provided
    /// note: this is intended to be called by a rcd-proxy instance upon registration of an
    /// rcd account - to be called only once
    pub fn init_at_dir(&mut self, root_dir: &str, admin_un: &str, admin_hash: Vec<u8>) {
        trace!("init at dir: {root_dir:?}");

        init_backing_store_at_dir_with_hash(
            self.rcd_settings.database_type,
            &self.rcd_settings.backing_database_name,
            root_dir,
            admin_un,
            admin_hash,
        );
    }

    pub fn start_at_existing_dir(&mut self, root_dir: &str) {
        configure_backing_store_at_existing_dir(
            self.rcd_settings.database_type,
            &self.rcd_settings.backing_database_name,
            root_dir,
        );
        let db_type = self.rcd_settings.database_type;

        match db_type {
            DatabaseType::Sqlite => {
                let sqlite_config = DbiConfigSqlite {
                    root_folder: root_dir.to_string(),
                    rcd_db_name: self.rcd_settings.backing_database_name.clone(),
                };

                let config = Dbi {
                    db_type,
                    mysql_config: None,
                    postgres_config: None,
                    sqlite_config: Some(sqlite_config),
                };

                self.db_interface = Some(config);
            }

            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
            _ => panic!("Unknown db type"),
        }
    }

    /// initializes the service from the settings, overriding the current working directory with the specified value
    pub fn start_at_dir(&mut self, root_dir: &str) {
        trace!("start at dir: {root_dir:?}");

        configure_backing_store_at_dir(
            self.rcd_settings.database_type,
            &self.rcd_settings.backing_database_name,
            &self.rcd_settings.admin_un,
            &self.rcd_settings.admin_pw,
            root_dir,
        );

        let db_type = self.rcd_settings.database_type;

        match db_type {
            DatabaseType::Sqlite => {
                let sqlite_config = DbiConfigSqlite {
                    root_folder: root_dir.to_string(),
                    rcd_db_name: self.rcd_settings.backing_database_name.clone(),
                };

                let config = Dbi {
                    db_type,
                    mysql_config: None,
                    postgres_config: None,
                    sqlite_config: Some(sqlite_config),
                };

                self.db_interface = Some(config);
            }

            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
            _ => panic!("Unknown db type"),
        }
    }

    pub fn enable_internal_logging(&self, root_dir: &str, log_level: LevelFilter) {
        if let Err(e) = SqliteLog::init_at_dir(log_level, root_dir.to_string()) {
            error!("{}", e);
        }
    }

    pub fn get_dbi(&self) -> Dbi {
        return self.db_interface.as_ref().unwrap().clone();
    }

    /// initalizes the service from the settings, using the current working directory as the root for files
    pub fn start(&mut self) {
        configure_backing_store(
            self.rcd_settings.database_type,
            &self.rcd_settings.backing_database_name,
            &self.rcd_settings.admin_un,
            &self.rcd_settings.admin_pw,
        );

        let db_type = self.rcd_settings.database_type;

        match db_type {
            DatabaseType::Sqlite => {
                let sqlite_config = DbiConfigSqlite {
                    root_folder: self.cwd(),
                    rcd_db_name: self.rcd_settings.backing_database_name.clone(),
                };

                let config = Dbi {
                    db_type,
                    mysql_config: None,
                    postgres_config: None,
                    sqlite_config: Some(sqlite_config),
                };

                self.db_interface = Some(config);
            }

            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
            _ => panic!("Unknown db type"),
        }
    }

    /// Initalizes the `RCD` instance with a random generated host_id, host_name, and token.
    /// Effectively this is the same thing as a call to rcd.rs (core) `.generate_host_info(host_name)`
    /// and returns the host_id.
    ///
    /// __This is intended to be called only ONCE when a brand new RCD instance
    /// has been created. This function WILL PANIC if there is a host id already set.__
    pub fn warn_init_host_info(&mut self) {
        self.init_dbi();
        let db = self.db_interface.as_ref().unwrap();

        trace!("{db:?}");

        let host_info = db.rcd_get_host_info();
        if host_info.is_none() {
            let id = GUID::rand();
            db.rcd_generate_host_info(&id.to_string());
        } else {
            let host_id = host_info.unwrap().id;
            panic!("a host id has already been set: {host_id}")
        }
    }

    pub fn get_host_id(&self) -> String {
        let db = self.db_interface.as_ref().unwrap();
        db.rcd_get_host_info().expect("no host info is set").id
    }

    #[tokio::main]
    pub async fn start_http_at_addr(mut self, http_addr: String, http_port: u16) {
        http::start_http_at_addr(&mut self, http_addr, http_port)
    }

    #[tokio::main]
    pub async fn start_http_at_addr_and_dir(
        mut self,
        http_addr: String,
        http_port: u16,
        root_dir: String,
    ) {
        http::start_http_at_addr_and_dir(&mut self, http_addr, http_port, root_dir)
    }

    pub fn shutdown_http(addr: &str, port: u32) {
        http::shutdown(addr, port);
    }

    #[tokio::main]
    pub async fn start_grpc_at_addrs_with_shutdown(
        mut self,
        db_name: String,
        client_address_port: String,
        db_address_port: String,
        root_folder: String,
        client_shutdown_listener: Listener,
        db_shutdown_listener: Listener,
        data_grpc_timeout_in_seconds: u32,
        settings: Option<RcdSettings>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        return grpc::start_grpc_at_addrs_with_shutdown(
            &mut self,
            db_name,
            client_address_port,
            db_address_port,
            root_folder,
            client_shutdown_listener,
            db_shutdown_listener,
            data_grpc_timeout_in_seconds,
            settings,
        );
    }

    pub fn start_grpc_client_service_alt(&self) -> Result<(), Box<dyn std::error::Error>> {
        return grpc::start_grpc_client_service_alt(self);
    }

    pub fn start_grpc_client_service_at_addr(
        &self,
        address_port: String,
        root_folder: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        return grpc::start_grpc_client_service_at_addr(self, address_port, root_folder);
    }

    fn init_dbi(&mut self) {
        let db_type = self.rcd_settings.database_type;

        match db_type {
            DatabaseType::Sqlite => {
                let sqlite_config = DbiConfigSqlite {
                    root_folder: self.root_dir.clone(),
                    rcd_db_name: self.rcd_settings.backing_database_name.clone(),
                };

                let config = Dbi {
                    db_type,
                    mysql_config: None,
                    postgres_config: None,
                    sqlite_config: Some(sqlite_config),
                };

                trace!("{config:?}");

                self.db_interface = Some(config);
            }
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
            _ => panic!("Unknown db type"),
        }
    }
}

/// Returns an RcdService from the config file
pub fn get_service_from_config_file(settings_filename: Option<String>) -> RcdService {
    let settings = get_config_from_settings_file(settings_filename);
    let mut service = RcdService {
        rcd_settings: settings,
        root_dir: String::from(""),
        db_interface: None,
        sql_client_channel: None,
        db_client_channel: None,
        core: None,
        core_data: None,
    };

    if service.root_dir.is_empty() {
        let wd = env::current_dir().unwrap();
        let cwd = wd.to_str().unwrap().to_string();
        service.root_dir = cwd;
    }

    service
}

#[allow(dead_code)]
/// Returns an RcdService from the supplied config (normally used in testing)
/// This function is normally called in tests
pub fn get_service_from_config(config: RcdSettings, root_dir: &str) -> RcdService {
    RcdService {
        rcd_settings: config,
        root_dir: root_dir.to_string(),
        db_interface: None,
        sql_client_channel: None,
        db_client_channel: None,
        core: None,
        core_data: None,
    }
}

pub fn get_config_from_settings_file(settings_filename: Option<String>) -> RcdSettings {
    let wd = env::current_dir().unwrap();
    let cwd = wd.to_str().unwrap();

    let filename: String = if settings_filename.is_none() {
        String::from("Settings.toml")
    } else {
        settings_filename.unwrap()
    };

    let settings_in_cwd = Path::new(cwd).join(filename.clone());

    let settings_location = if Path::exists(&settings_in_cwd) {
        settings_in_cwd.to_str().unwrap()
    } else {
        "src/Settings"
    };

    let error_message = format!(
        "{}{}{}{}",
        "Could not find ", filename, "in current directory or in default ", settings_location
    );

    let settings = Config::builder()
        .add_source(config::File::with_name(settings_location))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .expect(&error_message);

    info!("Using settings file: {settings_location}");

    let i_database_type = settings.get_int(&String::from("database_type")).unwrap();
    let database_type = DatabaseType::from_i64(i_database_type);

    let s_db_name = settings
        .get_string(&String::from("backing_database_name"))
        .unwrap();

    let s_client_service_addr_port = settings
        .get_string(&String::from("grpc_client_service_addr_port"))
        .unwrap();

    let s_client_timeout = settings
        .get_string(&String::from("client_grpc_timeout_in_seconds"))
        .unwrap();

    let s_data_timeout = settings
        .get_string(&String::from("data_grpc_timeout_in_seconds"))
        .unwrap();

    let client_timeout_in_seconds: u32 = s_client_timeout.parse().unwrap();
    let data_timeout_in_seconds: u32 = s_data_timeout.parse().unwrap();

    let d_client_service_addr_port = settings
        .get_string(&String::from("grpc_data_service_addr_port"))
        .unwrap();

    let admin_un = settings.get_string(&String::from("admin_un")).unwrap();

    let admin_pw = settings.get_string(&String::from("admin_pw")).unwrap();

    let http_addr = settings.get_string(&String::from("http_addr")).unwrap();
    let http_port = settings.get_int(&String::from("http_port")).unwrap() as u16;

    RcdSettings {
        admin_un,
        admin_pw,
        database_type,
        backing_database_name: s_db_name,
        grpc_client_service_addr_port: s_client_service_addr_port,
        grpc_data_service_addr_port: d_client_service_addr_port,
        client_grpc_timeout_in_seconds: client_timeout_in_seconds,
        data_grpc_timeout_in_seconds: data_timeout_in_seconds,
        http_addr,
        http_port,
    }
}

pub fn get_current_directory() -> String {
    let wd = env::current_dir().unwrap();
    let cwd = wd.to_str().unwrap().to_string();
    cwd
}
