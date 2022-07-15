use crate::client_srv::SqlClientImpl;
use config::Config;
use env_logger::{Builder, Target};
use log::{error, info, trace, warn};
use rusqlite::{named_params, Connection, Result};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::{cmp::PartialEq, sync::Arc};
//use crate::cdata::sql_client_server::SqlClientServer;
use crate::client_srv::cdata::sql_client_server::SqlClientServer;
//use crate::rcd::cdata::cdata::sql_client_server::SqlClientServer;
//use crate::rcd::client_srv::cdata::sql_client_server::SqlClientServer;
use std::{thread, time};

//use crate::client_srv::cdata::FILE_DESCRIPTOR_SET;
//use crate::rcd::cdata::cdata::FILE_DESCRIPTOR_SET;
use crate::rcd::client_srv::cdata::FILE_DESCRIPTOR_SET;

use tonic::transport::Server;

#[path = "rcd/client_srv.rs"]
pub mod cdata;

mod db_srv;
mod rcd_db;

#[path = "rcd/client_srv.rs"]
pub mod client_srv;

/// Represents settings for rcd that can be passed in on a test case
#[derive(Debug, Clone)]
pub struct RcdSettings {
    admin_un: String,
    admin_pw: String,
    database_type: DatabaseType,
    backing_database_name: String,
    client_service_addr_port: String,
    database_service_addr_port: String,
}

/// Represents the type of backing database rcd is hosting
/// # Types
/// * 0 - Unknown
/// * 1 - Sqlite
/// * 2 - Mysql
/// * 3 - Postgres
/// * 4 - Sqlserver
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DatabaseType {
    Unknown = 0,
    Sqlite = 1,
    Mysql = 2,
    Postgres = 3,
    Sqlserver = 4,
}

