pub mod grpc {

    use crate::test_harness::{self, ServiceAddr};
    use log::info;
    use rcd_client::RcdClient;
    use rcd_enum::updates_from_host_behavior::UpdatesFromHostBehavior;
    use std::sync::mpsc;
    use std::thread;

    /*
    # Test Description

    ## Purpose:
    This test checks to see if when an UPDATE statement is sent from the host if the participant's settings to
    `UpdatesFromHostBehavior::OverwriteWithLog` that rcd copies the row it's about to overwrite to a `_COOP_DATA_LOG`
    table before actually executing the overwrite.

    ## Feature Background
    We want to make sure that participants have full authority over their data. This means that if they want to see
    a history of changes that are being made to their data, they can do so. In this test, a value is initially set by a host
    and later is UPDATEd.

    We expect that the UPDATE from the host should succeed, but that we should also have a record of the changed row
    in the `EMPLOYEE_COOP_DATA_LOG` table in the partial database that the participant can review.

    ## Test Steps
    - Start an rcd instance for a main (host) and a participant
    - Host:
        - Generate a db and tables and a contract to send to particpant
    - Participant:
        - Accept contract
    - Host:
        - Send one row to participant to be inserted and test to make sure can read from participant
    - Participant:
        - Change UpdatesFromHostBehavior to Overwrite With Log
        - Update the newly added row from the previous step
        - Get the row id for the newly updated row
        - Get the data hash for the newly updated row
    - Host:
        - Attempt to read previously inserted row, and should returning matching data.
        - Get the data hash saved at the host
        - Check to make sure the hashes match (ensure that the update is correct)
    - Participant:
        - Send a query to `SELECT * FROM EMPLOYEE_COOP_DATA_LOG` locally at the participant. There should only
        be one row that is returned.

    ### Expected Results:
    The row returned at the participant should match the previously inserted value instead of the UPDATEd value.

    */

    #[test]
    fn test() {
        let test_name = "update_from_host_log_grpc";
        let test_db_name = format!("{}{}", test_name, ".db");
        let custom_contract_description = String::from("insert read remote row");

        let (tx_main, rx_main) = mpsc::channel();
        let (tx_participant, rx_participant) = mpsc::channel();
        let (tx_main_write, rx_main_read) = mpsc::channel();
        let (tx_p_change_update, rx_p_change_update) = mpsc::channel();
        let (tx_h_can_read, rx_h_can_read) = mpsc::channel();

        let (tx_h_data_hash, rx_h_data_hash) = mpsc::channel();
        let (tx_p_data_hash, rx_p_data_hash) = mpsc::channel();

        let (tx_p_row_id, rx_p_row_id) = mpsc::channel();
        let (tx_p_read_data_log, rx_p_read_data_log) = mpsc::channel();

        let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);

        let main_addrs = test_harness::start_service_with_grpc(&test_db_name, dirs.1);

        let main_addr_client_port = main_addrs.2;
        let main_addr_db_port = main_addrs.3;

        let main_client_shutdown_trigger = main_addrs.4;
        let main_db_shutdown_triger = main_addrs.5;

        let participant_addrs = test_harness::start_service_with_grpc(&test_db_name, dirs.2);

        let part_addr_client_port = participant_addrs.2;
        let part_addr_db_port = participant_addrs.3;

        let part_client_shutdown_trigger = participant_addrs.4;
        let part_db_shutdown_trigger = participant_addrs.5;

        test_harness::sleep_test();

        let main_contract_desc = custom_contract_description.clone();
        let participant_contract_desc = custom_contract_description;
        let main_db_name = test_db_name.clone();
        let participant_db_name = test_db_name;
        let pdn = participant_db_name;
        let main_db_name_write = main_db_name.clone();
        let db_name_copy = main_db_name_write.clone();

        let db_p_name = db_name_copy.clone();
        let db_p_name2 = db_p_name.clone();
        let db_pname3 = db_p_name2.clone();
        let db_h_name = db_name_copy.clone();

        let addr_1 = participant_addrs.0.clone();

        let main_srv_addr = main_addrs.0.clone();
        let addr = main_srv_addr.clone();

