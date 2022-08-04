mod client_srv;
mod crypt;
mod db_srv;
mod rcd_db;
mod rcd_enum;
mod sql_text;
mod sqlitedb;
mod sqlitedbpart;
mod table;
mod test_harness;
mod cdata;
mod rcd_settings;
mod rcd_service;

#[cfg(test)]
use client_srv::SqlClientImpl;
#[allow(unused_imports)]
use db_srv::DataServiceImpl;
use config::Config;
use log::info;
use std::env;
use std::path::Path;
#[cfg(test)]
use cdata::sql_client_server::SqlClientServer;
#[cfg(test)]
#[allow(unused_imports)]
use crate::cdata::data_service_client::DataServiceClient;
#[cfg(test)]
use tonic::transport::Server;


use crate::rcd_enum::DatabaseType;
use crate::rcd_service::RcdService;


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
            crate::rcd_db::configure(cwd.to_str().unwrap(), db_location);
            crate::rcd_db::configure_admin(admin_un, admin_pw, db_location)
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

pub mod tests {
    pub mod db_serv {
        // place_holder
        pub mod save_contract {

            #[cfg(test)]
            #[tokio::main]
            #[allow(dead_code, unused_variables)]
            async fn client_host(addr_port: &str) {
                unimplemented!();
            }

            #[cfg(test)]
            #[tokio::main]
            #[allow(dead_code, unused_variables)]
            async fn client_participant(addr_port: &str) {
                unimplemented!();
            }

            #[test]
            #[allow(dead_code, unused_variables)]
            fn test() {
                /*
                    We will need to kick off two services, the host and the participant
                    and we will need to also kick off two clients, one for each
                */

                unimplemented!();
            }
        }
    }

    pub mod client_serv {
        pub mod is_online {
            #[cfg(test)]
            use crate::cdata::sql_client_client::SqlClientClient;
            #[cfg(test)]
            use crate::cdata::TestRequest;
            #[cfg(test)]
            use log::info;
            extern crate futures;
            extern crate tokio;
            #[cfg(test)]
            use std::sync::mpsc;
            #[cfg(test)]
            use std::{thread, time};

            #[cfg(test)]
            #[tokio::main]
            async fn client(test_message: &str, addr_port: &str) -> String {
                let addr_port = format!("{}{}", String::from("http://"), addr_port);
                info!("client_if_online attempting to connect {}", addr_port);

                let endpoint = tonic::transport::Channel::builder(addr_port.parse().unwrap());
                let channel = endpoint.connect().await.unwrap();

                let mut client = SqlClientClient::new(channel);

                info!("created channel and client");

                let request = tonic::Request::new(TestRequest {
                    request_echo_message: test_message.to_string(),
                    request_time_utc: String::from(""),
                    request_origin_url: String::from(""),
                    request_origin_ip4: String::from(""),
                    request_origin_ip6: String::from(""),
                    request_port_number: 1234,
                });

                info!("sending request");

                let response = client.is_online(request).await.unwrap().into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                return String::from(&response.reply_echo_message);
            }

            #[test]
            fn test() {
                let test_message: &str = "test_client_srv";
                let (tx, rx) = mpsc::channel();

                let service = crate::get_service_from_config_file();
                let client_address_port = service.rcd_settings.client_service_addr_port.clone();
                println!("{:?}", &service);
                service.start();

                info!("starting client service");

                thread::spawn(move || {
                    let _service = service.start_client_service_alt();
                });

                let time = time::Duration::from_secs(5);

                info!("sleeping for 5 seconds...");

                thread::sleep(time);

                thread::spawn(move || {
                    let res = client(test_message, &client_address_port);
                    tx.send(res).unwrap();
                })
                .join()
                .unwrap();

                let response = rx.try_recv().unwrap();

                println!("test_is_online: got: {} sent: {}", response, test_message);

                assert_eq!(response, test_message);
            }
        }

        pub mod create_user_database {
            #[cfg(test)]
            use crate::cdata::sql_client_client::SqlClientClient;
            #[cfg(test)]
            use crate::cdata::CreateUserDatabaseRequest;
            #[cfg(test)]
            use log::info;
            extern crate futures;
            extern crate tokio;
            #[cfg(test)]
            use crate::test_harness;
            #[cfg(test)]
            use std::sync::mpsc;
            #[cfg(test)]
            use std::{thread, time};

