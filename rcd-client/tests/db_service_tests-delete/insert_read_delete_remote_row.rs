pub mod grpc {

    use crate::test_harness::{self, ServiceAddr};
    use log::{debug, info};
    use rcd_client::RcdClient;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn test() {
        let test_name = "add_read_delete_remote_gprc";
        let test_db_name = format!("{}{}", test_name, ".db");
        let custom_contract_description = String::from("add read delete remote row");

        let (tx_main, rx_main) = mpsc::channel();
        let (tx_participant, rx_participant) = mpsc::channel();
        let (tx_main_write, rx_main_read) = mpsc::channel();

        let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);
        let main_test_config = test_harness::grpc::start_service_with_grpc(&test_db_name, dirs.main_dir);
        let participant_test_config =
            test_harness::grpc::start_service_with_grpc(&test_db_name, dirs.participant_dir);

        test_harness::sleep_test();

        {
            let main_db_name = test_db_name.clone();
            let main_client_addr = main_test_config.client_address.clone();
            let participant_db_addr = participant_test_config.database_address.clone();
            let main_contract_desc = custom_contract_description.clone();
            thread::spawn(move || {
                let res = main_service_client(
                    &main_db_name,
                    &main_client_addr,
                    &participant_db_addr,
                    &main_contract_desc,
                );
                tx_main.send(res).unwrap();
            })
            .join()
            .unwrap();
        }

        let sent_participant_contract = rx_main.try_recv().unwrap();
        debug!("send_participant_contract: got: {sent_participant_contract}");

        assert!(sent_participant_contract);

        {
            let participant_client_addr = participant_test_config.client_address.clone();
            let participant_contract_desc = custom_contract_description;
            thread::spawn(move || {
                let res = participant_service_client(
                    &participant_client_addr,
                    &participant_contract_desc,
                );
                tx_participant.send(res).unwrap();
            })
            .join()
            .unwrap();
        }

        let participant_accepted_contract = rx_participant.try_recv().unwrap();
        debug!("participant_accpeted_contract: got: {participant_accepted_contract}");

        assert!(participant_accepted_contract);

        {
            let main_db_name_write = test_db_name.clone();
            let main_client_addr = main_test_config.client_address.clone();
            thread::spawn(move || {
                let res = main_execute_coop_write_and_read(&main_db_name_write, &main_client_addr);
                tx_main_write.send(res).unwrap();
            })
            .join()
            .unwrap();
        }

        let write_and_read_is_successful = rx_main_read.try_recv().unwrap();

        assert!(write_and_read_is_successful);

        test_harness::grpc::shutdown_grpc_test(&main_test_config, &participant_test_config);
    }

    #[cfg(test)]
    #[tokio::main]
    async fn main_service_client(
        db_name: &str,
        main_client_addr: &ServiceAddr,
        participant_db_addr: &ServiceAddr,
        contract_desc: &str,
    ) -> bool {
        use rcd_client::RcdClient;
        use rcd_enum::database_type::DatabaseType;
        use rcd_enum::logical_storage_policy::LogicalStoragePolicy;
        use rcd_enum::remote_delete_behavior::RemoteDeleteBehavior;
        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        info!(
            "main_service_client attempting to connect {}",
            main_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_grpc_client(
            main_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            60,
        )
        .await;

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
                "".to_string(),
                0,
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
    async fn main_execute_coop_write_and_read(
        db_name: &str,
        main_client_addr: &ServiceAddr,
    ) -> bool {
        use rcd_enum::database_type::DatabaseType;

        let mut client = RcdClient::new_grpc_client(
            main_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            60,
        )
        .await;

        client
            .execute_cooperative_write_at_host(
                db_name,
                "INSERT INTO EMPLOYEE ( Id, Name ) VALUES ( 999, 'ASDF');",
                "participant",
                "",
            )
            .await
            .unwrap();

        let data = client
            .execute_read_at_host(
                db_name,
                "SELECT ID FROM EMPLOYEE",
                DatabaseType::to_u32(DatabaseType::Sqlite),
            )
            .await
            .unwrap();

        debug!("{data:?}");

        let value = data
            .rows
            .first()
            .unwrap()
            .values
            .first()
            .unwrap()
            .value
            .clone();

        debug!("{value:?}");

        let expected_value = "999".as_bytes().to_vec();

        debug!("{expected_value:?}");

        assert_eq!(value, expected_value);

        let delete_result = client
            .execute_cooperative_write_at_host(
                db_name,
                "DELETE FROM EMPLOYEE WHERE Id = 999",
                "participant",
                "Id = 999",
            )
            .await
            .unwrap();

        delete_result
    }

    #[cfg(test)]
    #[tokio::main]

    async fn participant_service_client(
        participant_client_addr: &ServiceAddr,
        contract_desc: &str,
    ) -> bool {
        use log::info;
        use rcd_client::RcdClient;
        let mut has_contract = false;

        info!(
            "main_service_client attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_grpc_client(
            participant_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            60,
        )
        .await;

        client.generate_host_info("participant").await.unwrap();

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

        accepted_contract
    }
}

pub mod http {

    use crate::test_harness::{self, ServiceAddr};
    use log::{debug, info};
    use rcd_client::RcdClient;
    use std::sync::mpsc;
    use std::{thread};

    #[test]
    fn test() {
        let test_name = "add_read_delete_remote_http";
        let test_db_name = format!("{}{}", test_name, ".db");
        let custom_contract_description = String::from("add read delete remote row");

        let (tx_main, rx_main) = mpsc::channel();
        let (tx_participant, rx_participant) = mpsc::channel();
        let (tx_main_write, rx_main_read) = mpsc::channel();

        let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);

        let main_addrs = test_harness::http::start_service_with_http(&test_db_name, dirs.main_dir);

        let m_keep_alive = main_addrs.1;
        let main_addrs = main_addrs.0;

        let participant_addrs =
            test_harness::http::start_service_with_http(&test_db_name, dirs.participant_dir);
        let p_keep_alive = participant_addrs.1;
        let participant_addrs = participant_addrs.0;

        let main_addrs1 = main_addrs.clone();
        let main_addrs2 = main_addrs.clone();
        let participant_addrs1 = participant_addrs.clone();
        let participant_addrs2 = participant_addrs.clone();

        test_harness::sleep_test();

        let main_contract_desc = custom_contract_description.clone();
        let participant_contract_desc = custom_contract_description;
        let main_db_name = test_db_name;
        let main_db_name_write = main_db_name.clone();

        thread::spawn(move || {
            let res = main_service_client(
                &main_db_name,
                &main_addrs,
                &participant_addrs,
                &main_contract_desc,
            );
            tx_main.send(res).unwrap();
        })
        .join()
        .unwrap();

        let sent_participant_contract = rx_main.try_recv().unwrap();
        debug!("send_participant_contract: got: {sent_participant_contract}");

        assert!(sent_participant_contract);

        thread::spawn(move || {
            let res = participant_service_client(participant_addrs1, participant_contract_desc);
            tx_participant.send(res).unwrap();
        })
        .join()
        .unwrap();

        let participant_accepted_contract = rx_participant.try_recv().unwrap();
        debug!("participant_accpeted_contract: got: {participant_accepted_contract}");

        assert!(participant_accepted_contract);

        thread::spawn(move || {
            let res = main_execute_coop_write_and_read(&main_db_name_write, main_addrs1);
            tx_main_write.send(res).unwrap();
        })
        .join()
        .unwrap();

        let write_and_read_is_successful = rx_main_read.try_recv().unwrap();

        assert!(write_and_read_is_successful);

        let _ = m_keep_alive.send(false);
        let _ = p_keep_alive.send(false);

        test_harness::release_port(main_addrs2.port);
        test_harness::release_port(participant_addrs2.port);
        test_harness::shutdown_http(main_addrs2.ip4_addr, main_addrs2.port);
        test_harness::shutdown_http(participant_addrs2.ip4_addr, participant_addrs2.port);
    }

    #[cfg(test)]
    #[tokio::main]
    async fn main_service_client(
        db_name: &str,
        main_client_addr: &ServiceAddr,
        participant_db_addr: &ServiceAddr,
        contract_desc: &str,
    ) -> bool {
        use rcd_client::RcdClient;
        use rcd_enum::database_type::DatabaseType;
        use rcd_enum::logical_storage_policy::LogicalStoragePolicy;
        use rcd_enum::remote_delete_behavior::RemoteDeleteBehavior;
        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        info!(
            "main_service_client attempting to connect {}",
            main_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            60,
            main_client_addr.ip4_addr.clone(),
            main_client_addr.port,
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
                "127.0.0.1".to_string(),
                participant_db_addr.port as u16,
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
    async fn main_execute_coop_write_and_read(
        db_name: &str,
        main_client_addr: ServiceAddr,
    ) -> bool {
        use rcd_enum::database_type::DatabaseType;

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            60,
            "127.0.0.1".to_string(),
            main_client_addr.port,
        );

        client
            .execute_cooperative_write_at_host(
                db_name,
                "INSERT INTO EMPLOYEE ( Id, Name ) VALUES ( 999, 'ASDF');",
                "participant",
                "",
            )
            .await
            .unwrap();

        let data = client
            .execute_read_at_host(
                db_name,
                "SELECT ID FROM EMPLOYEE",
                DatabaseType::to_u32(DatabaseType::Sqlite),
            )
            .await
            .unwrap();

        debug!("{data:?}");

        let value = data
            .rows
            .first()
            .unwrap()
            .values
            .first()
            .unwrap()
            .value
            .clone();

        debug!("{value:?}");

        let expected_value = "999".as_bytes().to_vec();

        debug!("{expected_value:?}");

        assert_eq!(value, expected_value);

        let delete_result = client
            .execute_cooperative_write_at_host(
                db_name,
                "DELETE FROM EMPLOYEE WHERE Id = 999",
                "participant",
                "Id = 999",
            )
            .await
            .unwrap();

        delete_result
    }

    #[cfg(test)]
    #[tokio::main]

    async fn participant_service_client(
        participant_client_addr: ServiceAddr,
        contract_desc: String,
    ) -> bool {
        use log::info;
        use rcd_client::RcdClient;
        let mut has_contract = false;

        info!(
            "main_service_client attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            60,
            participant_client_addr.ip4_addr,
            participant_client_addr.port,
        );

        client.generate_host_info("participant").await.unwrap();

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

        accepted_contract
    }
}