        let h_addr = addr.clone();
        let p_addr = addr_1.clone();
        let p_addr2 = p_addr.clone();
        let p_addr3 = p_addr.clone();

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
            let res = participant_service_client(participant_addrs.0, participant_contract_desc);
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

        thread::spawn(move || {
            let res = main_execute_coop_write_and_read(&main_db_name_write, main_srv_addr);
            tx_main_write.send(res).unwrap();
        })
        .join()
        .unwrap();

        let write_and_read_is_successful = rx_main_read.try_recv().unwrap();

        assert!(write_and_read_is_successful);

        let new_behavior = UpdatesFromHostBehavior::OverwriteWithLog;

        thread::spawn(move || {
            let res = participant_changes_update_behavior(&pdn, addr_1, new_behavior);
            tx_p_change_update.send(res).unwrap();
        })
        .join()
        .unwrap();

        let update_at_participant_is_successful = rx_p_change_update.try_recv().unwrap();

        assert!(update_at_participant_is_successful);

        thread::spawn(move || {
            let res = main_read_updated_row_should_succeed(&db_name_copy, addr);
            tx_h_can_read.send(res).unwrap();
        })
        .join()
        .unwrap();

        let can_read_rows = rx_h_can_read.try_recv().unwrap();
        assert!(can_read_rows);

        thread::spawn(move || {
            let res = get_row_id_at_participant(&db_p_name, p_addr);
            tx_p_row_id.send(res).unwrap();
        })
        .join()
        .unwrap();

        let p_row_id = rx_p_row_id.try_recv().unwrap();
        let rid = p_row_id;
        let rid2 = p_row_id;

        thread::spawn(move || {
            let res = get_data_hash_for_changed_row_at_participant(&db_p_name2, p_addr2, rid);
            tx_p_data_hash.send(res).unwrap();
        })
        .join()
        .unwrap();

        let p_data_hash = rx_p_data_hash.try_recv().unwrap();

        thread::spawn(move || {
            let res = get_data_hash_for_changed_row_at_host(&db_h_name, h_addr, rid2);
            tx_h_data_hash.send(res).unwrap();
        })
        .join()
        .unwrap();

        let h_data_hash = rx_h_data_hash.try_recv().unwrap();

        assert_eq!(p_data_hash, h_data_hash);

        thread::spawn(move || {
            let res = get_data_logs_at_participant(&db_pname3, p_addr3);
            tx_p_read_data_log.send(res).unwrap();
        })
        .join()
        .unwrap();

        let p_read_data_log_is_correct = rx_p_read_data_log.try_recv().unwrap();

        assert!(p_read_data_log_is_correct);

        test_harness::release_port(main_addr_client_port);
        test_harness::release_port(main_addr_db_port);
        test_harness::release_port(part_addr_client_port);
        test_harness::release_port(part_addr_db_port);

