use rcd_core::comm::{RcdCommunication, RcdRemoteDbClient};
use rcd_core::dbi::Dbi;
use rcd_core::rcd::Rcd;
use rcd_core::remote_grpc::RemoteGrpc;
use rcd_grpc::data_srv::DataServiceImpl;
use rcd_grpc::sqlclient_srv::SqlClientImpl;
use rcdproto::rcdp::{data_service_server::DataServiceServer, sql_client_server::SqlClientServer};
use std::{env, thread};
use tonic::transport::Server;
use triggered::Listener;

use super::RcdService;

#[tokio::main]
pub async fn start_client_service_at_addr_with_shutdown(
    database_name: &str,
    own_db_addr_port: &str,
    address_port: String,
    root_folder: String,
    db_interface: Option<Dbi>,
    shutdown: Listener,
    core: Rcd,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = address_port.parse().unwrap();

    let sql_client = SqlClientImpl {
        root_folder: root_folder,
        database_name: database_name.to_string(),
        addr_port: address_port.to_string(),
        own_db_addr_port: own_db_addr_port.to_string(),
        db_interface: db_interface,
        core: Some(core),
    };

    let sql_client_service = tonic_reflection::server::Builder::configure()
        .build()
        .unwrap();

    println!("Client Service Starting At: {}", addr);

    Server::builder()
        .add_service(SqlClientServer::new(sql_client))
        .add_service(sql_client_service) // Add this
        .serve_with_shutdown(addr, shutdown)
        .await?;

    Ok(())
}

#[tokio::main]
pub async fn start_db_service_at_addr_with_shutdown(
    database_name: &str,
    address_port: String,
    root_folder: String,
    db_interface: Option<Dbi>,
    shutdown: Listener,
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
        .serve_with_shutdown(addr, shutdown)
        .await?;

    Ok(())
}

pub fn start_grpc_at_addrs_with_shutdown(
    service: &mut RcdService,
    db_name: String,
    client_address_port: String,
    db_address_port: String,
    root_folder: String,
    client_shutdown_listener: Listener,
    db_shutdown_listener: Listener,
) -> Result<(), Box<dyn std::error::Error>> {
    let db1 = db_name.clone();
    let db2 = db_name.clone();

    let root1 = root_folder.clone();
    let root2 = root_folder.clone();

    let db_addr1 = db_address_port.clone();
    let db_addr2 = db_address_port.clone();

    let dbi1 = service.db_interface.clone();
    let dbi2 = service.db_interface.clone();
    let dbi3 = service.db_interface.clone();

    let grpc = RemoteGrpc {
        db_addr_port: db_address_port.clone(),
    };

    let remote_client = RcdRemoteDbClient {
        comm_type: RcdCommunication::Grpc,
        grpc: Some(grpc),
        http: None,
    };

    let core = Rcd {
        db_interface: Some(dbi3.unwrap()),
        remote_client: Some(remote_client),
    };

    thread::spawn(move || {
        let name = db1.clone();
        let _ = start_client_service_at_addr_with_shutdown(
            &name.to_string(),
            &db_addr1,
            client_address_port,
            root1,
            dbi1,
            client_shutdown_listener,
            core,
        )
        .unwrap();
    });

    thread::spawn(move || {
        let name = db2.clone();
        let _ = start_db_service_at_addr_with_shutdown(
            &name.to_string(),
            db_addr2,
            root2,
            dbi2,
            db_shutdown_listener,
        )
        .unwrap();
    });

    Ok(())
}

#[tokio::main]
pub async fn start_grpc_client_service_alt(
    service: &RcdService,
) -> Result<(), Box<dyn std::error::Error>> {
    let address_port = &service.rcd_settings.client_service_addr_port;
    let own_db_addr_port = &service.rcd_settings.database_service_addr_port;
    let addr = address_port.parse().unwrap();
    let database_name = &service.rcd_settings.backing_database_name;

    let wd = env::current_dir().unwrap();
    let root_folder = wd.to_str().unwrap();

    let dbi = service.db_interface.clone().unwrap();
    let dbi2 = dbi.clone();

    let grpc = RemoteGrpc {
        db_addr_port: own_db_addr_port.clone(),
    };

    let remote_client = RcdRemoteDbClient {
        comm_type: RcdCommunication::Grpc,
        grpc: Some(grpc),
        http: None,
    };

    let core = Rcd {
        db_interface: Some(dbi2),
        remote_client: Some(remote_client),
    };

    let sql_client = SqlClientImpl {
        root_folder: root_folder.to_string(),
        database_name: database_name.to_string(),
        addr_port: address_port.to_string(),
        own_db_addr_port: own_db_addr_port.to_string(),
        db_interface: Some(dbi),
        core: Some(core),
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
pub async fn start_grpc_client_service_at_addr(
    service: &RcdService,
    address_port: String,
    root_folder: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("start_client_service_at_addr: {}", &address_port);

    let addr = address_port.parse().unwrap();
    let database_name = &service.rcd_settings.backing_database_name;
    let own_db_addr_port = &service.rcd_settings.database_service_addr_port;

    let dbi = service.db_interface.clone().unwrap();
    let core = configure_core_for_grpc(&dbi, own_db_addr_port);

    let sql_client = SqlClientImpl {
        root_folder: root_folder,
        database_name: database_name.to_string(),
        addr_port: address_port.to_string(),
        own_db_addr_port: own_db_addr_port.to_string(),
        db_interface: Some(dbi),
        core: Some(core),
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

fn configure_core_for_grpc(dbi: &Dbi, own_db_addr_port: &str) -> Rcd {
    let grpc = RemoteGrpc {
        db_addr_port: own_db_addr_port.to_string(),
    };

    let remote_client = RcdRemoteDbClient {
        comm_type: RcdCommunication::Grpc,
        grpc: Some(grpc),
        http: None,
    };

    return Rcd {
        db_interface: Some(dbi.clone()),
        remote_client: Some(remote_client),
    };
}