// https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
impl DatabaseType {
    fn from_i64(value: i64) -> DatabaseType {
        match value {
            0 => DatabaseType::Unknown,
            1 => DatabaseType::Sqlite,
            2 => DatabaseType::Mysql,
            3 => DatabaseType::Postgres,
            4 => DatabaseType::Sqlserver,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RcdService {
    rcd_settings: RcdSettings,
}

impl RcdService {
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

        let item = client_srv::start_service(
            &self.rcd_settings.client_service_addr_port,
            &cwd,
            &self.rcd_settings.backing_database_name,
        );
    }

    #[tokio::main]
    pub async fn start_client_async(self: &Self) -> Result<(), Box<dyn std::error::Error>> {
        info!("start_client_service");

        let wd = env::current_dir().unwrap();
        let cwd = wd.to_str().unwrap();

        client_srv::start_service(
            &self.rcd_settings.client_service_addr_port,
            &cwd,
            &self.rcd_settings.backing_database_name,
        )
    }

    #[tokio::main]
    pub async fn start_client_service_alt(self: &Self) -> Result<(), Box<dyn std::error::Error>> {
        let address_port = &self.rcd_settings.client_service_addr_port;
        let addr = address_port.parse().unwrap();
        let database_name = &self.rcd_settings.backing_database_name;

        let wd = env::current_dir().unwrap();
        let root_folder = wd.to_str().unwrap();

        let sql_client = SqlClientImpl {
            root_folder: root_folder.to_string(),
            database_name: database_name.to_string(),
            addr_port: address_port.to_string(),
        };

        let sql_client_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(
                crate::rcd::client_srv::cdata::FILE_DESCRIPTOR_SET,
            )
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

    pub fn start_data_service(self: &Self) {
        info!("start_data_service");
        db_srv::start_service(&self.rcd_settings.database_service_addr_port);
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
    let _db_path = Path::new(&cwd.to_str().unwrap()).join(&backing_db_name);
    let db_location = _db_path.as_os_str().to_str().unwrap();

    match db_type {
        DatabaseType::Sqlite => {
            rcd_db::configure(cwd.to_str().unwrap(), db_location);
            rcd_db::configure_admin(admin_un, admin_pw, db_location)
        }
        DatabaseType::Mysql => do_nothing(),
        DatabaseType::Postgres => do_nothing(),
        DatabaseType::Sqlserver => do_nothing(),
        _ => panic!("Unknown db type"),
    }
}

fn do_nothing() {
    println!("do nothing");
}

/// Test function that returns a call from the rcd mod
pub fn hello() {
    println!("hello rcd_service");
}

/// Returns an RcdService from the config file
pub fn get_service_from_config_file() -> RcdService {
    let settings = get_config_from_settings_file();
    let service = RcdService {
        rcd_settings: settings,
    };
    return service;
}

/// Returns an RcdService from the supplied config (normally used in testing)
pub fn get_service_from_config(config: RcdSettings) -> RcdService {
    return RcdService {
        rcd_settings: config,
    };
}

fn get_config_from_settings_file() -> RcdSettings {
    let settings = Config::builder()
        .add_source(config::File::with_name("src/Settings"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

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

    let rcd_setting = RcdSettings {
        admin_un: String::from("tester"),
        admin_pw: String::from("123456"),
        database_type: database_type,
        backing_database_name: s_db_name,
        client_service_addr_port: s_client_service_addr_port,
        database_service_addr_port: d_client_service_addr_port,
    };

    return rcd_setting;
}

fn init() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.is_test(true).try_init();
}

#[test]
fn test_read_settings_from_file() {
    init();
    let service = get_service_from_config_file();
    let db_type = service.rcd_settings.database_type;
    assert_eq!(db_type, DatabaseType::Sqlite);
}

#[test]
fn test_read_settings_from_config() {
    init();
    let rcd_setting = RcdSettings {
        admin_un: String::from("tester"),
        admin_pw: String::from("123456"),
        database_type: DatabaseType::Unknown,
        backing_database_name: String::from(""),
        client_service_addr_port: String::from("[::1]:50051"),
        database_service_addr_port: String::from(""),
    };

    let service = get_service_from_config(rcd_setting);

    assert_eq!(service.rcd_settings.database_type, DatabaseType::Unknown);
}

#[test]
fn test_configure_backing_db() {
    init();
    // to see the output, run the test with the following
    // cargo test -- --nocapture
    // RUST_LOG=debug cargo test -- --nocapture

    let rcd_setting = RcdSettings {
        admin_un: String::from("tester"),
        admin_pw: String::from("123456"),
        database_type: DatabaseType::Sqlite,
        backing_database_name: String::from("rcd_test.db"),
        client_service_addr_port: String::from("[::1]:50051"),
        database_service_addr_port: String::from(""),
    };

    let cwd = env::current_dir().unwrap();
    let db_path = Path::new(&cwd).join(&rcd_setting.backing_database_name);

    if db_path.exists() {
        fs::remove_file(&db_path).unwrap();
    }

    assert!(!db_path.exists());

    let service = get_service_from_config(rcd_setting);
    service.start();

    assert!(db_path.exists());
}

#[test]
fn test_hash() {
    init();

    info!("test_hash: running");

    let cwd = env::current_dir().unwrap();
    let backing_database_name = String::from("test.db");
    let db_path = Path::new(&cwd).join(&backing_database_name);

    if db_path.exists() {
        fs::remove_file(&db_path).unwrap();
    }

    rcd_db::configure(&cwd.to_str().unwrap(), &backing_database_name);

    let db_conn = Connection::open(&db_path).unwrap();

    let un = String::from("tester");
    let pw = String::from("1234");

    rcd_db::create_login(&un, &pw, &db_conn);
    let has_login = rcd_db::has_login(&un, &db_conn).unwrap();

    info!("test_hash: has_login {}", &has_login);

    assert!(&has_login);

    let is_valid = rcd_db::verify_login(&un, &pw, &db_conn);

    info!("test_hash: is_valid {}", is_valid);

    assert!(is_valid);
}

#[test]
fn test_hash_false() {
    init();
    info!("test_hash_false: running");

    let cwd = env::current_dir().unwrap();
    let backing_database_name = String::from("test.db");
    let db_path = Path::new(&cwd).join(&backing_database_name);

    if db_path.exists() {
        fs::remove_file(&db_path).unwrap();
    }

    rcd_db::configure(&cwd.to_str().unwrap(), &backing_database_name);

    let db_conn = Connection::open(&db_path).unwrap();

    let un = String::from("tester_fail");
    let pw = String::from("1234");

    rcd_db::create_login(&un, &pw, &db_conn);
    let has_login = rcd_db::has_login(&un, &db_conn).unwrap();

    info!("test_hash_false: has_login {}", &has_login);

    assert!(&has_login);

    let wrong_pw = String::from("43210");

    let is_valid = rcd_db::verify_login(&un, &wrong_pw, &db_conn);

    info!("test_hash_false: is_valid {}", is_valid);

    assert!(!is_valid);
}

pub mod test_client_srv {
    use crate::cdata::sql_client_client::SqlClientClient;
    use crate::cdata::TestRequest;
    use crate::rcd;
    use futures::future::lazy;
    use log::info;
    extern crate futures;
    extern crate tokio;
    use std::cell::RefCell;
    use std::ops::DerefMut;
    use std::sync::{mpsc, Arc, Mutex};
    use std::{thread, time};

    #[tokio::main]
    async fn check_if_online(test_message: &str, addr_port: &str) -> String {
        info!("check_if_online attempting to connect");

        let default_addr_port = "http://[::1]:50051";

        // creating a channel ie connection to server
        let channel = tonic::transport::Channel::from_static(addr_port)
            .connect()
            .await
            .unwrap();
        // creating gRPC client from channel
        let mut client = SqlClientClient::new(channel);

        info!("created channel and client");

        // creating a new Request
        let request = tonic::Request::new(TestRequest {
            request_echo_message: test_message.to_string(),
            request_time_utc: String::from(""),
            request_origin_url: String::from(""),
            request_origin_ip4: String::from(""),
            request_origin_ip6: String::from(""),
            request_port_number: 1234,
        });
        // sending request and waiting for response

        info!("sending request");

        let response = client.is_online(request).await.unwrap().into_inner();
        println!("RESPONSE={:?}", response);
        info!("response back");

        return String::from(&response.reply_echo_message);
    }

    #[test]
    fn test_is_online() {
        //RUST_LOG=debug RUST_BACKTRACE=1 cargo test -- --nocapture
        // https://stackoverflow.com/questions/47764448/how-to-test-grpc-apis

        let test_message: &str = "test_client_srv";
        let (tx, rx) = mpsc::channel();

        let service = rcd::get_service_from_config_file();
        println!("{:?}", service);
        service.start();

        info!("starting client service");

        thread::spawn(move || {
            service.start_client_service_alt();
        });

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        let client_addr_port = service.rcd_settings.client_service_addr_port;

        // https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th

        thread::spawn(move || {
            let res = check_if_online(test_message, &client_addr_port);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        println!("test_is_online: got: {} sent: {}", response, test_message);

        assert_eq!(response, test_message);
    }
}
