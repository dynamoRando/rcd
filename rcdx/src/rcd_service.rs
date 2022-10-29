use config::Config;
use log::info;
use rcd_common::db::DbiConfigSqlite;
use rcd_common::rcd_enum::DatabaseType;
use rcd_common::rcd_settings::RcdSettings;
use rcd_core::dbi::Dbi;
use rcd_core::rcd::Rcd;
use rcd_grpc::data_srv::{self, DataServiceImpl};
use rcd_grpc::sqlclient_srv::{self, SqlClientImpl};
use rcdproto::rcdp::{data_service_server::DataServiceServer, sql_client_server::SqlClientServer};
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::{env, thread};
use tonic::transport::Server;
use triggered::Listener;

mod grpc;
mod http;

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
}

impl RcdService {
    pub fn cwd(&self) -> String {
        if self.root_dir == "" {
            let wd = env::current_dir().unwrap();
            let cwd = wd.to_str().unwrap();
            return cwd.to_string();
        } else {
            return self.root_dir.clone();
        }
    }

    pub fn start_at_dir(self: &mut Self, root_dir: &str) {
        configure_backing_store_at_dir(
            self.rcd_settings.database_type,
            &self.rcd_settings.backing_database_name,
            &self.rcd_settings.admin_un,
            &self.rcd_settings.admin_pw,
            &root_dir,
        );

        let db_type = self.rcd_settings.database_type;

        match db_type {
            DatabaseType::Sqlite => {
                let sqlite_config = DbiConfigSqlite {
                    root_folder: root_dir.to_string(),
                    rcd_db_name: self.rcd_settings.backing_database_name.clone(),
                };

                let config = Dbi {
                    db_type: db_type,
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

    pub fn get_dbi(&self) -> Dbi {
        return self.db_interface.as_ref().unwrap().clone();
    }

    pub fn start(self: &mut Self) {
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
                    db_type: db_type,
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

    pub fn start_client_service(self: &Self) {
        info!("start_client_service");

        let wd = env::current_dir().unwrap();
        let cwd = wd.to_str().unwrap();

        let _item = sqlclient_srv::start_client_service(
            &self.rcd_settings.client_service_addr_port,
            &cwd,
            &self.rcd_settings.backing_database_name,
            &self.rcd_settings.database_service_addr_port,
        );
    }

    pub fn start_db_service(&self) {
        info!("start_db_service");

        let wd = env::current_dir().unwrap();
        let cwd = wd.to_str().unwrap();

        let _item = data_srv::start_db_service(
            &self.rcd_settings.client_service_addr_port,
            &cwd,
            &self.rcd_settings.backing_database_name,
        );
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        return grpc::start_grpc_at_addrs_with_shutdown(
            &mut self,
            db_name,
            client_address_port,
            db_address_port,
            root_folder,
            client_shutdown_listener,
            db_shutdown_listener,
        );
    }

    #[tokio::main]
    pub async fn start_services_at_addrs(
        self: &Self,
        db_name: String,
        client_address_port: String,
        db_address_port: String,
        root_folder: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db1 = db_name.clone();
        let db2 = db_name.clone();

        let root1 = root_folder.clone();
        let root2 = root_folder.clone();

        let db_addr1 = db_address_port.clone();
        let db_addr2 = db_address_port.clone();

        let dbi1 = self.db_interface.clone();
        let dbi2 = self.db_interface.clone();

        thread::spawn(move || {
            let name = db1.clone();
            let _ = Self::start_client_service_at_addr_alt(
                &name.to_string(),
                &db_addr1,
                client_address_port,
                root1,
                dbi1,
            )
            .unwrap();
        });

        thread::spawn(move || {
            let name = db2.clone();
            let _ = Self::start_db_service_at_addr_alt(&name.to_string(), db_addr2, root2, dbi2)
                .unwrap();
        });

        Ok(())
    }

    pub fn start_grpc_client_service_alt(self: &Self) -> Result<(), Box<dyn std::error::Error>> {
        return grpc::start_grpc_client_service_alt(self);
    }

    #[tokio::main]
    pub async fn start_client_service_at_addr_alt(
        database_name: &str,
        own_db_addr_port: &str,
        address_port: String,
        root_folder: String,
        db_interface: Option<Dbi>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let addr = address_port.parse().unwrap();

        let sql_client = SqlClientImpl {
            root_folder: root_folder,
            database_name: database_name.to_string(),
            addr_port: address_port.to_string(),
            own_db_addr_port: own_db_addr_port.to_string(),
            db_interface: db_interface,
            core: None,
        };

        let sql_client_service = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        println!("Client Service Starting At: {}", addr);

        Server::builder()
            .add_service(SqlClientServer::new(sql_client))
            .add_service(sql_client_service) // Add this
            .serve(addr)
            .await?;

        Ok(())
    }

    #[tokio::main]
    pub async fn start_client_service_at_addr(
        self: &Self,
        address_port: String,
        root_folder: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("start_client_service_at_addr: {}", &address_port);

        let addr = address_port.parse().unwrap();
        let database_name = &self.rcd_settings.backing_database_name;
        let own_db_addr_port = &self.rcd_settings.database_service_addr_port;

        let dbi = self.db_interface.clone().unwrap();

        let sql_client = SqlClientImpl {
            root_folder: root_folder,
            database_name: database_name.to_string(),
            addr_port: address_port.to_string(),
            own_db_addr_port: own_db_addr_port.to_string(),
            db_interface: Some(dbi),
            core: None,
        };

        let sql_client_service = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        println!(
            "start_client_service_at_addr: sql client server listening on {}",
            addr
        );

        Server::builder()
            .add_service(SqlClientServer::new(sql_client))
            .add_service(sql_client_service) // Add this
            .serve(addr)
            .await?;

        Ok(())
    }

    #[tokio::main]
    pub async fn start_db_service_at_addr(
        self: &Self,
        address_port: String,
        root_folder: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("start_db_service_at_addr: {}", &address_port);

        let addr = address_port.parse().unwrap();
        let database_name = &self.rcd_settings.backing_database_name;

        let dbi = self.db_interface.clone().unwrap();

        let data_service = DataServiceImpl {
            root_folder: root_folder,
            database_name: database_name.to_string(),
            addr_port: address_port.to_string(),
            db_interface: Some(dbi),
        };

        let data_service_server = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        println!("start_db_service_at_addr: db server listening on {}", addr);

        Server::builder()
            .add_service(DataServiceServer::new(data_service))
            .add_service(data_service_server) // Add this
            .serve(addr)
            .await?;

        Ok(())
    }

    #[tokio::main]
    pub async fn start_db_service_at_addr_alt(
        database_name: &str,
        address_port: String,
        root_folder: String,
        db_interface: Option<Dbi>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let addr = address_port.parse().unwrap();

        let data_service = DataServiceImpl {
            root_folder: root_folder,
            database_name: database_name.to_string(),
            addr_port: address_port.to_string(),
            db_interface: db_interface,
        };

        let data_service_server = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        println!("Database Service Starting At: {}", addr);

        Server::builder()
            .add_service(DataServiceServer::new(data_service))
            .add_service(data_service_server) // Add this
            .serve(addr)
            .await?;

        Ok(())
    }
}

/// Returns an RcdService from the config file
pub fn get_service_from_config_file() -> RcdService {
    let settings = get_config_from_settings_file();
    let mut service = RcdService {
        rcd_settings: settings,
        root_dir: String::from(""),
        db_interface: None,
        sql_client_channel: None,
        db_client_channel: None,
        core: None,
    };

    if service.root_dir == "" {
        let wd = env::current_dir().unwrap().clone();
        let cwd = wd.to_str().unwrap().to_string().clone();
        service.root_dir = cwd.to_string();
    }

    return service;
}

#[allow(dead_code)]
/// Returns an RcdService from the supplied config (normally used in testing)
/// This function is normally called in tests
pub fn get_service_from_config(config: RcdSettings) -> RcdService {
    return RcdService {
        rcd_settings: config,
        root_dir: String::from(""),
        db_interface: None,
        sql_client_channel: None,
        db_client_channel: None,
        core: None,
    };
}

pub fn get_config_from_settings_file() -> RcdSettings {
    let wd = env::current_dir().unwrap();
    let cwd = wd.to_str().unwrap();
    let settings_in_cwd = Path::new(cwd).join("Settings.toml");

    let settings_location;

    if Path::exists(&settings_in_cwd) {
        settings_location = settings_in_cwd.to_str().unwrap();
    } else {
        settings_location = "src/Settings";
    }

    let error_message = format!(
        "{}{}",
        "Could not find Settings.toml in current directort or in default ", settings_location
    );

    let settings = Config::builder()
        .add_source(config::File::with_name(settings_location))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .expect(&error_message);

    let i_database_type = settings.get_int(&String::from("database_type")).unwrap();
    let database_type = DatabaseType::from_i64(i_database_type);

    let s_db_name = settings
        .get_string(&String::from("backing_database_name"))
        .unwrap();

    let s_client_service_addr_port = settings
        .get_string(&String::from("client_service_addr_port"))
        .unwrap();

    let d_client_service_addr_port = settings
        .get_string(&String::from("data_service_addr_port"))
        .unwrap();

    let admin_un = settings.get_string(&String::from("admin_un")).unwrap();

    let admin_pw = settings.get_string(&String::from("admin_pw")).unwrap();

    let rcd_setting = RcdSettings {
        admin_un: admin_un,
        admin_pw: admin_pw,
        database_type: database_type,
        backing_database_name: s_db_name,
        client_service_addr_port: s_client_service_addr_port,
        database_service_addr_port: d_client_service_addr_port,
    };

    return rcd_setting;
}

pub fn get_current_directory() -> String {
    let wd = env::current_dir().unwrap().clone();
    let cwd = wd.to_str().unwrap().to_string().clone();
    return cwd.to_string();
}
