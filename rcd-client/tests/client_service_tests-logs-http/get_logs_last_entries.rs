
pub mod http {

    use log::{info, debug};
    use rcdx::rcd_service::{get_service_from_config_file, RcdService};
    extern crate futures;
    extern crate tokio;
    use crate::test_harness;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    pub fn test() {
        let test_name = "get_logs_http";
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

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            service.start_http_at_addr_and_dir("127.0.0.1".to_string(), port_num as u16, root_dir);
            test_harness::sleep_test();
        })
        .join()
        .unwrap();

        thread::spawn(move || {
            let res = client(&test_db_name, &target_client_address_port, port_num);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        debug!("has logs: got: {response}");

        assert!(response);

        test_harness::release_port(port_num);
        RcdService::shutdown_http("127.0.0.1".to_string(), port_num);
    }

    #[cfg(test)]
    #[tokio::main]
    async fn client(db_name: &str, addr_port: &str, port: u32) -> bool {
        #[allow(unused_imports)]
        use log::Log;
        use rcd_client::RcdClient;

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);
        use rcd_enum::database_type::DatabaseType;
        use rcd_enum::logical_storage_policy::LogicalStoragePolicy;
        use rcd_enum::remote_delete_behavior::RemoteDeleteBehavior;

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!("has_table attempting to connect {}", addr_port);

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            60,
            "127.0.0.1".to_string(),
            port,
        );

        client.create_user_database(db_name).await.unwrap();
        client.enable_cooperative_features(db_name).await.unwrap();

        let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

        client
            .execute_write_at_host(db_name, &drop_table_statement, database_type, "")
            .await
            .unwrap();

        let create_table_statement =
            String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

        client
            .execute_write_at_host(db_name, &create_table_statement, database_type, "")
            .await
            .unwrap();

        let logical_storage_policy = LogicalStoragePolicy::HostOnly;

        client
            .set_logical_storage_policy(db_name, "EMPLOYEE", logical_storage_policy)
            .await
            .unwrap();

        let behavior = RemoteDeleteBehavior::Ignore;

        client
            .generate_contract(db_name, "tester", "desc", behavior)
            .await
            .unwrap();

        client
            .execute_write_at_host(
                db_name,
                "INSERT INTO EMPLOYEE ( Id, Name ) VALUES ( 1234, 'Rando');",
                DatabaseType::to_u32(DatabaseType::Sqlite),
                "",
            )
            .await
            .unwrap();

        client.has_table(db_name, "EMPLOYEE").await.unwrap();

        let logs = client.get_last_log_entries(5).await.unwrap().logs;
        !logs.is_empty()
    }
}
