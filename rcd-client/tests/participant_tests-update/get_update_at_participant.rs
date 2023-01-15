pub mod grpc {

    use crate::test_harness::{self, ServiceAddr};
    use log::info;
    use rcd_client::RcdClient;
    use rcd_enum::updates_to_host_behavior::UpdatesToHostBehavior;
    use std::sync::mpsc;
    use std::thread;

    /*
    # Test Description

    */

    #[test]
    fn test() {
        let test_name = "get_update_from_part_gprc";
        let test_db_name = format!("{}{}", test_name, ".db");
        let custom_contract_description = String::from("insert read remote row");

        let (tx_main, rx_main) = mpsc::channel();
        let (tx_participant, rx_participant) = mpsc::channel();
        let (tx_main_write, rx_main_read) = mpsc::channel();
        let (tx_p_change_update, rx_p_change_update) = mpsc::channel();

        let dirs = test_harness::get_test_temp_dir_main_and_participant(&test_name);

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
        let participant_contract_desc = custom_contract_description.clone();
        let main_db_name = test_db_name.clone();
        let participant_db_name = test_db_name.clone();
        let pdn = participant_db_name.clone();
        let main_db_name_write = main_db_name.clone();

        let addr_1 = participant_addrs.0.clone();

        let main_srv_addr = main_addrs.0.clone();

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

        let new_behavior = UpdatesToHostBehavior::SendDataHashChange;

        thread::spawn(move || {
            let res = participant_changes_update_behavior(&pdn, addr_1, new_behavior);
            tx_p_change_update.send(res).unwrap();
        })
        .join()
        .unwrap();

        let update_at_participant_is_successful = rx_p_change_update.try_recv().unwrap();

        assert!(update_at_participant_is_successful);

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
        use rcd_client::RcdClient;
        use rcd_common::rcd_enum::LogicalStoragePolicy;
        use rcd_common::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};

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
        use rcd_common::rcd_enum::DatabaseType;

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

        return value == expected_value;
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

        return accepted_contract;
    }

    #[cfg(test)]
    #[tokio::main]
    async fn participant_changes_update_behavior(
        db_name: &str,
        participant_client_addr: ServiceAddr,
        behavior: UpdatesToHostBehavior,
    ) -> bool {
        use log::info;
        use rcd_client::RcdClient;
        use rcd_common::rcd_enum::DatabaseType;

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
            .change_updates_to_host_behavior(db_name, "EMPLOYEE", behavior)
            .await;

        assert!(change_update_behavior.unwrap());

        let statement = String::from("UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999");
        let update_result = client
            .execute_write_at_participant(
                db_name,
                &statement,
                DatabaseType::to_u32(DatabaseType::Sqlite),
                "ID = 999",
            )
            .await;

        if !update_result.unwrap() {
            return false;
        }

        let response = client
            .get_updates_to_host_behavior(db_name, "EMPLOYEE")
            .await
            .unwrap();
        let response_behavior = UpdatesToHostBehavior::from_u32(response.behavior);

        return response_behavior == behavior;
    }
}

pub mod http {

    use crate::test_harness::{self, ServiceAddr};
    use log::info;
    use rcd_client::RcdClient;
    use rcd_enum::updates_to_host_behavior::UpdatesToHostBehavior;
    use std::sync::mpsc;
    use std::{thread, time};

    /*
    # Test Description

    */

    #[test]
    fn test() {
        let test_name = "get_update_from_part_http";
        let test_db_name = format!("{}{}", test_name, ".db");
        let custom_contract_description = String::from("insert read remote row");

        let (tx_main, rx_main) = mpsc::channel();
        let (tx_participant, rx_participant) = mpsc::channel();
        let (tx_main_write, rx_main_read) = mpsc::channel();
        let (tx_p_change_update, rx_p_change_update) = mpsc::channel();

        let dirs = test_harness::get_test_temp_dir_main_and_participant(&test_name);

        let main_addrs = test_harness::start_service_with_http(&test_db_name, dirs.1);

        let m_keep_alive = main_addrs.1;
        let main_addrs = main_addrs.0;

        let ma1 = main_addrs.clone();
        let ma4 = main_addrs.clone();

        let participant_addrs = test_harness::start_service_with_http(&test_db_name, dirs.2);

        let p_keep_alive = participant_addrs.1;
        let participant_addrs = participant_addrs.0;

        let pa1 = participant_addrs.clone();
        let pa2 = participant_addrs.clone();
        let pa6 = participant_addrs.clone();

        let time = time::Duration::from_secs(1);
        info!("sleeping for 1 seconds...");
        thread::sleep(time);

        let main_contract_desc = custom_contract_description.clone();
        let participant_contract_desc = custom_contract_description.clone();
        let main_db_name = test_db_name.clone();
        let participant_db_name = test_db_name.clone();
        let pdn = participant_db_name.clone();
        let main_db_name_write = main_db_name.clone();

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

        let new_behavior = UpdatesToHostBehavior::SendDataHashChange;

        thread::spawn(move || {
            let res = participant_changes_update_behavior(&pdn, pa2, new_behavior);
            tx_p_change_update.send(res).unwrap();
        })
        .join()
        .unwrap();

        let update_at_participant_is_successful = rx_p_change_update.try_recv().unwrap();

        assert!(update_at_participant_is_successful);

        let _ = m_keep_alive.send(false);
        let _ = p_keep_alive.send(false);

        test_harness::release_port(ma4.port);
        test_harness::release_port(pa6.port);

        test_harness::shutdown_http(ma4.ip4_addr, ma4.port);
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
        use rcd_client::RcdClient;
        use rcd_common::rcd_enum::LogicalStoragePolicy;
        use rcd_common::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};

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
        use rcd_common::rcd_enum::DatabaseType;

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

        return value == expected_value;
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

        return accepted_contract;
    }

    #[cfg(test)]
    #[tokio::main]
    async fn participant_changes_update_behavior(
        db_name: &str,
        participant_client_addr: ServiceAddr,
        behavior: UpdatesToHostBehavior,
    ) -> bool {
        use log::info;
        use rcd_client::RcdClient;
        use rcd_common::rcd_enum::DatabaseType;

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
            .change_updates_to_host_behavior(db_name, "EMPLOYEE", behavior)
            .await;

        assert!(change_update_behavior.unwrap());

        let statement = String::from("UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999");
        let update_result = client
            .execute_write_at_participant(
                db_name,
                &statement,
                DatabaseType::to_u32(DatabaseType::Sqlite),
                "ID = 999",
            )
            .await;

        if !update_result.unwrap() {
            return false;
        }

        let response = client
            .get_updates_to_host_behavior(db_name, "EMPLOYEE")
            .await
            .unwrap();
        let response_behavior = UpdatesToHostBehavior::from_u32(response.behavior);

        return response_behavior == behavior;
    }
}
