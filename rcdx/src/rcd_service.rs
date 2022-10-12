use crate::configure_backing_store_at_dir;
use crate::data_srv::DataServiceImpl;
use crate::dbi::{Dbi, DbiConfigSqlite};
use crate::rcd_enum::DatabaseType;
use crate::sqlclient_srv::SqlClientImpl;
use crate::{configure_backing_store, rcd_settings::RcdSettings};
use rcdproto::rcdp::{data_service_server::DataServiceServer, sql_client_server::SqlClientServer};
use std::{env, thread};
use tonic::transport::Server;
use log::info;

#[derive(Debug, Clone)]
pub struct RcdService {
    pub rcd_settings: RcdSettings,
    pub root_dir: String,
    pub db_interface: Option<Dbi>,
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

        let _item = crate::sqlclient_srv::start_client_service(
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

        let _item = crate::data_srv::start_db_service(
            &self.rcd_settings.client_service_addr_port,
            &cwd,
            &self.rcd_settings.backing_database_name,
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

    #[tokio::main]
    pub async fn start_client_service_alt(self: &Self) -> Result<(), Box<dyn std::error::Error>> {
        let address_port = &self.rcd_settings.client_service_addr_port;
        let own_db_addr_port = &self.rcd_settings.database_service_addr_port;
        let addr = address_port.parse().unwrap();
        let database_name = &self.rcd_settings.backing_database_name;

        let wd = env::current_dir().unwrap();
        let root_folder = wd.to_str().unwrap();

        let dbi = self.db_interface.clone().unwrap();

        let sql_client = SqlClientImpl {
            root_folder: root_folder.to_string(),
            database_name: database_name.to_string(),
            addr_port: address_port.to_string(),
            own_db_addr_port: own_db_addr_port.to_string(),
            db_interface: Some(dbi),
        };

        let sql_client_service = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        println!("sql client server listening on {}", addr);

        Server::builder()
            .add_service(SqlClientServer::new(sql_client))
            .add_service(sql_client_service) // Add this
            .serve(addr)
            .await?;

        Ok(())
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
