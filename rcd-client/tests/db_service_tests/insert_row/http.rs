
    use crate::test_harness::{self, ServiceAddr};
    use log::{debug, info};
    use rcd_client::RcdClient;
    use std::sync::{mpsc, Arc};
    use std::thread;

    #[test]
    fn test() {
        /*
            We will need to kick off two services, the host and the participant
            and we will need to also kick off two clients, one for each
        */

        let test_name = "insert_remote_row_http";
        let db = Arc::new(format!("{}{}", test_name, ".db"));
        let contract = Arc::new(String::from("insert remote row"));

        let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);
        let main_test_config = test_harness::http::start_service_with_http(&db, dirs.main_dir);
        let participant_addrs = test_harness::http::start_service_with_http(&db, dirs.participant_dir);

        test_harness::sleep_test();

        {
            let (tx, rx) = mpsc::channel();
            
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
        }

        thread::spawn(move || {
            let res = participant_service_client(pa1, participant_contract_desc);
            tx_participant.send(res).unwrap();
        })
        .join()
        .unwrap();

        let participant_accepted_contract = rx_participant.try_recv().unwrap();
        debug!("participant_accpeted_contract: got: {participant_accepted_contract}");

        assert!(participant_accepted_contract);

        thread::spawn(move || {
            let res = main_execute_coop_write(&main_db_name_write, ma1);
            tx_main_write.send(res).unwrap();
        })
        .join()
        .unwrap();

        let write_is_successful = rx_main_read.try_recv().unwrap();

        assert!(write_is_successful);

        let _ = m_keep_alive.send(false);
        let _ = p_keep_alive.send(false);

        test_harness::release_port(ma2.port);
        test_harness::release_port(pa2.port);
        test_harness::shutdown_http(ma2.ip4_addr, ma2.port);
        test_harness::shutdown_http(pa2.ip4_addr, pa2.port);
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
                participant_db_addr.ip4_addr.clone(),
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

    async fn main_execute_coop_write(db_name: &str, main_client_addr: ServiceAddr) -> bool {
        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            60,
            main_client_addr.ip4_addr.clone(),
            main_client_addr.port,
        );

        return client
            .execute_cooperative_write_at_host(
                db_name,
                "INSERT INTO EMPLOYEE ( Id, Name ) VALUES ( 999, 'ASDF');",
                "participant",
                "",
            )
            .await
            .unwrap();
    }

    #[cfg(test)]
    #[tokio::main]
    async fn participant_service_client(
        participant_client_addr: ServiceAddr,
        contract_desc: String,
    ) -> bool {
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
            participant_client_addr.ip4_addr.clone(),
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