            #[test]
            fn test() {
                let test_db_name: &str = "test_create_user_db.db";
                let (tx, rx) = mpsc::channel();
                let port_num = test_harness::TEST_SETTINGS
                    .lock()
                    .unwrap()
                    .get_next_avail_port();

                let service = crate::get_service_from_config_file();
                let client_address_port =
                    format!("{}{}", String::from("[::1]:"), port_num.to_string());
                let target_client_address_port = client_address_port.clone();
                println!("{:?}", &service);

                service.start();

                info!("starting client at {}", &client_address_port);
                info!("starting client service");

                thread::spawn(move || {
                    let _service = service.start_client_service_at_addr(client_address_port);
                });

                let time = time::Duration::from_secs(5);

                info!("sleeping for 5 seconds...");

                thread::sleep(time);

                thread::spawn(move || {
                    let res = client(test_db_name, &target_client_address_port);
                    tx.send(res).unwrap();
                })
                .join()
                .unwrap();

                let response = rx.try_recv().unwrap();

                println!("create_user_database: got: {}", response);

                assert!(response);
            }

            #[cfg(test)]
            #[tokio::main]
            async fn client(db_name: &str, addr_port: &str) -> bool {
                use crate::cdata::AuthRequest;

                let addr_port = format!("{}{}", String::from("http://"), addr_port);
                info!(
                    "client_create_user_database attempting to connect {}",
                    addr_port
                );

                let endpoint = tonic::transport::Channel::builder(addr_port.parse().unwrap());
                let channel = endpoint.connect().await.unwrap();
                let mut client = SqlClientClient::new(channel);

                info!("created channel and client");

                let auth = AuthRequest {
                    user_name: String::from("tester"),
                    pw: String::from("123456"),
                    pw_hash: Vec::new(),
                    token: Vec::new(),
                };

                let request = tonic::Request::new(CreateUserDatabaseRequest {
                    authentication: Some(auth),
                    database_name: db_name.to_string(),
                });

                info!("sending request");

                let response = client
                    .create_user_database(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                return response.is_created;
            }

            #[test]
            fn negative_test() {
                let test_db_name: &str = "test_create_user_db_false.db";
                let (tx, rx) = mpsc::channel();
                let port_num = test_harness::TEST_SETTINGS
                    .lock()
                    .unwrap()
                    .get_next_avail_port();

                let service = crate::get_service_from_config_file();
                let client_address_port =
                    format!("{}{}", String::from("[::1]:"), port_num.to_string());
                let target_client_address_port = client_address_port.clone();
                println!("{:?}", &service);

                service.start();

                info!("starting client at {}", &client_address_port);
                info!("starting client service");

                thread::spawn(move || {
                    let _service = service.start_client_service_at_addr(client_address_port);
                });

                let time = time::Duration::from_secs(5);

                info!("sleeping for 5 seconds...");

                thread::sleep(time);

                thread::spawn(move || {
                    let res = negative_client(test_db_name, &target_client_address_port);
                    tx.send(res).unwrap();
                })
                .join()
                .unwrap();

                let response = rx.try_recv().unwrap();

                println!("create_user_database: got: {}", response);

                assert!(!response);
            }

            #[cfg(test)]
            #[tokio::main]
            async fn negative_client(db_name: &str, addr_port: &str) -> bool {
                use crate::cdata::AuthRequest;

                let addr_port = format!("{}{}", String::from("http://"), addr_port);
                info!(
                    "client_create_user_database attempting to connect {}",
                    addr_port
                );

                let endpoint = tonic::transport::Channel::builder(addr_port.parse().unwrap());
                let channel = endpoint.connect().await.unwrap();

                let mut client = SqlClientClient::new(channel);

                info!("created channel and client");

                // send incorrect login
                let auth = AuthRequest {
                    user_name: String::from("wrong_user"),
                    pw: String::from("123456"),
                    pw_hash: Vec::new(),
                    token: Vec::new(),
                };

                let request = tonic::Request::new(CreateUserDatabaseRequest {
                    authentication: Some(auth),
                    database_name: db_name.to_string(),
                });

                info!("sending request");

                let response = client
                    .create_user_database(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                return response.is_created;
            }
        }

        pub mod enable_coooperative_features {
            #[cfg(test)]
            use crate::cdata::sql_client_client::SqlClientClient;
            #[cfg(test)]
            use crate::cdata::CreateUserDatabaseRequest;
            #[cfg(test)]
            use log::info;
            extern crate futures;
            extern crate tokio;
            #[cfg(test)]
            use crate::test_harness;
            #[cfg(test)]
            use std::sync::mpsc;
            #[cfg(test)]
            use std::{thread, time};

            #[test]
            fn test() {
                let test_db_name: &str = "test_enable_coop.db";
                let (tx, rx) = mpsc::channel();
                let port_num = test_harness::TEST_SETTINGS
                    .lock()
                    .unwrap()
                    .get_next_avail_port();

                let service = crate::get_service_from_config_file();
                let client_address_port =
                    format!("{}{}", String::from("[::1]:"), port_num.to_string());
                let target_client_address_port = client_address_port.clone();
                println!("{:?}", &service);

                service.start();

                info!("starting client at {}", &client_address_port);
                info!("starting client service");

                thread::spawn(move || {
                    let _service = service.start_client_service_at_addr(client_address_port);
                });

                let time = time::Duration::from_secs(5);

                info!("sleeping for 5 seconds...");

                thread::sleep(time);

                thread::spawn(move || {
                    let res = client(test_db_name, &target_client_address_port);
                    tx.send(res).unwrap();
                })
                .join()
                .unwrap();

                let response = rx.try_recv().unwrap();

                println!("create_enable_cooperative_features: got: {}", response);

                assert!(response);
            }

            #[cfg(test)]
            #[tokio::main]
            async fn client(db_name: &str, addr_port: &str) -> bool {
                use crate::cdata::{AuthRequest, EnableCoooperativeFeaturesRequest};

                let addr_port = format!("{}{}", String::from("http://"), addr_port);
                info!(
                    "client_create_enable_cooperative_features attempting to connect {}",
                    addr_port
                );

                let endpoint = tonic::transport::Channel::builder(addr_port.parse().unwrap());
                let channel = endpoint.connect().await.unwrap();
                let mut client = SqlClientClient::new(channel);

                info!("created channel and client");

                let auth = AuthRequest {
                    user_name: String::from("tester"),
                    pw: String::from("123456"),
                    pw_hash: Vec::new(),
                    token: Vec::new(),
                };

                let request = tonic::Request::new(CreateUserDatabaseRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                });

                info!("sending request");

                let response = client
                    .create_user_database(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                assert!(response.is_created);

                let enable_coop_request = tonic::Request::new(EnableCoooperativeFeaturesRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                });

                let coop_response = client
                    .enable_coooperative_features(enable_coop_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", coop_response);
                info!("response back");

                return coop_response.is_successful;
            }
        }

        pub mod create_db_enable_coop_read_write {
            #[cfg(test)]
            use crate::cdata::sql_client_client::SqlClientClient;
            #[cfg(test)]
            use crate::cdata::CreateUserDatabaseRequest;
            #[cfg(test)]
            use log::info;
            extern crate futures;
            extern crate tokio;
            #[cfg(test)]
            use crate::test_harness;
            #[cfg(test)]
            use std::sync::mpsc;
            #[cfg(test)]
            use std::{thread, time};

            #[test]
            pub fn test() {
                let test_db_name: &str = "test_create_db_read_write.db";
                let (tx, rx) = mpsc::channel();
                let port_num = test_harness::TEST_SETTINGS
                    .lock()
                    .unwrap()
                    .get_next_avail_port();

                let service = crate::get_service_from_config_file();
                let client_address_port =
                    format!("{}{}", String::from("[::1]:"), port_num.to_string());
                let target_client_address_port = client_address_port.clone();
                println!("{:?}", &service);

                service.start();

                info!("starting client at {}", &client_address_port);
                info!("starting client service");

                thread::spawn(move || {
                    let _service = service.start_client_service_at_addr(client_address_port);
                });

                let time = time::Duration::from_secs(5);

                info!("sleeping for 5 seconds...");

                thread::sleep(time);

                thread::spawn(move || {
                    let res = client(test_db_name, &target_client_address_port);
                    tx.send(res).unwrap();
                })
                .join()
                .unwrap();

                let response = rx.try_recv().unwrap();

                println!(
                    "create_db_enable_coop_read_write: got: is_error: {}",
                    response
                );

                assert!(!response);
            }

            #[cfg(test)]
            #[tokio::main]
            async fn client(db_name: &str, addr_port: &str) -> bool {
                use crate::{
                    cdata::{
                        AuthRequest, EnableCoooperativeFeaturesRequest, ExecuteReadRequest,
                        ExecuteWriteRequest,
                    },
                    lib::DatabaseType,
                };

                let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

                let addr_port = format!("{}{}", String::from("http://"), addr_port);
                info!(
                    "create_db_enable_coop_read_write attempting to connect {}",
                    addr_port
                );

                let endpoint = tonic::transport::Channel::builder(addr_port.parse().unwrap());
                let channel = endpoint.connect().await.unwrap();
                let mut client = SqlClientClient::new(channel);

                info!("created channel and client");

                let auth = AuthRequest {
                    user_name: String::from("tester"),
                    pw: String::from("123456"),
                    pw_hash: Vec::new(),
                    token: Vec::new(),
                };

                let request = tonic::Request::new(CreateUserDatabaseRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                });

                info!("sending request");

                let response = client
                    .create_user_database(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                assert!(response.is_created);

                let enable_coop_request = tonic::Request::new(EnableCoooperativeFeaturesRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                });

                let coop_response = client
                    .enable_coooperative_features(enable_coop_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", coop_response);
                info!("response back");

                let enable_coop_features = coop_response.is_successful;

                let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

                assert!(enable_coop_features);

                let execute_write_drop_request = tonic::Request::new(ExecuteWriteRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                    sql_statement: drop_table_statement,
                    database_type: database_type,
                });

                let execute_write_drop_reply = client
                    .execute_write(execute_write_drop_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", execute_write_drop_reply);
                info!("response back");

                assert!(execute_write_drop_reply.is_successful);

                let create_table_statement =
                    String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

                let execute_write_create_request = tonic::Request::new(ExecuteWriteRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                    sql_statement: create_table_statement,
                    database_type: database_type,
                });

                let execute_write_create_reply = client
                    .execute_write(execute_write_create_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", execute_write_create_reply);
                info!("response back");

                assert!(execute_write_create_reply.is_successful);

                let add_record_statement =
                    String::from("INSERT INTO EMPLOYEE (Id, Name) VALUES (1, 'Randy');");

                let execute_write_request = tonic::Request::new(ExecuteWriteRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                    sql_statement: add_record_statement,
                    database_type: database_type,
                });

                let execute_write_reply = client
                    .execute_write(execute_write_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", execute_write_reply);
                info!("response back");

                assert!(execute_write_reply.is_successful);

                let read_record_statement = String::from("SELECT Id, Name FROM EMPLOYEE");

                let execute_read_request = tonic::Request::new(ExecuteReadRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                    sql_statement: read_record_statement,
                    database_type: database_type,
                });

                let execute_read_reply = client
                    .execute_read(execute_read_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", execute_read_reply);
                info!("response back");

                let resultsets = &execute_read_reply.results[0];

                return resultsets.is_error;
            }
        }

        pub mod get_set_logical_storage_policy {
            #[cfg(test)]
            use crate::cdata::sql_client_client::SqlClientClient;
            #[cfg(test)]
            use crate::cdata::CreateUserDatabaseRequest;
            #[cfg(test)]
            use crate::lib;
            #[cfg(test)]
            use crate::rcd_enum::LogicalStoragePolicy;
            #[cfg(test)]
            use log::info;
            extern crate futures;
            extern crate tokio;
            #[cfg(test)]
            use crate::test_harness;
            #[cfg(test)]
            use std::sync::mpsc;
            #[cfg(test)]
            use std::{thread, time};

            #[test]
            pub fn test() {
                let test_db_name: &str = "test_create_db_read_write.db";
                let (tx, rx) = mpsc::channel();
                let port_num = test_harness::TEST_SETTINGS
                    .lock()
                    .unwrap()
                    .get_next_avail_port();

                let service = lib::get_service_from_config_file();
                let client_address_port =
                    format!("{}{}", String::from("[::1]:"), port_num.to_string());
                let target_client_address_port = client_address_port.clone();
                println!("{:?}", &service);
                let policy = LogicalStoragePolicy::ParticpantOwned;
                let i_policy = LogicalStoragePolicy::to_u32(policy);

                service.start();

                info!("starting client at {}", &client_address_port);
                info!("starting client service");

                thread::spawn(move || {
                    let _service = service.start_client_service_at_addr(client_address_port);
                });

                let time = time::Duration::from_secs(5);

                info!("sleeping for 5 seconds...");

                thread::sleep(time);

                thread::spawn(move || {
                    let res = client(test_db_name, &target_client_address_port, i_policy);
                    tx.send(res).unwrap();
                })
                .join()
                .unwrap();

                let response = rx.try_recv().unwrap();

                println!(
                    "get_set_logical_storage_policy: got: policy_num: {}",
                    response
                );

                assert_eq!(i_policy, response);
            }

            #[cfg(test)]
            #[tokio::main]
            async fn client(db_name: &str, addr_port: &str, policy_num: u32) -> u32 {
                #[allow(unused_imports)]
                use log::Log;

                use crate::{
                    cdata::{
                        AuthRequest, EnableCoooperativeFeaturesRequest, ExecuteReadRequest,
                        ExecuteWriteRequest, GetLogicalStoragePolicyRequest,
                        SetLogicalStoragePolicyRequest,
                    },
                    lib::DatabaseType,
                };

                let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

                let addr_port = format!("{}{}", String::from("http://"), addr_port);
                info!(
                    "create_db_enable_coop_read_write attempting to connect {}",
                    addr_port
                );

                let endpoint = tonic::transport::Channel::builder(addr_port.parse().unwrap());
                let channel = endpoint.connect().await.unwrap();
                let mut client = SqlClientClient::new(channel);

                info!("created channel and client");

                let auth = AuthRequest {
                    user_name: String::from("tester"),
                    pw: String::from("123456"),
                    pw_hash: Vec::new(),
                    token: Vec::new(),
                };

                let request = tonic::Request::new(CreateUserDatabaseRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                });

                info!("sending request");

                let response = client
                    .create_user_database(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                assert!(response.is_created);

                let enable_coop_request = tonic::Request::new(EnableCoooperativeFeaturesRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                });

                let coop_response = client
                    .enable_coooperative_features(enable_coop_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", coop_response);
                info!("response back");

                let enable_coop_features = coop_response.is_successful;

                let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

                assert!(enable_coop_features);

                let execute_write_drop_request = tonic::Request::new(ExecuteWriteRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                    sql_statement: drop_table_statement,
                    database_type: database_type,
                });

                let execute_write_drop_reply = client
                    .execute_write(execute_write_drop_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", execute_write_drop_reply);
                info!("response back");

                assert!(execute_write_drop_reply.is_successful);

                let create_table_statement =
                    String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

                let execute_write_create_request = tonic::Request::new(ExecuteWriteRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                    sql_statement: create_table_statement,
                    database_type: database_type,
                });

                let execute_write_create_reply = client
                    .execute_write(execute_write_create_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", execute_write_create_reply);
                info!("response back");

                assert!(execute_write_create_reply.is_successful);

                let add_record_statement =
                    String::from("INSERT INTO EMPLOYEE (Id, Name) VALUES (1, 'Randy');");

                let execute_write_request = tonic::Request::new(ExecuteWriteRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                    sql_statement: add_record_statement,
                    database_type: database_type,
                });

                let execute_write_reply = client
                    .execute_write(execute_write_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", execute_write_reply);
                info!("response back");

                assert!(execute_write_reply.is_successful);

                let read_record_statement = String::from("SELECT Id, Name FROM EMPLOYEE");

                let execute_read_request = tonic::Request::new(ExecuteReadRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                    sql_statement: read_record_statement,
                    database_type: database_type,
                });

                let execute_read_reply = client
                    .execute_read(execute_read_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", execute_read_reply);
                info!("response back");

                let resultsets = &execute_read_reply.results[0];

                assert!(!resultsets.is_error);

                let set_logical_policy_request =
                    tonic::Request::new(SetLogicalStoragePolicyRequest {
                        authentication: Some(auth.clone()),
                        database_name: db_name.to_string(),
                        table_name: String::from("EMPLOYEE"),
                        policy_mode: policy_num,
                    });

                let set_policy_reply = client
                    .set_logical_storage_policy(set_logical_policy_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", set_policy_reply);
                info!("response back");

                let get_logical_storage_policy_request =
                    tonic::Request::new(GetLogicalStoragePolicyRequest {
                        authentication: Some(auth.clone()),
                        database_name: db_name.to_string(),
                        table_name: String::from("EMPLOYEE"),
                    });

                let get_policy_reply = client
                    .get_logical_storage_policy(get_logical_storage_policy_request)
                    .await
                    .unwrap()
                    .into_inner();

                println!("RESPONSE={:?}", set_policy_reply);
                info!("response back");

                let i_res_policy = get_policy_reply.policy_mode;
                return i_res_policy;
            }
        }

        pub mod has_table {
            #[cfg(test)]
            use crate::cdata::sql_client_client::SqlClientClient;
            #[cfg(test)]
            use crate::cdata::CreateUserDatabaseRequest;
            #[cfg(test)]
            use crate::lib;
            #[cfg(test)]
            use log::info;
            extern crate futures;
            extern crate tokio;
            #[cfg(test)]
            use crate::test_harness;
            #[cfg(test)]
            use std::sync::mpsc;
            #[cfg(test)]
            use std::{thread, time};

            #[test]
            pub fn test() {
                let test_db_name: &str = "test_create_has_table.db";
                let (tx, rx) = mpsc::channel();
                let port_num = test_harness::TEST_SETTINGS
                    .lock()
                    .unwrap()
                    .get_next_avail_port();

                let service = lib::get_service_from_config_file();
                let client_address_port =
                    format!("{}{}", String::from("[::1]:"), port_num.to_string());
                let target_client_address_port = client_address_port.clone();
                println!("{:?}", &service);

                service.start();

                info!("starting client at {}", &client_address_port);
                info!("starting client service");

                thread::spawn(move || {
                    let _service = service.start_client_service_at_addr(client_address_port);
                });

                let time = time::Duration::from_secs(5);

                info!("sleeping for 5 seconds...");

                thread::sleep(time);

                thread::spawn(move || {
                    let res = client(test_db_name, &target_client_address_port);
                    tx.send(res).unwrap();
                })
                .join()
                .unwrap();

                let response = rx.try_recv().unwrap();

                println!("has table: got: {}", response);

                assert!(response);
            }

            #[cfg(test)]
            #[tokio::main]
            async fn client(db_name: &str, addr_port: &str) -> bool {
                #[allow(unused_imports)]
                use log::Log;

                use crate::{
                    cdata::{
                        AuthRequest, EnableCoooperativeFeaturesRequest, ExecuteWriteRequest,
                        HasTableRequest,
                    },
                    lib::DatabaseType,
                };

                let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

                let addr_port = format!("{}{}", String::from("http://"), addr_port);
                info!("has_table attempting to connect {}", addr_port);

                let endpoint = tonic::transport::Channel::builder(addr_port.parse().unwrap());
                let channel = endpoint.connect().await.unwrap();
                let mut client = SqlClientClient::new(channel);

                info!("created channel and client");

                let auth = AuthRequest {
                    user_name: String::from("tester"),
                    pw: String::from("123456"),
                    pw_hash: Vec::new(),
                    token: Vec::new(),
                };

                let request = tonic::Request::new(CreateUserDatabaseRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                });

                info!("sending request");

                let response = client
                    .create_user_database(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                assert!(response.is_created);

                let enable_coop_request = tonic::Request::new(EnableCoooperativeFeaturesRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                });

                let coop_response = client
                    .enable_coooperative_features(enable_coop_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", coop_response);
                info!("response back");

                let enable_coop_features = coop_response.is_successful;

                let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

                assert!(enable_coop_features);

                let execute_write_drop_request = tonic::Request::new(ExecuteWriteRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                    sql_statement: drop_table_statement,
                    database_type: database_type,
                });

                let execute_write_drop_reply = client
                    .execute_write(execute_write_drop_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", execute_write_drop_reply);
                info!("response back");

                assert!(execute_write_drop_reply.is_successful);

                let create_table_statement =
                    String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

                let execute_write_create_request = tonic::Request::new(ExecuteWriteRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                    sql_statement: create_table_statement,
                    database_type: database_type,
                });

                let execute_write_create_reply = client
                    .execute_write(execute_write_create_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", execute_write_create_reply);
                info!("response back");

                assert!(execute_write_create_reply.is_successful);

                let has_table_request = tonic::Request::new(HasTableRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                    table_name: String::from("EMPLOYEE"),
                });

                let has_table_reply = client
                    .has_table(has_table_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", has_table_reply);
                info!("response back");

                return has_table_reply.has_table;
            }
        }

        pub mod generate_contract {
            #[cfg(test)]
            use crate::cdata::sql_client_client::SqlClientClient;
            #[cfg(test)]
            use crate::cdata::CreateUserDatabaseRequest;
            #[cfg(test)]
            use crate::lib;
            #[cfg(test)]
            use log::info;
            extern crate futures;
            extern crate tokio;
            #[cfg(test)]
            use crate::test_harness;
            #[cfg(test)]
            use std::sync::mpsc;
            #[cfg(test)]
            use std::{thread, time};

            #[test]
            pub fn negative_test() {
                let test_db_name: &str = "test_gen_contract_negative.db";
                let (tx, rx) = mpsc::channel();
                let port_num = test_harness::TEST_SETTINGS
                    .lock()
                    .unwrap()
                    .get_next_avail_port();

                let service = lib::get_service_from_config_file();
                let client_address_port =
                    format!("{}{}", String::from("[::1]:"), port_num.to_string());
                let target_client_address_port = client_address_port.clone();
                println!("{:?}", &service);

                service.start();

                info!("starting client at {}", &client_address_port);
                info!("starting client service");

                thread::spawn(move || {
                    let _service = service.start_client_service_at_addr(client_address_port);
                });

                let time = time::Duration::from_secs(5);

                info!("sleeping for 5 seconds...");

                thread::sleep(time);

                thread::spawn(move || {
                    let res = client_negative(test_db_name, &target_client_address_port);
                    tx.send(res).unwrap();
                })
                .join()
                .unwrap();

                let response = rx.try_recv().unwrap();

                println!("generate_contract_negative: got: {}", response);

                assert!(!response);
            }

            #[cfg(test)]
            #[tokio::main]
            async fn client_negative(db_name: &str, addr_port: &str) -> bool {
                #[allow(unused_imports)]
                use log::Log;

                use crate::{
                    cdata::{
                        AuthRequest, EnableCoooperativeFeaturesRequest, ExecuteWriteRequest,
                        GenerateContractRequest,
                    },
                    lib::DatabaseType,
                    rcd_enum::RemoteDeleteBehavior,
                };

                let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

                let addr_port = format!("{}{}", String::from("http://"), addr_port);
                info!("has_table attempting to connect {}", addr_port);

                let endpoint = tonic::transport::Channel::builder(addr_port.parse().unwrap());
                let channel = endpoint.connect().await.unwrap();
                let mut client = SqlClientClient::new(channel);

                info!("created channel and client");

                let auth = AuthRequest {
                    user_name: String::from("tester"),
                    pw: String::from("123456"),
                    pw_hash: Vec::new(),
                    token: Vec::new(),
                };

                let request = tonic::Request::new(CreateUserDatabaseRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                });

                info!("sending request");

                let response = client
                    .create_user_database(request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", response);
                info!("response back");

                assert!(response.is_created);

                let enable_coop_request = tonic::Request::new(EnableCoooperativeFeaturesRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                });

                let coop_response = client
                    .enable_coooperative_features(enable_coop_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", coop_response);
                info!("response back");

                let enable_coop_features = coop_response.is_successful;

                let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

                assert!(enable_coop_features);

                let execute_write_drop_request = tonic::Request::new(ExecuteWriteRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                    sql_statement: drop_table_statement,
                    database_type: database_type,
                });

                let execute_write_drop_reply = client
                    .execute_write(execute_write_drop_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", execute_write_drop_reply);
                info!("response back");

                assert!(execute_write_drop_reply.is_successful);

                let create_table_statement =
                    String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

                let execute_write_create_request = tonic::Request::new(ExecuteWriteRequest {
                    authentication: Some(auth.clone()),
                    database_name: db_name.to_string(),
                    sql_statement: create_table_statement,
                    database_type: database_type,
                });

                let execute_write_create_reply = client
                    .execute_write(execute_write_create_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", execute_write_create_reply);
                info!("response back");

                assert!(execute_write_create_reply.is_successful);

                let behavior = RemoteDeleteBehavior::Ignore;
                let i_behavior = RemoteDeleteBehavior::to_u32(behavior);

                let generate_contract_request = tonic::Request::new(GenerateContractRequest {
                    authentication: Some(auth.clone()),
                    host_name: String::from("tester"),
                    description: String::from("this is a desc"),
                    database_name: db_name.to_string(),
                    remote_delete_behavior: i_behavior,
                });

                let generate_contract_reply = client
                    .generate_contract(generate_contract_request)
                    .await
                    .unwrap()
                    .into_inner();
                println!("RESPONSE={:?}", generate_contract_reply);
                info!("response back");

                return generate_contract_reply.is_successful;
            }
        }

        #[test]
        fn get_harness_value() {
            let current = crate::test_harness::TEST_SETTINGS
                .lock()
                .unwrap()
                .get_current_port();
            let next = crate::test_harness::TEST_SETTINGS
                .lock()
                .unwrap()
                .get_next_avail_port();
            assert_eq!(current + 1, next);
        }
    }

    pub mod rcd {
        #[allow(unused_imports)]
        use crate::client_srv::SqlClientImpl;
        #[allow(unused_imports)]
        use crate::lib;
        #[allow(unused_imports)]
        use config::Config;
        #[allow(unused_imports)]
        use env_logger::{Builder, Target};
        #[allow(unused_imports)]
        use log::info;
        #[allow(unused_imports)]
        use rusqlite::{Connection, Result};
        #[allow(unused_imports)]
        use std::env;
        #[allow(unused_imports)]
        use std::fs;
        #[allow(unused_imports)]
        use std::path::Path;

        #[cfg(test)]
        /// Attempts to set the log builder for tests
        pub fn init() {
            let mut builder = Builder::from_default_env();
            builder.target(Target::Stdout);
            let _init = builder.is_test(true).try_init();
        }

        #[test]
        /// Attempts to read settings from the Settings.toml
        fn read_settings_from_config() {
            // ARRANGE
            init();
            let rcd_setting = lib::RcdSettings {
                admin_un: String::from("tester"),
                admin_pw: String::from("123456"),
                database_type: lib::DatabaseType::Unknown,
                backing_database_name: String::from(""),
                client_service_addr_port: String::from("[::1]:50051"),
                database_service_addr_port: String::from(""),
            };

            // ACT
            let service = lib::get_service_from_config(rcd_setting);

            // ASSERT
            assert_eq!(
                service.rcd_settings.database_type,
                lib::DatabaseType::Unknown
            );
        }

        #[test]
        /// Attempts to set the backing RCD database name
        fn configure_backing_db() {
            init();
            // to see the output, run the test with the following
            // cargo test -- --nocapture
            // RUST_LOG=debug cargo test -- --nocapture

            // ARRANGE
            let rcd_setting = lib::RcdSettings {
                admin_un: String::from("tester"),
                admin_pw: String::from("123456"),
                database_type: lib::DatabaseType::Sqlite,
                backing_database_name: String::from("rcd_test.db"),
                client_service_addr_port: String::from("[::1]:50051"),
                database_service_addr_port: String::from(""),
            };

            let cwd = env::current_dir().unwrap();
            let db_path = Path::new(&cwd).join(&rcd_setting.backing_database_name);

            if db_path.exists() {
                fs::remove_file(&db_path).unwrap();
            }

            // ACT
            let service = lib::get_service_from_config(rcd_setting);
            service.start();

            // ASSERT
            assert!(db_path.exists());
        }

        #[test]
        /// Attempts to validate the username and pw are hashing correctly
        fn hash() {
            // ARRANGE
            init();

            info!("test_hash: running");

            let cwd = env::current_dir().unwrap();
            let backing_database_name = String::from("test.db");
            let db_path = Path::new(&cwd).join(&backing_database_name);

            if db_path.exists() {
                fs::remove_file(&db_path).unwrap();
            }

            crate::rcd_db::configure(&cwd.to_str().unwrap(), &backing_database_name);

            let db_conn = Connection::open(&db_path).unwrap();

            let un = String::from("tester");
            let pw = String::from("1234");

            // ACT
            crate::rcd_db::create_login(&un, &pw, &db_conn);
            let has_login = crate::rcd_db::has_login(&un, &db_conn).unwrap();

            info!("test_hash: has_login {}", &has_login);

            let is_valid = crate::rcd_db::verify_login(&un, &pw, &db_conn);

            info!("test_hash: is_valid {}", is_valid);

            // ASSERT
            assert!(&has_login);
            assert!(is_valid);
        }

        #[test]
        /// Tests the functionality of getting the next available testing port for the client service
        fn get_harness_value() {
            // ARRANGE, ACT
            let current = crate::test_harness::TEST_SETTINGS
                .lock()
                .unwrap()
                .get_current_port();
            let next = crate::test_harness::TEST_SETTINGS
                .lock()
                .unwrap()
                .get_next_avail_port();

            // ASSERT
            assert_eq!(current + 1, next);
        }

        #[test]
        /// Attempts a negative test of hashing the un and pw to make sure that it can fail
        fn hash_negative() {
            // ARRANGE
            init();
            info!("test_hash_false: running");

            let cwd = env::current_dir().unwrap();
            let backing_database_name = String::from("test.db");
            let db_path = Path::new(&cwd).join(&backing_database_name);

            if db_path.exists() {
                fs::remove_file(&db_path).unwrap();
            }

            crate::rcd_db::configure(&cwd.to_str().unwrap(), &backing_database_name);

            let db_conn = Connection::open(&db_path).unwrap();

            let un = String::from("tester_fail");
            let pw = String::from("1234");

            // ACT
            crate::rcd_db::create_login(&un, &pw, &db_conn);
            let has_login = crate::rcd_db::has_login(&un, &db_conn).unwrap();

            info!("test_hash_false: has_login {}", &has_login);

            let wrong_pw = String::from("43210");
            let is_valid = crate::rcd_db::verify_login(&un, &wrong_pw, &db_conn);

            info!("test_hash_false: is_valid {}", is_valid);

            // ASSERT
            assert!(&has_login);
            assert!(!is_valid);
        }

        #[test]
        /// Attempts to read a value from the Settings.toml file
        fn read_settings_from_file() {
            init();
            let service = lib::get_service_from_config_file();
            let db_type = service.rcd_settings.database_type;
            assert_eq!(db_type, lib::DatabaseType::Sqlite);
        }
    }
}

#[cfg(test)]
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
