use crate::cdata::data_service_server::{DataService, DataServiceServer};
use crate::cdata::sql_client_server::SqlClientServer;
use crate::configure_backing_store_at_dir;
use crate::data_srv::DataServiceImpl;
use crate::sqlclient_srv::SqlClientImpl;
use crate::{configure_backing_store, rcd_settings::RcdSettings};
use log::info;
use std::env;
use tonic::transport::Server;

#[derive(Debug, Clone)]
pub struct RcdService {
    pub rcd_settings: RcdSettings,
    pub root_dir: String,
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

    pub fn start_at_dir(self: &Self, root_dir: &str) {
        configure_backing_store_at_dir(
            self.rcd_settings.database_type,
            &self.rcd_settings.backing_database_name,
            &self.rcd_settings.admin_un,
            &self.rcd_settings.admin_pw,
            &root_dir,
        );
    }

    pub fn start(self: &Self) {
        configure_backing_store(
            self.rcd_settings.database_type,
            &self.rcd_settings.backing_database_name,
            &self.rcd_settings.admin_un,
            &self.rcd_settings.admin_pw,
        );
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
    pub async fn start_client_service_alt(self: &Self) -> Result<(), Box<dyn std::error::Error>> {
        let address_port = &self.rcd_settings.client_service_addr_port;
        let own_db_addr_port = &self.rcd_settings.database_service_addr_port;
        let addr = address_port.parse().unwrap();
        let database_name = &self.rcd_settings.backing_database_name;

        let wd = env::current_dir().unwrap();
        let root_folder = wd.to_str().unwrap();

        let sql_client = SqlClientImpl {
            root_folder: root_folder.to_string(),
            database_name: database_name.to_string(),
            addr_port: address_port.to_string(),
            own_db_addr_port: own_db_addr_port.to_string(),
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

    #[allow(dead_code)]
    #[tokio::main]
    pub async fn start_client_service_at_addr(
        self: &Self,
        address_port: String,
        root_folder: String
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("start_client_service_at_addr: {}", &address_port);

        let addr = address_port.parse().unwrap();
        let database_name = &self.rcd_settings.backing_database_name;
        let own_db_addr_port = &self.rcd_settings.database_service_addr_port;

        let sql_client = SqlClientImpl {
            root_folder: root_folder,
            database_name: database_name.to_string(),
            addr_port: address_port.to_string(),
            own_db_addr_port: own_db_addr_port.to_string(),
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

    #[allow(dead_code)]
    #[tokio::main]
    pub async fn start_db_service_at_addr(
        self: &Self,
        address_port: String,
        root_folder: String
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("start_db_service_at_addr: {}", &address_port);

        let addr = address_port.parse().unwrap();
        let database_name = &self.rcd_settings.backing_database_name;
    
        let data_service = DataServiceImpl {
            root_folder: root_folder,
            database_name: database_name.to_string(),
            addr_port: address_port.to_string()
        };

        let data_service_server = tonic_reflection::server::Builder::configure()
            .build()
            .unwrap();

        println!(
            "start_db_service_at_addr: db server listening on {}",
            addr
        );

        Server::builder()
            .add_service(DataServiceServer::new(data_service))
            .add_service(data_service_server) // Add this
            .serve(addr)
            .await?;

        Ok(())
    }
}
