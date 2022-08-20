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
        let test_message: &str = "is_online";
        let (tx, rx) = mpsc::channel();

        let root_dir = super::test_harness::get_test_temp_dir(test_message);
        println!("{}", root_dir);

        let service = get_service_from_config_file();
        let client_address_port = service.rcd_settings.client_service_addr_port.clone();
        println!("{:?}", &service);
        service.start_at_dir(&root_dir);

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
    use rcd::get_service_from_config_file;
    use rcd::rcd_sql_client::RcdClient;
    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    fn test() {
        let test_name = "create_user_database_positive";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = super::test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = super::test_harness::get_test_temp_dir(&test_name);
        println!("{}", root_dir);

        let service = get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_client_service_at_addr(client_address_port, root_dir);
        });

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        thread::spawn(move || {
            let res = client(&test_db_name, &target_client_address_port);
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
        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!(
            "client_create_user_database attempting to connect {}",
            addr_port
        );

        let client = RcdClient::new(addr_port, String::from("tester"), String::from("123456"));
        return client.create_user_database(db_name).await.unwrap();
    }

    #[test]
    fn negative_test() {

        let test_name = "create_user_database_negative";
        let test_db_name = format!("{}{}", test_name, ".db");
        
        let (tx, rx) = mpsc::channel();
        let port_num = super::test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = super::test_harness::get_test_temp_dir(&test_name);
        println!("{}", root_dir);

        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_client_service_at_addr(client_address_port, root_dir);
        });

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        thread::spawn(move || {
            let res = negative_client(&test_db_name, &target_client_address_port);
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
        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!(
            "client_create_user_database attempting to connect {}",
            addr_port
        );

        let client = RcdClient::new(
            addr_port,
            String::from("wrong_user"),
            String::from("123456"),
        );
        return client.create_user_database(db_name).await.unwrap();
    }
}

pub mod enable_coooperative_features {
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
        let test_name = "enable_coooperative_features";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = super::test_harness::get_test_temp_dir(test_name);
        println!("{}", root_dir);

        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_client_service_at_addr(client_address_port, root_dir);
        });

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        thread::spawn(move || {
            let res = client(&test_db_name, &target_client_address_port);
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
        use rcd::rcd_sql_client::RcdClient;

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!(
            "client_create_enable_cooperative_features attempting to connect {}",
            addr_port
        );

        let client = RcdClient::new(addr_port, String::from("tester"), String::from("123456"));
        return client.enable_cooperative_features(db_name).await.unwrap();
    }
}

pub mod create_db_enable_coop_read_write {

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
        let test_name = "create_db_enable_coop_read_write";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = super::test_harness::get_test_temp_dir(&test_name);
        println!("{}", root_dir);
        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_client_service_at_addr(client_address_port, root_dir);
        });

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        thread::spawn(move || {
            let res = client(&test_db_name, &target_client_address_port);
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
    #[allow(unused_assignments)]
    async fn client(db_name: &str, addr_port: &str) -> bool {
        use rcd::{rcd_enum::DatabaseType, rcd_sql_client::RcdClient};

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!(
            "create_db_enable_coop_read_write attempting to connect {}",
            addr_port
        );

        let client = RcdClient::new(addr_port, String::from("tester"), String::from("123456"));
        let is_db_created = client.create_user_database(db_name).await.unwrap();

        assert!(is_db_created);

        let enable_coop_features = client.enable_cooperative_features(db_name).await.unwrap();
        let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

        assert!(enable_coop_features);
        let mut execute_write_drop_is_successful = false;
        execute_write_drop_is_successful = client
            .execute_write(db_name, &drop_table_statement, database_type)
            .await
            .unwrap();

        assert!(execute_write_drop_is_successful);

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        let mut execute_write_create_reply_is_successful = false;
        execute_write_create_reply_is_successful = client
            .execute_write(db_name, &create_table_statement, database_type)
            .await
            .unwrap();

        assert!(execute_write_create_reply_is_successful);

        let add_record_statement =
            String::from("INSERT INTO EMPLOYEE (Id, Name) VALUES (1, 'Randy');");

        let mut execute_write_add_record_is_successful = false;
        execute_write_add_record_is_successful = client
            .execute_write(db_name, &add_record_statement, database_type)
            .await
            .unwrap();

        assert!(execute_write_add_record_is_successful);

        let read_record_statement = String::from("SELECT Id, Name FROM EMPLOYEE");
        let read_reply = client
            .execute_read(db_name, &read_record_statement, database_type)
            .await
            .unwrap();

        return read_reply.is_error;
    }
}

pub mod get_set_logical_storage_policy {

    use log::info;
    use rcd::rcd_enum::LogicalStoragePolicy;
    extern crate futures;
    extern crate tokio;
    use crate::test_harness;
    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    pub fn test() {
        let test_name = "get_set_logical_storage_policy";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

            let root_dir = super::test_harness::get_test_temp_dir(&test_name);
            println!("{}", root_dir);            
        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);
        let policy = LogicalStoragePolicy::ParticpantOwned;
        let i_policy = LogicalStoragePolicy::to_u32(policy);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_client_service_at_addr(client_address_port, root_dir);
        });

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        thread::spawn(move || {
            let res = client(&test_db_name, &target_client_address_port, i_policy);
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

        use rcd::{rcd_enum::DatabaseType, rcd_sql_client::RcdClient};

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!(
            "create_db_enable_coop_read_write attempting to connect {}",
            addr_port
        );

        let client = RcdClient::new(addr_port, String::from("tester"), String::from("123456"));

        let create_db_is_successful = client.create_user_database(db_name).await.unwrap();

        assert!(create_db_is_successful);

        let enable_coop_features_is_successful =
            client.enable_cooperative_features(db_name).await.unwrap();

        let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

        assert!(enable_coop_features_is_successful);

        let drop_table_is_successful = client
            .execute_write(db_name, &drop_table_statement, database_type)
            .await
            .unwrap();

        assert!(drop_table_is_successful);

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        let create_table_is_successful = client
            .execute_write(db_name, &create_table_statement, database_type)
            .await
            .unwrap();

        assert!(create_table_is_successful);

        let add_record_statement =
            String::from("INSERT INTO EMPLOYEE (Id, Name) VALUES (1, 'Randy');");

        let execute_write_is_successful = client
            .execute_write(db_name, &add_record_statement, database_type)
            .await
            .unwrap();

        assert!(execute_write_is_successful);

        let read_record_statement = String::from("SELECT Id, Name FROM EMPLOYEE");

        let result = client
            .execute_read(db_name, &read_record_statement, database_type)
            .await
            .unwrap();

        assert!(!result.is_error);

        let _set_policy_is_successful = client
            .set_logical_storage_policy(
                db_name,
                "EMPLOYEE",
                LogicalStoragePolicy::from_i64(policy_num as i64),
            )
            .await
            .unwrap();

        let policy_response = client
            .get_logical_storage_policy(db_name, "EMPLOYEE")
            .await
            .unwrap();

        let i_res_policy = LogicalStoragePolicy::to_u32(policy_response);
        return i_res_policy;
    }
}

