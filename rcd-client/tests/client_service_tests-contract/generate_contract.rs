pub mod grpc {
    use log::info;
    use rcdx::rcd_service::get_service_from_config_file;
    use std::sync::mpsc;
    use std::{thread};

    use crate::test_harness;

    #[test]
    pub fn test() {
        let test_name = "generate_contract_positive_grpc";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = test_harness::get_test_temp_dir(&test_name);
        println!("{}", root_dir);
        let mut service = get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        let cwd = service.cwd();
        test_harness::delete_test_database(&test_db_name, &cwd);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_grpc_client_service_at_addr(client_address_port, root_dir);
        });

       test_harness::sleep_test();

        thread::spawn(move || {
            let res = client(&test_db_name, &target_client_address_port);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        println!("generate_contract: got: {}", response);

        assert!(response);

        test_harness::release_port(port_num);
    }

    #[test]
    pub fn negative_test() {
        let test_name = "generate_contract_negative_gprc";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = test_harness::get_test_temp_dir(test_name);
        println!("{}", root_dir);
        let mut service = get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_grpc_client_service_at_addr(client_address_port, root_dir);
        });

       test_harness::sleep_test();

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
        use rcd_client::RcdClient;
        use rcd_common::rcd_enum::LogicalStoragePolicy;
        use rcd_common::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!("has_table attempting to connect {}", addr_port);

        let mut client = RcdClient::new_grpc_client(
            addr_port,
            String::from("tester"),
            String::from("123456"),
            5,
        ).await;

        client.create_user_database(db_name).await.unwrap();
        client.enable_cooperative_features(db_name).await.unwrap();
        client
            .execute_write_at_host(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type, "")
            .await
            .unwrap();

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        client
            .execute_write_at_host(db_name, &create_table_statement, database_type, "")
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
        use rcd_client::RcdClient;
        use rcd_common::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!("has_table attempting to connect {}", addr_port);

        let mut client = RcdClient::new_grpc_client(
            addr_port,
            String::from("tester"),
            String::from("123456"),
            5,
        ).await;
        
        client.create_user_database(db_name).await.unwrap();
        client.enable_cooperative_features(db_name).await.unwrap();
        client
            .execute_write_at_host(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type, "")
            .await
            .unwrap();

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        client
            .execute_write_at_host(db_name, &create_table_statement, database_type, "")
            .await
            .unwrap();

        let behavior = RemoteDeleteBehavior::Ignore;

        return client
            .generate_contract(db_name, "tester", "desc", behavior)
            .await
            .unwrap();
    }
}

pub mod http {
    use log::info;
    use rcdx::rcd_service::{get_service_from_config_file, RcdService};
    use std::sync::mpsc;
    use std::{thread};

    use crate::test_harness;

    #[test]
    pub fn test() {
        let test_name = "generate_contract_positive_http";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = test_harness::get_test_temp_dir(&test_name);
        println!("{}", root_dir);
        let mut service = get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        let cwd = service.cwd();
        test_harness::delete_test_database(&test_db_name, &cwd);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_http_at_addr_and_dir(
                "127.0.0.1".to_string(),
                port_num as u16,
                root_dir,
            );
        });

       test_harness::sleep_test();

        thread::spawn(move || {
            let res = client(&test_db_name, &target_client_address_port, port_num);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        println!("generate_contract: got: {}", response);

        assert!(response);

        test_harness::release_port(port_num);
        RcdService::shutdown_http("127.0.0.1".to_string(), port_num);
    }

    #[test]
    pub fn negative_test() {
        let test_name = "generate_contract_negative_http";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = test_harness::get_test_temp_dir(test_name);
        println!("{}", root_dir);
        let mut service = get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_http_at_addr_and_dir(
                "127.0.0.1".to_string(),
                port_num as u16,
                root_dir,
            );
        });

        test_harness::sleep_test();

        thread::spawn(move || {
            let res = client_negative(&test_db_name, &target_client_address_port, port_num);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        println!("generate_contract_negative: got: {}", response);

        assert!(!response);
        RcdService::shutdown_http("127.0.0.1".to_string(), port_num);
    }

    #[cfg(test)]
    #[tokio::main]
    async fn client(db_name: &str, addr_port: &str, port_num: u32) -> bool {
        use rcd_client::RcdClient;
        use rcd_common::rcd_enum::LogicalStoragePolicy;
        use rcd_common::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!("has_table attempting to connect {}", addr_port);

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            "127.0.0.1".to_string(),
            port_num,
        );
        client.create_user_database(db_name).await.unwrap();
        client.enable_cooperative_features(db_name).await.unwrap();
        client
            .execute_write_at_host(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type, "")
            .await
            .unwrap();

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        client
            .execute_write_at_host(db_name, &create_table_statement, database_type, "")
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
    async fn client_negative(db_name: &str, addr_port: &str, port_num: u32) -> bool {
        use rcd_client::RcdClient;
        use rcd_common::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!("has_table attempting to connect {}", addr_port);

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            "127.0.0.1".to_string(),
            port_num,
        );
        client.create_user_database(db_name).await.unwrap();
        client.enable_cooperative_features(db_name).await.unwrap();
        client
            .execute_write_at_host(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type, "")
            .await
            .unwrap();

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        client
            .execute_write_at_host(db_name, &create_table_statement, database_type, "")
            .await
            .unwrap();

        let behavior = RemoteDeleteBehavior::Ignore;

        return client
            .generate_contract(db_name, "tester", "desc", behavior)
            .await
            .unwrap();
    }
}
