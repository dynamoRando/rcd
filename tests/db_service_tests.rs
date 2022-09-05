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
        let custom_contract_description = String::from("This is a custom description from test");

        let (tx_main, rx_main) = mpsc::channel();
        let (tx_participant, rx_participant) = mpsc::channel();

        let dirs = super::test_harness::get_test_temp_dir_main_and_participant(&test_name);

        let main_addrs = super::test_harness::start_service(&test_db_name, dirs.1);
        let participant_addrs = super::test_harness::start_service(&test_db_name, dirs.2);

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        let main_contract_desc = custom_contract_description.clone();
        let participant_contract_desc = custom_contract_description.clone();
        let main_db_name = test_db_name.clone();
        let participant_db_name = test_db_name.clone();

        thread::spawn(move || {
            let res = main_service_client(
                &main_db_name,
                main_addrs.0,
                participant_addrs.1,
                main_contract_desc,
            );
            tx_main.send(res).unwrap();
        })
        .join()
        .unwrap();

        let sent_participant_contract = rx_main.try_recv().unwrap();
        println!(
            "send_participant_contract: got: {}",
            sent_participant_contract
        );

        assert!(sent_participant_contract);

        thread::spawn(move || {
            let res = participant_service_client(
                &participant_db_name,
                participant_addrs.0,
                participant_contract_desc,
            );
            tx_participant.send(res).unwrap();
        })
        .join()
        .unwrap();

        let participant_got_contract = rx_participant.try_recv().unwrap();
        println!(
            "participant_got_contract: got: {}",
            participant_got_contract
        );

        assert!(participant_got_contract);
    }

    #[cfg(test)]
    #[tokio::main]
    #[allow(unused_variables)]
    async fn main_service_client(
        db_name: &str,
        main_client_addr: ServiceAddr,
        participant_db_addr: ServiceAddr,
        contract_desc: String,
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
            .generate_contract(db_name, "tester", &contract_desc, behavior)
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
        contract_desc: String,
    ) -> bool {
        use rcd::rcd_enum::DatabaseType;
        use rcd::rcd_sql_client::RcdClient;

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);
        let mut has_contract = false;

        info!(
            "main_service_client attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let client = RcdClient::new(
            participant_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
        );

        let pending_contracts = client.view_pending_contracts().await.unwrap();

        for contract in &pending_contracts {
            if contract.description == contract_desc {
                has_contract = true;
                break;
            }
        }

        return has_contract;
    }
}

pub mod accept_contract {
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

        let test_name = "save_accept_contract";
        let test_db_name = format!("{}{}", test_name, ".db");
        let custom_contract_description = String::from("This is a custom description from test");

        let (tx_main, rx_main) = mpsc::channel();
        let (tx_participant, rx_participant) = mpsc::channel();

        let dirs = super::test_harness::get_test_temp_dir_main_and_participant(&test_name);

        let main_addrs = super::test_harness::start_service(&test_db_name, dirs.1);
        let participant_addrs = super::test_harness::start_service(&test_db_name, dirs.2);

        let time = time::Duration::from_secs(5);

        info!("sleeping for 5 seconds...");

        thread::sleep(time);

        let main_contract_desc = custom_contract_description.clone();
        let participant_contract_desc = custom_contract_description.clone();
        let main_db_name = test_db_name.clone();
        let participant_db_name = test_db_name.clone();

        thread::spawn(move || {
            let res = main_service_client(
                &main_db_name,
                main_addrs.0,
                participant_addrs.1,
                main_contract_desc,
            );
            tx_main.send(res).unwrap();
        })
        .join()
        .unwrap();

        let sent_participant_contract = rx_main.try_recv().unwrap();
        println!(
            "send_participant_contract: got: {}",
            sent_participant_contract
        );

        assert!(sent_participant_contract);

        thread::spawn(move || {
            let res = participant_service_client(
                &participant_db_name,
                participant_addrs.0,
                participant_contract_desc,
            );
            tx_participant.send(res).unwrap();
        })
        .join()
        .unwrap();

        let participant_accepted_contract = rx_participant.try_recv().unwrap();
        println!(
            "participant_accpeted_contract: got: {}",
            participant_accepted_contract
        );

        assert!(participant_accepted_contract);
    }

    #[cfg(test)]
    #[tokio::main]
    #[allow(unused_variables)]
    async fn main_service_client(
        db_name: &str,
        main_client_addr: ServiceAddr,
        participant_db_addr: ServiceAddr,
        contract_desc: String,
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
            .generate_contract(db_name, "tester", &contract_desc, behavior)
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
        contract_desc: String,
    ) -> bool {
        use rcd::rcd_enum::DatabaseType;
        use rcd::rcd_sql_client::RcdClient;

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);
        let mut has_contract = false;

        info!(
            "main_service_client attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let client = RcdClient::new(
            participant_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
        );

        let pending_contracts = client.view_pending_contracts().await.unwrap();

        for contract in &pending_contracts {
            if contract.description == contract_desc {
                has_contract = true;
                break;
            }
        }

        let mut accepted_contract = false;

        if has_contract {
            accepted_contract = client.accept_pending_contract("tester").await.unwrap();
        }

        return accepted_contract;
    }
}
