mod test_harness;

pub mod save_contract {
    use crate::test_harness::ServiceAddr;
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

        let main_addrs = super::test_harness::start_service(&test_db_name, dirs.1);
        let participant_addrs = super::test_harness::start_service(&test_db_name, dirs.2);

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        thread::spawn(move || {
            let res = main_service_client(&test_db_name, main_addrs.0, participant_addrs.1);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        println!("generate_contract: got: {}", response);

        unimplemented!();
    }

    #[cfg(test)]
    #[tokio::main]
    #[allow(unused_variables)]
    async fn main_service_client(
        db_name: &str,
        main_client_addr: ServiceAddr,
        participant_db_addr: ServiceAddr,
    ) -> bool {
        use rcd::rcd_enum::LogicalStoragePolicy;
        use rcd::rcd_sql_client::RcdClient;
        use rcd::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        info!(
            "main_service_client attempting to connect {}",
            main_client_addr.to_full_string_with_http()
        );

        let client = RcdClient::new(
            main_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
        );
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

        client
            .generate_contract(db_name, "tester", "desc", behavior)
            .await
            .unwrap();

        client
            .add_participant(
                db_name,
                "participant",
                &participant_db_addr.ip4_addr,
                participant_db_addr.port,
            )
            .await
            .unwrap();

        return client
            .send_participant_contract(db_name, "participant")
            .await
            .unwrap();
    }

    #[cfg(test)]
    #[tokio::main]
    #[allow(dead_code, unused_variables)]
    async fn participant_service_client(
        db_name: &str,
        participant_client_addr: ServiceAddr,
    ) -> bool {
        use rcd::rcd_enum::DatabaseType;
        use rcd::rcd_sql_client::RcdClient;

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        info!(
            "main_service_client attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let client = RcdClient::new(
            participant_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
        );

        unimplemented!();
    }
}
