pub mod grpc {
    #[cfg(test)]
    use log::info;
    use rcdx::rcd_service::get_service_from_config_file;
    extern crate futures;
    extern crate tokio;
    #[cfg(test)]
    use crate::test_harness;
    #[cfg(test)]
    use std::sync::mpsc;
    #[cfg(test)]
    use std::thread;

    #[test]
    pub fn test() {
        let test_name = "create_db_enable_coop_read_write";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = test_harness::get_test_temp_dir(test_name);
        println!("{root_dir}");
        let mut service = get_service_from_config_file(None);
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num);
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
            let res = client(&test_db_name, &target_client_address_port);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        println!(
            "create_db_enable_coop_read_write: got: is_error: {response}"
        );

        assert!(!response);

        test_harness::release_port(port_num);
    }

    #[cfg(test)]
    #[tokio::main]
    #[allow(unused_assignments)]
    async fn client(db_name: &str, addr_port: &str) -> bool {
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
            5,
        )
        .await;

        let is_db_created = client.create_user_database(db_name).await.unwrap();

        assert!(is_db_created);

        let enable_coop_features = client.enable_cooperative_features(db_name).await.unwrap();
        let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

        assert!(enable_coop_features);
        let mut execute_write_drop_is_successful = false;
        execute_write_drop_is_successful = client
            .execute_write_at_host(db_name, &drop_table_statement, database_type, "")
            .await
            .unwrap();

        assert!(execute_write_drop_is_successful);

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        let mut execute_write_create_reply_is_successful = false;
        execute_write_create_reply_is_successful = client
            .execute_write_at_host(db_name, &create_table_statement, database_type, "")
            .await
            .unwrap();

        assert!(execute_write_create_reply_is_successful);

        let add_record_statement =
            String::from("INSERT INTO EMPLOYEE (Id, Name) VALUES (1, 'Randy');");

        let mut execute_write_add_record_is_successful = false;
        execute_write_add_record_is_successful = client
            .execute_write_at_host(db_name, &add_record_statement, database_type, "")
            .await
            .unwrap();

        assert!(execute_write_add_record_is_successful);

        let read_record_statement = String::from("SELECT Id, Name FROM EMPLOYEE");
        let read_reply = client
            .execute_read_at_host(db_name, &read_record_statement, database_type)
            .await
            .unwrap();

        read_reply.is_error
    }
}

pub mod http {
    #[cfg(test)]
    use log::info;
    use rcdx::rcd_service::{get_service_from_config_file, RcdService};
    extern crate futures;
    extern crate tokio;
    #[cfg(test)]
    use crate::test_harness;
    #[cfg(test)]
    use std::sync::mpsc;
    #[cfg(test)]
    use std::thread;

    #[test]
    pub fn test() {
        let test_name = "create_db_enable_coop_read_write_http";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = test_harness::get_test_temp_dir(test_name);
        println!("{root_dir}");
        let mut service = get_service_from_config_file(None);
        let client_address_port = format!("{}{}", String::from("127.0.0.1:"), port_num);
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            service.start_http_at_addr_and_dir("127.0.0.1".to_string(), port_num as u16, root_dir);
        });

        test_harness::sleep_test();

        thread::spawn(move || {
            let res = client(&test_db_name, &target_client_address_port, port_num);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        println!(
            "create_db_enable_coop_read_write: got: is_error: {response}"
        );

        assert!(!response);

        test_harness::release_port(port_num);
        RcdService::shutdown_http("127.0.0.1".to_string(), port_num);
    }

    #[cfg(test)]
    #[tokio::main]
    async fn client(db_name: &str, addr_port: &str, port_num: u32) -> bool {
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
            5,
            "127.0.0.1".to_string(),
            port_num,
        );

        let is_db_created = client.create_user_database(db_name).await.unwrap();

        assert!(is_db_created);

        let enable_coop_features = client.enable_cooperative_features(db_name).await.unwrap();
        let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

        assert!(enable_coop_features);

        let execute_write_drop_is_successful = client
            .execute_write_at_host(db_name, &drop_table_statement, database_type, "")
            .await
            .unwrap();

        assert!(execute_write_drop_is_successful);

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        let execute_write_create_reply_is_successful = client
            .execute_write_at_host(db_name, &create_table_statement, database_type, "")
            .await
            .unwrap();

        assert!(execute_write_create_reply_is_successful);

        let add_record_statement =
            String::from("INSERT INTO EMPLOYEE (Id, Name) VALUES (1, 'Randy');");

        let execute_write_add_record_is_successful = client
            .execute_write_at_host(db_name, &add_record_statement, database_type, "")
            .await
            .unwrap();

        assert!(execute_write_add_record_is_successful);

        let read_record_statement = String::from("SELECT Id, Name FROM EMPLOYEE");
        let read_reply = client
            .execute_read_at_host(db_name, &read_record_statement, database_type)
            .await
            .unwrap();

        read_reply.is_error
    }
}