pub mod has_table {

    use log::info;
    extern crate futures;
    extern crate tokio;
    use crate::test_harness;
    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    pub fn test() {
        let test_name = "has_table";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = super::test_harness::get_test_temp_dir(test_name);
        println!("{}", root_dir);
        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_client_service_at_addr(client_address_port, root_dir);
        });

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        thread::spawn(move || {
            let res = client(&test_db_name, &target_client_address_port);
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

        use rcd::{rcd_enum::DatabaseType, rcd_sql_client::RcdClient};

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!("has_table attempting to connect {}", addr_port);

        let client = RcdClient::new(addr_port, String::from("tester"), String::from("123456"));

        client.create_user_database(db_name).await.unwrap();
        client.enable_cooperative_features(db_name).await.unwrap();

        let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

        client
            .execute_write(db_name, &drop_table_statement, database_type)
            .await
            .unwrap();

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        client
            .execute_write(db_name, &create_table_statement, database_type)
            .await
            .unwrap();

        return client.has_table(db_name, "EMPLOYEE").await.unwrap();
    }
}

pub mod generate_contract {

    use super::test_harness;
    use log::info;
    use std::sync::mpsc;
    use std::{thread, time};

    #[test]
    pub fn test() {
        let test_name = "generate_contract_positive";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = super::test_harness::get_test_temp_dir(&test_name);
        println!("{}", root_dir);
        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        let cwd = service.cwd();
        test_harness::delete_test_database(&test_db_name, &cwd);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_client_service_at_addr(client_address_port, root_dir);
        });

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        thread::spawn(move || {
            let res = client(&test_db_name, &target_client_address_port);
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
        let test_name = "generate_contract_negative";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = super::test_harness::get_test_temp_dir(test_name);
        println!("{}", root_dir);
        let service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_client_service_at_addr(client_address_port, root_dir);
        });

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        thread::spawn(move || {
            let res = client_negative(&test_db_name, &target_client_address_port);
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
        use rcd::rcd_enum::LogicalStoragePolicy;
        use rcd::rcd_sql_client::RcdClient;
        use rcd::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!("has_table attempting to connect {}", addr_port);

        let client = RcdClient::new(addr_port, String::from("tester"), String::from("123456"));
        client.create_user_database(db_name).await.unwrap();
        client.enable_cooperative_features(db_name).await.unwrap();
        client
            .execute_write(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type)
            .await
            .unwrap();

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        client
            .execute_write(db_name, &create_table_statement, database_type)
            .await
            .unwrap();

        let logical_storage_policy = LogicalStoragePolicy::ParticpantOwned;

        client
            .set_logical_storage_policy(db_name, "EMPLOYEE", logical_storage_policy)
            .await
            .unwrap();

        let behavior = RemoteDeleteBehavior::Ignore;

        return client
            .generate_contract(db_name, "tester", "desc", behavior)
            .await
            .unwrap();
    }

    #[cfg(test)]
    #[tokio::main]
    async fn client_negative(db_name: &str, addr_port: &str) -> bool {
        use rcd::{
            rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior, rcd_sql_client::RcdClient,
        };

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!("has_table attempting to connect {}", addr_port);

        let client = RcdClient::new(addr_port, String::from("tester"), String::from("123456"));
        client.create_user_database(db_name).await.unwrap();
        client.enable_cooperative_features(db_name).await.unwrap();
        client
            .execute_write(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type)
            .await
            .unwrap();

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        client
            .execute_write(db_name, &create_table_statement, database_type)
            .await
            .unwrap();

        let behavior = RemoteDeleteBehavior::Ignore;

        return client
            .generate_contract(db_name, "tester", "desc", behavior)
            .await
            .unwrap();
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
