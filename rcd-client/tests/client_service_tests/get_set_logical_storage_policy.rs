pub mod grpc {

    use log::{info, debug};
    use rcdx::rcd_service::get_service_from_config_file;
    extern crate futures;
    extern crate tokio;
    use crate::test_harness;
    use rcd_enum::logical_storage_policy::LogicalStoragePolicy;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    pub fn test() {
        test_harness::init_log_to_screen(log::LevelFilter::Info);
        
        let test_name = "get_set_logical_storage_policy_grpc";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = test_harness::get_test_temp_dir(test_name);
        debug!("{root_dir}");
        let mut service = get_service_from_config_file(None);
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num);
        let target_client_address_port = client_address_port.clone();
        debug!("{:?}", &service);
        let policy = LogicalStoragePolicy::ParticpantOwned;
        let i_policy = LogicalStoragePolicy::to_u32(policy);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_grpc_client_service_at_addr(client_address_port, root_dir);
        });

        test_harness::sleep_test();

        thread::spawn(move || {
            let res = client(&test_db_name, &target_client_address_port, i_policy);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        debug!("get_set_logical_storage_policy: got: policy_num: {response}");

        assert_eq!(i_policy, response);

        test_harness::release_port(port_num);
    }

    #[cfg(test)]
    #[tokio::main]
    async fn client(db_name: &str, addr_port: &str, policy_num: u32) -> u32 {
        #[allow(unused_imports)]
        use log::Log;
        use rcd_client::RcdClient;
        use rcd_enum::database_type::DatabaseType;

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!(
            "create_db_enable_coop_read_write attempting to connect {}",
            addr_port
        );

        let mut client = RcdClient::new_grpc_client(
            addr_port,
            String::from("tester"),
            String::from("123456"),
            60,
        )
        .await;

        let create_db_is_successful = client.create_user_database(db_name).await.unwrap();

        assert!(create_db_is_successful);

        let enable_coop_features_is_successful =
            client.enable_cooperative_features(db_name).await.unwrap();

        let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

        assert!(enable_coop_features_is_successful);

        let drop_table_is_successful = client
            .execute_write_at_host(db_name, &drop_table_statement, database_type, "")
            .await
            .unwrap();

        assert!(drop_table_is_successful);

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        let create_table_is_successful = client
            .execute_write_at_host(db_name, &create_table_statement, database_type, "")
            .await
            .unwrap();

        assert!(create_table_is_successful);

        let add_record_statement =
            String::from("INSERT INTO EMPLOYEE (Id, Name) VALUES (1, 'Randy');");

        let execute_write_is_successful = client
            .execute_write_at_host(db_name, &add_record_statement, database_type, "")
            .await
            .unwrap();

        assert!(execute_write_is_successful);

        let read_record_statement = String::from("SELECT Id, Name FROM EMPLOYEE");

        let result = client
            .execute_read_at_host(db_name, &read_record_statement, database_type)
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

        LogicalStoragePolicy::to_u32(policy_response)
    }
}

pub mod http {

    use log::{info, debug};

    use rcd_enum::logical_storage_policy::LogicalStoragePolicy;

    use rcdx::rcd_service::{get_service_from_config_file, RcdService};
    extern crate futures;
    extern crate tokio;
    use crate::test_harness;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    pub fn test() {
        let test_name = "get_set_logical_storage_policy_http";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = test_harness::get_test_temp_dir(test_name);
        debug!("{root_dir}");
        let mut service = get_service_from_config_file(None);
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num);
        let target_client_address_port = client_address_port.clone();
        debug!("{:?}", &service);
        let policy = LogicalStoragePolicy::ParticpantOwned;
        let i_policy = LogicalStoragePolicy::to_u32(policy);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            service.start_http_at_addr_and_dir("127.0.0.1".to_string(), port_num as u16, root_dir);
        });

        test_harness::sleep_test();

        thread::spawn(move || {
            let res = client(
                &test_db_name,
                &target_client_address_port,
                i_policy,
                port_num,
            );
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        debug!("get_set_logical_storage_policy: got: policy_num: {response}");

        assert_eq!(i_policy, response);

        test_harness::release_port(port_num);
        RcdService::shutdown_http("127.0.0.1".to_string(), port_num);
    }

    #[cfg(test)]
    #[tokio::main]
    async fn client(db_name: &str, addr_port: &str, policy_num: u32, port: u32) -> u32 {
        #[allow(unused_imports)]
        use log::Log;
        use rcd_client::RcdClient;
        use rcd_enum::database_type::DatabaseType;

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!(
            "create_db_enable_coop_read_write attempting to connect {}",
            addr_port
        );

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            60,
            "127.0.0.1".to_string(),
            port,
        );

        let create_db_is_successful = client.create_user_database(db_name).await.unwrap();

        assert!(create_db_is_successful);

        let enable_coop_features_is_successful =
            client.enable_cooperative_features(db_name).await.unwrap();

        let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

        assert!(enable_coop_features_is_successful);

        let drop_table_is_successful = client
            .execute_write_at_host(db_name, &drop_table_statement, database_type, "")
            .await
            .unwrap();

        assert!(drop_table_is_successful);

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        let create_table_is_successful = client
            .execute_write_at_host(db_name, &create_table_statement, database_type, "")
            .await
            .unwrap();

        assert!(create_table_is_successful);

        let add_record_statement =
            String::from("INSERT INTO EMPLOYEE (Id, Name) VALUES (1, 'Randy');");

        let execute_write_is_successful = client
            .execute_write_at_host(db_name, &add_record_statement, database_type, "")
            .await
            .unwrap();

        assert!(execute_write_is_successful);

        let read_record_statement = String::from("SELECT Id, Name FROM EMPLOYEE");

        let result = client
            .execute_read_at_host(db_name, &read_record_statement, database_type)
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

        LogicalStoragePolicy::to_u32(policy_response)
    }
}
