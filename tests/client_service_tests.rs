#[path = "test_harness.rs"]
mod test_harness;

pub mod is_online {

    use log::info;
    use rcd::cdata::sql_client_client::SqlClientClient;
    use rcd::cdata::TestRequest;
    use rcd::get_service_from_config_file;
    use std::sync::mpsc;
    use std::{thread, time};

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

        let service = get_service_from_config_file();
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

    use log::info;
    use rcd::cdata::sql_client_client::SqlClientClient;
    use rcd::get_service_from_config_file;
    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    fn test() {
        let test_db_name: &str = "test_create_user_db.db";
        let (tx, rx) = mpsc::channel();
        let port_num = super::test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let service = get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
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
        use rcd::cdata::AuthRequest;
        use rcd::cdata::CreateUserDatabaseRequest;

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
        let port_num = super::test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
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
        use rcd::cdata::AuthRequest;
        use rcd::cdata::CreateUserDatabaseRequest;

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
    use log::info;
    #[cfg(test)]
    use rcd::cdata::sql_client_client::SqlClientClient;
    #[cfg(test)]
    use rcd::cdata::CreateUserDatabaseRequest;
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

        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
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
        use rcd::cdata::{AuthRequest, EnableCoooperativeFeaturesRequest};

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
    use log::info;
    use rcd::cdata::sql_client_client::SqlClientClient;
    use rcd::cdata::CreateUserDatabaseRequest;
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

        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
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
        use rcd::{
            cdata::{
                AuthRequest, EnableCoooperativeFeaturesRequest, ExecuteReadRequest,
                ExecuteWriteRequest,
            },
            rcd_enum::DatabaseType,
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

    use log::info;
    use rcd::cdata::sql_client_client::SqlClientClient;
    use rcd::cdata::CreateUserDatabaseRequest;
    use rcd::rcd_enum::LogicalStoragePolicy;
    extern crate futures;
    extern crate tokio;
    use crate::test_harness;
    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    pub fn test() {
        let test_db_name: &str = "test_create_db_read_write.db";
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
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

        use rcd::{
            cdata::{
                AuthRequest, EnableCoooperativeFeaturesRequest, ExecuteReadRequest,
                ExecuteWriteRequest, GetLogicalStoragePolicyRequest,
                SetLogicalStoragePolicyRequest,
            },
            rcd_enum::DatabaseType,
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

        let set_logical_policy_request = tonic::Request::new(SetLogicalStoragePolicyRequest {
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

    use log::info;
    use rcd::cdata::sql_client_client::SqlClientClient;
    use rcd::cdata::CreateUserDatabaseRequest;
    extern crate futures;
    extern crate tokio;
    use crate::test_harness;
    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    pub fn test() {
        let test_db_name: &str = "test_create_has_table.db";
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
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

        use rcd::{
            cdata::{
                AuthRequest, EnableCoooperativeFeaturesRequest, ExecuteWriteRequest,
                HasTableRequest,
            },
            rcd_enum::DatabaseType,
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

    use super::test_harness;
    use log::info;
    use rcd::cdata::sql_client_client::SqlClientClient;
    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    pub fn test() {
        let test_db_name: &str = "test_gen_contract_positive.db";
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start();

        let cwd = service.cwd();
        test_harness::delete_test_database(test_db_name, &cwd);

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

        println!("generate_contract: got: {}", response);

        assert!(response);
    }

    #[test]
    pub fn negative_test() {
        let test_db_name: &str = "test_gen_contract_negative.db";
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
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
    async fn client(db_name: &str, addr_port: &str) -> bool {
        use rcd::cdata::{CreateUserDatabaseRequest, SetLogicalStoragePolicyRequest};
        use rcd::rcd_enum::LogicalStoragePolicy;
        use rcd::{
            cdata::{
                AuthRequest, EnableCoooperativeFeaturesRequest, ExecuteWriteRequest,
                GenerateContractRequest,
            },
            rcd_enum::DatabaseType,
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

        let logical_storage_policy = LogicalStoragePolicy::ParticpantOwned;

        let set_logical_storage_policy_request =
            tonic::Request::new(SetLogicalStoragePolicyRequest {
                authentication: Some(auth.clone()),
                database_name: db_name.to_string(),
                table_name: String::from("EMPLOYEE"),
                policy_mode: LogicalStoragePolicy::to_u32(logical_storage_policy),
            });

        let set_logical_storage_policy_reply = client
            .set_logical_storage_policy(set_logical_storage_policy_request)
            .await
            .unwrap()
            .into_inner();

        println!("RESPONSE={:?}", set_logical_storage_policy_reply);
        info!("response back");

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

    #[cfg(test)]
    #[tokio::main]
    async fn client_negative(db_name: &str, addr_port: &str) -> bool {
        use rcd::cdata::CreateUserDatabaseRequest;
        use rcd::{
            cdata::{
                AuthRequest, EnableCoooperativeFeaturesRequest, ExecuteWriteRequest,
                GenerateContractRequest,
            },
            rcd_enum::DatabaseType,
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