        main_client_shutdown_trigger.trigger();
        main_db_shutdown_triger.trigger();
        part_client_shutdown_trigger.trigger();
        part_db_shutdown_trigger.trigger();
    }

    #[cfg(test)]
    #[tokio::main]

    async fn main_service_client(
        db_name: &str,
        main_client_addr: ServiceAddr,
        participant_db_addr: ServiceAddr,
        contract_desc: String,
    ) -> bool {
        use rcd_enum::database_type::DatabaseType;
        use rcd_enum::logical_storage_policy::LogicalStoragePolicy;
        use rcd_enum::remote_delete_behavior::RemoteDeleteBehavior;

        use rcd_client::RcdClient;

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        info!(
            "main_service_client attempting to connect {}",
            main_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_grpc_client(
            main_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            5,
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
        main_client_addr: ServiceAddr,
    ) -> bool {
        use rcd_enum::database_type::DatabaseType;

        let mut client = RcdClient::new_grpc_client(
            main_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            5,
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

        println!("{:?}", data);

        let value = data
            .rows
            .first()
            .unwrap()
            .values
            .first()
            .unwrap()
            .value
            .clone();

        println!("{:?}", value);

        let expected_value = "999".as_bytes().to_vec();

        println!("{:?}", expected_value);

        value == expected_value
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
            "participant_service_client attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_grpc_client(
            participant_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            5,
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

    #[cfg(test)]
    #[tokio::main]

    async fn participant_changes_update_behavior(
        db_name: &str,
        participant_client_addr: ServiceAddr,
        behavior: UpdatesFromHostBehavior,
    ) -> bool {
        use log::info;

        use rcd_client::RcdClient;

        info!(
            "participant_changes_update_behavior attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_grpc_client(
            participant_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            5,
        )
        .await;

        let change_update_behavior = client
            .change_updates_from_host_behavior(db_name, "EMPLOYEE", behavior)
            .await;

        change_update_behavior.unwrap()
    }

    #[cfg(test)]
    #[tokio::main]

    async fn get_row_id_at_participant(db_name: &str, participant_client_addr: ServiceAddr) -> u32 {
        use log::info;

        use rcd_client::RcdClient;

        info!(
            "get_data_hash_for_changed_row_at_participant attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_grpc_client(
            participant_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            5,
        )
        .await;

        let row_ids = client
            .get_row_id_at_participant(db_name, "EMPLOYEE", "NAME = 'TESTER'")
            .await
            .unwrap();
        let row_id = *row_ids.first().unwrap();

        row_id
    }

    #[cfg(test)]
    #[tokio::main]

    async fn get_data_hash_for_changed_row_at_participant(
        db_name: &str,
        participant_client_addr: ServiceAddr,
        row_id: u32,
    ) -> u64 {
        use log::info;

        use rcd_client::RcdClient;

        info!(
            "get_data_hash_for_changed_row_at_participant attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_grpc_client(
            participant_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            5,
        )
        .await;

        let data_hash_result = client
            .get_data_hash_at_participant(db_name, "EMPLOYEE", row_id)
            .await
            .unwrap();

        data_hash_result
    }

    #[cfg(test)]
    #[tokio::main]

    async fn get_data_hash_for_changed_row_at_host(
        db_name: &str,
        participant_client_addr: ServiceAddr,
        row_id: u32,
    ) -> u64 {
        use log::info;

        use rcd_client::RcdClient;

        info!(
            "get_data_hash_for_changed_row_at_participant attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_grpc_client(
            participant_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            5,
        )
        .await;

        let data_hash_result = client
            .get_data_hash_at_host(db_name, "EMPLOYEE", row_id)
            .await
            .unwrap();

        data_hash_result
    }

    #[cfg(test)]
    #[tokio::main]

    async fn main_read_updated_row_should_succeed(
        db_name: &str,
        main_client_addr: ServiceAddr,
    ) -> bool {
        use rcd_enum::database_type::DatabaseType;

        let mut client = RcdClient::new_grpc_client(
            main_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            5,
        )
        .await;

        let statement = String::from("UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999");
        let update_result = client
            .execute_cooperative_write_at_host(db_name, &statement, "participant", "ID = 999")
            .await;

        assert!(update_result.unwrap());

        let cmd = String::from("SELECT NAME FROM EMPLOYEE WHERE Id = 999");
        let read_result = client
            .execute_read_at_host(db_name, &cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
            .await;

        let results = read_result.unwrap();

        let row = results.rows.first().unwrap();

        let value = &row.values[1].value.clone();

        println!("{:?}", value);

        let expected_value = "TESTER".as_bytes().to_vec();

        println!("{:?}", expected_value);

        *value == expected_value
    }

    #[cfg(test)]
    #[tokio::main]
    async fn get_data_logs_at_participant(
        db_name: &str,
        participant_client_addr: ServiceAddr,
    ) -> bool {
        use log::info;
        use rcd_client::RcdClient;
        use rcd_enum::database_type::DatabaseType;

        info!(
            "get_data_logs_at_participant attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_grpc_client(
            participant_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            5,
        )
        .await;

        let cmd = "SELECT * FROM EMPLOYEE_COOP_DATA_LOG";
        let read_result = client
            .execute_read_at_participant(db_name, cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
            .await
            .unwrap();

        println!("{:?}", read_result);

        let row = read_result.rows.first().unwrap();
        let value = &row.values[1].value.clone();

        println!("{:?}", value);

        let expected_value = "ASDF".as_bytes().to_vec();
        println!("{:?}", expected_value);

        *value == expected_value
    }
}

pub mod http {

    use crate::test_harness::{self, ServiceAddr};
    use log::info;
    use rcd_client::RcdClient;
    use rcd_enum::updates_from_host_behavior::UpdatesFromHostBehavior;
    use std::sync::mpsc;
    use std::{thread, time};

    /*
    # Test Description

    ## Purpose:
    This test checks to see if when an UPDATE statement is sent from the host if the participant's settings to
    `UpdatesFromHostBehavior::OverwriteWithLog` that rcd copies the row it's about to overwrite to a `_COOP_DATA_LOG`
    table before actually executing the overwrite.

    ## Feature Background
    We want to make sure that participants have full authority over their data. This means that if they want to see
    a history of changes that are being made to their data, they can do so. In this test, a value is initially set by a host
    and later is UPDATEd.

    We expect that the UPDATE from the host should succeed, but that we should also have a record of the changed row
    in the `EMPLOYEE_COOP_DATA_LOG` table in the partial database that the participant can review.

    ## Test Steps
    - Start an rcd instance for a main (host) and a participant
    - Host:
        - Generate a db and tables and a contract to send to particpant
    - Participant:
        - Accept contract
    - Host:
        - Send one row to participant to be inserted and test to make sure can read from participant
    - Participant:
        - Change UpdatesFromHostBehavior to Overwrite With Log
        - Update the newly added row from the previous step
        - Get the row id for the newly updated row
        - Get the data hash for the newly updated row
    - Host:
        - Attempt to read previously inserted row, and should returning matching data.
        - Get the data hash saved at the host
        - Check to make sure the hashes match (ensure that the update is correct)
    - Participant:
        - Send a query to `SELECT * FROM EMPLOYEE_COOP_DATA_LOG` locally at the participant. There should only
        be one row that is returned.

    ### Expected Results:
    The row returned at the participant should match the previously inserted value instead of the UPDATEd value.

    */

    #[test]
    fn test() {
        let test_name = "update_from_host_log_http";
        let test_db_name = format!("{}{}", test_name, ".db");
        let custom_contract_description = String::from("insert read remote row");

        let (tx_main, rx_main) = mpsc::channel();
        let (tx_participant, rx_participant) = mpsc::channel();
        let (tx_main_write, rx_main_read) = mpsc::channel();
        let (tx_p_change_update, rx_p_change_update) = mpsc::channel();
        let (tx_h_can_read, rx_h_can_read) = mpsc::channel();

        let (tx_h_data_hash, rx_h_data_hash) = mpsc::channel();
        let (tx_p_data_hash, rx_p_data_hash) = mpsc::channel();

        let (tx_p_row_id, rx_p_row_id) = mpsc::channel();
        let (tx_p_read_data_log, rx_p_read_data_log) = mpsc::channel();

        let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);

        let main_addrs = test_harness::start_service_with_http(&test_db_name, dirs.1);

        let m_keep_alive = main_addrs.1;
        let main_addrs = main_addrs.0;

        let ma1 = main_addrs.clone();
        let ma2 = main_addrs.clone();
        let ma3 = main_addrs.clone();
        let ma4 = main_addrs.clone();

        let participant_addrs = test_harness::start_service_with_http(&test_db_name, dirs.2);

        let p_keep_alive = participant_addrs.1;
        let participant_addrs = participant_addrs.0;

        let pa1 = participant_addrs.clone();
        let pa2 = participant_addrs.clone();
        let pa3 = participant_addrs.clone();
        let pa4 = participant_addrs.clone();
        let pa5 = participant_addrs.clone();
        let pa6 = participant_addrs.clone();

        let time = time::Duration::from_secs(1);
        info!("sleeping for 1 seconds...");
        thread::sleep(time);

        let main_contract_desc = custom_contract_description.clone();
        let participant_contract_desc = custom_contract_description;
        let main_db_name = test_db_name.clone();
        let participant_db_name = test_db_name;
        let pdn = participant_db_name;
        let main_db_name_write = main_db_name.clone();
        let db_name_copy = main_db_name_write.clone();

        let db_p_name = db_name_copy.clone();
        let db_p_name2 = db_p_name.clone();
        let db_pname3 = db_p_name2.clone();
        let db_h_name = db_name_copy.clone();

        thread::spawn(move || {
            let res = main_service_client(
                &main_db_name,
                main_addrs,
                participant_addrs,
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
            let res = participant_service_client(pa1, participant_contract_desc);
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

        thread::spawn(move || {
            let res = main_execute_coop_write_and_read(&main_db_name_write, ma1);
            tx_main_write.send(res).unwrap();
        })
        .join()
        .unwrap();

        let write_and_read_is_successful = rx_main_read.try_recv().unwrap();

        assert!(write_and_read_is_successful);

        let new_behavior = UpdatesFromHostBehavior::OverwriteWithLog;

        thread::spawn(move || {
            let res = participant_changes_update_behavior(&pdn, pa2, new_behavior);
            tx_p_change_update.send(res).unwrap();
        })
        .join()
        .unwrap();

        let update_at_participant_is_successful = rx_p_change_update.try_recv().unwrap();

        assert!(update_at_participant_is_successful);

        thread::spawn(move || {
            let res = main_read_updated_row_should_succeed(&db_name_copy, ma2);
            tx_h_can_read.send(res).unwrap();
        })
        .join()
        .unwrap();

        let can_read_rows = rx_h_can_read.try_recv().unwrap();
        assert!(can_read_rows);

        thread::spawn(move || {
            let res = get_row_id_at_participant(&db_p_name, pa3);
            tx_p_row_id.send(res).unwrap();
        })
        .join()
        .unwrap();

        let p_row_id = rx_p_row_id.try_recv().unwrap();
        let rid = p_row_id;
        let rid2 = p_row_id;

        thread::spawn(move || {
            let res = get_data_hash_for_changed_row_at_participant(&db_p_name2, pa4, rid);
            tx_p_data_hash.send(res).unwrap();
        })
        .join()
        .unwrap();

        let p_data_hash = rx_p_data_hash.try_recv().unwrap();

        thread::spawn(move || {
            let res = get_data_hash_for_changed_row_at_host(&db_h_name, ma4, rid2);
            tx_h_data_hash.send(res).unwrap();
        })
        .join()
        .unwrap();

        let h_data_hash = rx_h_data_hash.try_recv().unwrap();

        assert_eq!(p_data_hash, h_data_hash);

        thread::spawn(move || {
            let res = get_data_logs_at_participant(&db_pname3, pa5);
            tx_p_read_data_log.send(res).unwrap();
        })
        .join()
        .unwrap();

        let p_read_data_log_is_correct = rx_p_read_data_log.try_recv().unwrap();

        assert!(p_read_data_log_is_correct);

        let _ = m_keep_alive.send(false);
        let _ = p_keep_alive.send(false);

        test_harness::release_port(ma3.port);
        test_harness::release_port(pa6.port);

        test_harness::shutdown_http(ma3.ip4_addr, ma3.port);
        test_harness::shutdown_http(pa6.ip4_addr, pa6.port);
    }

    #[cfg(test)]
    #[tokio::main]

    async fn main_service_client(
        db_name: &str,
        main_client_addr: ServiceAddr,
        participant_db_addr: ServiceAddr,
        contract_desc: String,
    ) -> bool {
        use rcd_enum::database_type::DatabaseType;
        use rcd_enum::logical_storage_policy::LogicalStoragePolicy;
        use rcd_enum::remote_delete_behavior::RemoteDeleteBehavior;

        use rcd_client::RcdClient;

        let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

        info!(
            "main_service_client attempting to connect {}",
            main_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            main_client_addr.ip4_addr,
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
    async fn main_execute_coop_write_and_read(
        db_name: &str,
        main_client_addr: ServiceAddr,
    ) -> bool {
        use rcd_enum::database_type::DatabaseType;

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            main_client_addr.ip4_addr,
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

        println!("{:?}", data);

        let value = data
            .rows
            .first()
            .unwrap()
            .values
            .first()
            .unwrap()
            .value
            .clone();

        println!("{:?}", value);

        let expected_value = "999".as_bytes().to_vec();

        println!("{:?}", expected_value);

        value == expected_value
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
            "participant_service_client attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
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

    #[cfg(test)]
    #[tokio::main]
    async fn participant_changes_update_behavior(
        db_name: &str,
        participant_client_addr: ServiceAddr,
        behavior: UpdatesFromHostBehavior,
    ) -> bool {
        use log::info;

        use rcd_client::RcdClient;

        info!(
            "participant_changes_update_behavior attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            participant_client_addr.ip4_addr,
            participant_client_addr.port,
        );

        let change_update_behavior = client
            .change_updates_from_host_behavior(db_name, "EMPLOYEE", behavior)
            .await;

        change_update_behavior.unwrap()
    }

    #[cfg(test)]
    #[tokio::main]
    async fn get_row_id_at_participant(db_name: &str, participant_client_addr: ServiceAddr) -> u32 {
        use log::info;

        use rcd_client::RcdClient;

        info!(
            "get_data_hash_for_changed_row_at_participant attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            participant_client_addr.ip4_addr,
            participant_client_addr.port,
        );

        let row_ids = client
            .get_row_id_at_participant(db_name, "EMPLOYEE", "NAME = 'TESTER'")
            .await
            .unwrap();
        let row_id = *row_ids.first().unwrap();

        row_id
    }

    #[cfg(test)]
    #[tokio::main]

    async fn get_data_hash_for_changed_row_at_participant(
        db_name: &str,
        participant_client_addr: ServiceAddr,
        row_id: u32,
    ) -> u64 {
        use log::info;

        use rcd_client::RcdClient;

        info!(
            "get_data_hash_for_changed_row_at_participant attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            participant_client_addr.ip4_addr,
            participant_client_addr.port,
        );

        let data_hash_result = client
            .get_data_hash_at_participant(db_name, "EMPLOYEE", row_id)
            .await
            .unwrap();

        data_hash_result
    }

    #[cfg(test)]
    #[tokio::main]
    async fn get_data_hash_for_changed_row_at_host(
        db_name: &str,
        main_client_addr: ServiceAddr,
        row_id: u32,
    ) -> u64 {
        use log::info;

        use rcd_client::RcdClient;

        info!(
            "get_data_hash_for_changed_row_at_participant attempting to connect {}",
            main_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            main_client_addr.ip4_addr,
            main_client_addr.port,
        );

        let data_hash_result = client
            .get_data_hash_at_host(db_name, "EMPLOYEE", row_id)
            .await
            .unwrap();

        data_hash_result
    }

    #[cfg(test)]
    #[tokio::main]

    async fn main_read_updated_row_should_succeed(
        db_name: &str,
        main_client_addr: ServiceAddr,
    ) -> bool {
        use rcd_enum::database_type::DatabaseType;

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            main_client_addr.ip4_addr,
            main_client_addr.port,
        );

        let statement = String::from("UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999");
        let update_result = client
            .execute_cooperative_write_at_host(db_name, &statement, "participant", "ID = 999")
            .await;

        assert!(update_result.unwrap());

        let cmd = String::from("SELECT NAME FROM EMPLOYEE WHERE Id = 999");
        let read_result = client
            .execute_read_at_host(db_name, &cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
            .await;

        let results = read_result.unwrap();

        let row = results.rows.first().unwrap();

        let value = &row.values[1].value.clone();

        println!("{:?}", value);

        let expected_value = "TESTER".as_bytes().to_vec();

        println!("{:?}", expected_value);

        *value == expected_value
    }

    #[cfg(test)]
    #[tokio::main]
    async fn get_data_logs_at_participant(
        db_name: &str,
        participant_client_addr: ServiceAddr,
    ) -> bool {
        use log::info;
        use rcd_enum::database_type::DatabaseType;

        use rcd_client::RcdClient;

        info!(
            "get_data_logs_at_participant attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            participant_client_addr.ip4_addr,
            participant_client_addr.port,
        );

        let cmd = "SELECT * FROM EMPLOYEE_COOP_DATA_LOG";
        let read_result = client
            .execute_read_at_participant(db_name, cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
            .await
            .unwrap();

        println!("{:?}", read_result);

        let row = read_result.rows.first().unwrap();
        let value = &row.values[1].value.clone();

        println!("{:?}", value);

        let expected_value = "ASDF".as_bytes().to_vec();
        println!("{:?}", expected_value);

        *value == expected_value
    }
}
