mod test_harness;

pub mod save_contract {
    use log::info;
    use std::sync::mpsc;
    use std::{thread, time};

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

        let test_name = "save_contract";
        let test_db_name = format!("{}{}", test_name, ".db");

        let (tx, rx) = mpsc::channel();

        let dirs = super::test_harness::get_test_temp_dir_main_and_participant(&test_name);

        let main_addrs = main_service_start(&test_db_name, dirs.1);
        let participant_addrs = participant_service_start(&test_db_name, dirs.2);

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        thread::spawn(move || {
            let res = main_service_client(&test_db_name, &main_addrs.0);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        println!("generate_contract: got: {}", response);

        unimplemented!();
    }

    /// returns a tuple for the addr_port of the client service and the db service
    fn main_service_start(test_db_name: &str, root_dir: String) -> (String, String) {
        let client_port_num = super::test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let db_port_num = super::test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let main_service = rcd::get_service_from_config_file();

        let client_address_port =
            format!("{}{}", String::from("[::1]:"), client_port_num.to_string());
        let target_client_address_port = client_address_port.clone();

        let db_address_port = format!("{}{}", String::from("[::1]:"), db_port_num.to_string());
        let target_db_address_port = db_address_port.clone();

        println!("{:?}", &main_service);

        main_service.start_at_dir(root_dir.as_str());

        let cwd = main_service.cwd();
        super::test_harness::delete_test_database(test_db_name, &cwd);

        info!("starting main client at {}", &client_address_port);
        info!("starting client service");

        let dir = root_dir.clone();

        thread::spawn(move || {
            let d = dir.clone();
            let e = d.clone();
            main_service
                .start_client_service_at_addr(client_address_port, d)
                .unwrap();
            main_service
                .start_db_service_at_addr(db_address_port, e)
                .unwrap();
        });

        return (target_client_address_port, target_db_address_port);
    }

    fn participant_service_start(test_db_name: &str, root_dir: String) -> (String, String) {
        let client_port_num = super::test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let db_port_num = super::test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let participant_service = rcd::get_service_from_config_file();

        let client_address_port =
            format!("{}{}", String::from("[::1]:"), client_port_num.to_string());
        let target_client_address_port = client_address_port.clone();

        let db_address_port = format!("{}{}", String::from("[::1]:"), db_port_num.to_string());
        let target_db_address_port = db_address_port.clone();

        println!("{:?}", &participant_service);

        participant_service.start_at_dir(root_dir.as_str());

        let cwd = participant_service.cwd();
        super::test_harness::delete_test_database(test_db_name, &cwd);

        info!("starting main client at {}", &client_address_port);
        info!("starting client service");

        let dir = root_dir.clone();

        thread::spawn(move || {
            let d = dir.clone();
            let e = d.clone();
            participant_service
                .start_client_service_at_addr(client_address_port, d)
                .unwrap();
            participant_service
                .start_db_service_at_addr(db_address_port, e)
                .unwrap();
        });

        return (target_client_address_port, target_db_address_port);
    }

    #[cfg(test)]
    #[tokio::main]
    async fn main_service_client(db_name: &str, addr_port: &str) -> bool {
        use rcd::rcd_enum::LogicalStoragePolicy;
        use rcd::rcd_sql_client::RcdClient;
        use rcd::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!("main_service_client attempting to connect {}", addr_port);

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
}
