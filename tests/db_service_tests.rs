mod test_harness;

pub mod save_contract {
    use log::info;
    use std::fs;
    use std::path::Path;
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

        let root_dir = super::test_harness::get_test_temp_dir(&test_name);
        let main_path = Path::new(&root_dir).join("main");

        if main_path.exists() {
            fs::remove_dir_all(&main_path).unwrap();
        }

        fs::create_dir_all(&main_path).unwrap();

        let main_dir = main_path.as_os_str().to_str().unwrap();

        let participant_path = Path::new(&root_dir).join("participant");

        if participant_path.exists() {
            fs::remove_dir_all(&participant_path).unwrap();
        }

        fs::create_dir_all(&participant_path).unwrap();

        let participant_dir = participant_path.as_os_str().to_str().unwrap();

        let main_client_addr_port = main_service_start(&test_db_name, main_dir.to_string());
        let participant_client_addr_port =
            participant_service_start(&test_db_name, participant_dir.to_string());

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        thread::spawn(move || {
            let res = main_service_client(&test_db_name, &main_client_addr_port);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        println!("generate_contract: got: {}", response);

        unimplemented!();
    }

    fn main_service_start(test_db_name: &str, root_dir: String) -> String {
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
        println!("{:?}", &main_service);

        let db_address_port = format!("{}{}", String::from("[::1]:"), db_port_num.to_string());

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

        return target_client_address_port;
    }

    fn participant_service_start(test_db_name: &str, root_dir: String) -> String {
        let port_num = super::test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let participant_service = rcd::get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &participant_service);

        participant_service.start_at_dir(root_dir.as_str());

        let cwd = participant_service.cwd();
        super::test_harness::delete_test_database(test_db_name, &cwd);

        info!("starting participant client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service =
                participant_service.start_client_service_at_addr(client_address_port, root_dir);
        });

        return target_client_address_port;
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
