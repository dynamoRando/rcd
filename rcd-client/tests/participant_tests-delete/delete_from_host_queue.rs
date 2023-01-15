pub mod grpc {

    use crate::test_harness::{self, ServiceAddr};
    use log::info;
    use rcd_enum::{deletes_from_host_behavior::DeletesFromHostBehavior};
    use rcd_client::RcdClient;
    use std::sync::mpsc;
    use std::{thread};
    
    /*
    # Test Description
    
    */
    #[test]
    fn test() {
        let test_name = "delete_from_host_with_queue_grpc";
        let test_db_name = format!("{}{}", test_name, ".db");
        let custom_contract_description = String::from("insert read remote row");
    
        let delete_statement = "DELETE FROM EMPLOYEE WHERE Id = 999";
        let delete_statement2 = delete_statement.clone();
    
        let where_clause = "Id = 999";
    
        let (tx_main, rx_main) = mpsc::channel();
        let (tx_participant, rx_participant) = mpsc::channel();
        let (tx_main_write, rx_main_read) = mpsc::channel();
        let (tx_p_deny_write, rx_p_deny_write) = mpsc::channel();
        let (tx_h_auth_fail, rx_h_auth_fail) = mpsc::channel();
    
        let (tx_p_accept_delete, rx_p_accept_delete) = mpsc::channel();
        let (tx_m_no_rows, rx_m_no_rows) = mpsc::channel();
    
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
        let main_db_name_ = main_db_name.clone();
        let participant_db_name = test_db_name.clone();
        let pdn = participant_db_name.clone();
        let main_db_name_write = main_db_name.clone();
        let db_name_copy = main_db_name_write.clone();
        let db_name_copy2 = db_name_copy.clone();
    
        let addr_1 = participant_addrs.0.clone();
        let addr_1_1clone = addr_1.clone();
    
        let main_srv_addr = main_addrs.0.clone();
        let addr = main_srv_addr.clone();
        let addr2 = addr.clone();
    
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
    
        let new_behavior = DeletesFromHostBehavior::QueueForReview;
    
        thread::spawn(move || {
            let res = participant_changes_delete_behavior(&pdn, addr_1, new_behavior);
            tx_p_deny_write.send(res).unwrap();
        })
        .join()
        .unwrap();
    
        let status_change_is_successful = rx_p_deny_write.try_recv().unwrap();
    
        assert!(status_change_is_successful);
    
        thread::spawn(move || {
            let res = main_delete_should_fail(&db_name_copy, delete_statement, where_clause, addr);
            tx_h_auth_fail.send(res).unwrap();
        })
        .join()
        .unwrap();
    
        let should_succeed_not_succeed = rx_h_auth_fail.try_recv().unwrap();
    
        // this is returning false, because we have queued up the delete for review
        // and so we don't return the row_id from the participant back to the host
        // and because we can't find the row_id to delete
        // we are unable to delete the data hash from the meta data
        assert!(!should_succeed_not_succeed);
    
        // participant - gets pending deletes and later accepts the deletion
        thread::spawn(move || {
            let res = participant_get_and_approve_pending_deletion(
                &main_db_name_,
                addr_1_1clone,
                delete_statement2,
            );
            tx_p_accept_delete.send(res).unwrap();
        })
        .join()
        .unwrap();
    
        let has_and_accept_delete = rx_p_accept_delete.try_recv().unwrap();
        assert!(has_and_accept_delete);
    
        thread::spawn(move || {
            let res = main_should_not_have_rows(&db_name_copy2, addr2);
            tx_m_no_rows.send(res).unwrap();
        })
        .join()
        .unwrap();
    
        let should_have_no_rows = rx_m_no_rows.try_recv().unwrap();
    
        assert!(should_have_no_rows);
    
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
    #[allow(unused_variables)]
    async fn main_service_client(
        db_name: &str,
        main_client_addr: ServiceAddr,
        participant_db_addr: ServiceAddr,
        contract_desc: String,
    ) -> bool {
        use rcd_common::rcd_enum::LogicalStoragePolicy;
        use rcd_common::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};
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
        ).await;

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
                0
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
    async fn main_execute_coop_write_and_read(db_name: &str, main_client_addr: ServiceAddr) -> bool {
        use rcd_common::rcd_enum::DatabaseType;
    
        let mut client = RcdClient::new_grpc_client(
            main_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            5,
        ).await;
    
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
        ).await;
    
        let _ = client.generate_host_info("participant").await.unwrap();
    
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
    async fn participant_changes_delete_behavior(
        db_name: &str,
        participant_client_addr: ServiceAddr,
        behavior: DeletesFromHostBehavior,
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
        ).await;
    
        let result = client
            .change_deletes_from_host_behavior(db_name, "EMPLOYEE", behavior)
            .await;
    
        return result.unwrap();
    }
    
    #[cfg(test)]
    #[tokio::main]
    async fn main_delete_should_fail(
        db_name: &str,
        delete_statement: &str,
        where_clause: &str,
        main_client_addr: ServiceAddr,
    ) -> bool {
        let mut client = RcdClient::new_grpc_client(
            main_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            5,
        ).await;
    
        let delete_result = client
            .execute_cooperative_write_at_host(db_name, delete_statement, "participant", where_clause)
            .await;
        return delete_result.unwrap();
    }
    
    #[cfg(test)]
    #[tokio::main]
    async fn participant_get_and_approve_pending_deletion(
        db_name: &str,
        participant_client_addr: ServiceAddr,
        delete_statement: &str,
    ) -> bool {
        use log::info;
        use rcd_client::RcdClient;
    
        let mut has_statement: bool = false;
        let mut statement_row_id: u32 = 0;
    
        info!(
            "get_pending_deletions_at_participant attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );
    
        let mut client = RcdClient::new_grpc_client(
            participant_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            5,
        ).await;
    
        let pending_updates = client
            .get_pending_actions_at_participant(db_name, "EMPLOYEE", "DELETE")
            .await
            .unwrap();
    
        for statement in &pending_updates.pending_statements {
            if statement.statement == delete_statement {
                has_statement = true;
                statement_row_id = statement.row_id;
            }
        }
    
        assert!(has_statement);
    
        if has_statement {
            println!("has statement");
    
            let accept_delete_result = client
                .accept_pending_action_at_participant(db_name, "EMPLOYEE", statement_row_id)
                .await
                .unwrap();
    
            println!("{:?}", accept_delete_result);
    
            return accept_delete_result.is_successful;
        }
    
        return false;
    }
    
    #[cfg(test)]
    #[tokio::main]
    async fn main_should_not_have_rows(db_name: &str, main_client_addr: ServiceAddr) -> bool {
        use rcd_common::rcd_enum::DatabaseType;
    
        let mut client = RcdClient::new_grpc_client(
            main_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            5,
        ).await;
    
        let result = client
            .execute_read_at_host(
                db_name,
                "SELECT * FROM EMPLOYEE",
                DatabaseType::to_u32(DatabaseType::Sqlite),
            )
            .await;
    
        let rows = result.unwrap().rows.len();
    
        return rows == 0;
    }
    
}


pub mod http {

    use crate::test_harness::{self, ServiceAddr};
    use log::info;
    use rcd_enum::{deletes_from_host_behavior::DeletesFromHostBehavior};
    use rcd_client::RcdClient;
    use std::sync::mpsc;
    use std::{thread, time};
    
    /*
    # Test Description
    
    */
    #[test]
    fn test() {
        let test_name = "delete_from_host_with_queue_http";
        let test_db_name = format!("{}{}", test_name, ".db");
        let custom_contract_description = String::from("insert read remote row");
    
        let delete_statement = "DELETE FROM EMPLOYEE WHERE Id = 999";
        let delete_statement2 = delete_statement.clone();
    
        let where_clause = "Id = 999";
    
        let (tx_main, rx_main) = mpsc::channel();
        let (tx_participant, rx_participant) = mpsc::channel();
        let (tx_main_write, rx_main_read) = mpsc::channel();
        let (tx_p_deny_write, rx_p_deny_write) = mpsc::channel();
        let (tx_h_auth_fail, rx_h_auth_fail) = mpsc::channel();
    
        let (tx_p_accept_delete, rx_p_accept_delete) = mpsc::channel();
        let (tx_m_no_rows, rx_m_no_rows) = mpsc::channel();
    
        let dirs = test_harness::get_test_temp_dir_main_and_participant(&test_name);
    
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
    
        let time = time::Duration::from_secs(1);
    
        info!("sleeping for 1 seconds...");
    
        thread::sleep(time);
    
        let main_contract_desc = custom_contract_description.clone();
        let participant_contract_desc = custom_contract_description.clone();
        let main_db_name = test_db_name.clone();
        let main_db_name_ = main_db_name.clone();
        let participant_db_name = test_db_name.clone();
        let pdn = participant_db_name.clone();
        let main_db_name_write = main_db_name.clone();
        let db_name_copy = main_db_name_write.clone();
        let db_name_copy2 = db_name_copy.clone();
    
    
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
    
        let new_behavior = DeletesFromHostBehavior::QueueForReview;
    
        thread::spawn(move || {
            let res = participant_changes_delete_behavior(&pdn, pa2, new_behavior);
            tx_p_deny_write.send(res).unwrap();
        })
        .join()
        .unwrap();
    
        let status_change_is_successful = rx_p_deny_write.try_recv().unwrap();
    
        assert!(status_change_is_successful);
    
        thread::spawn(move || {
            let res = main_delete_should_fail(&db_name_copy, delete_statement, where_clause, ma2);
            tx_h_auth_fail.send(res).unwrap();
        })
        .join()
        .unwrap();
    
        let should_succeed_not_succeed = rx_h_auth_fail.try_recv().unwrap();
    
        // this is returning false, because we have queued up the delete for review
        // and so we don't return the row_id from the participant back to the host
        // and because we can't find the row_id to delete
        // we are unable to delete the data hash from the meta data
        assert!(!should_succeed_not_succeed);
    
        // participant - gets pending deletes and later accepts the deletion
        thread::spawn(move || {
            let res = participant_get_and_approve_pending_deletion(
                &main_db_name_,
                pa3,
                delete_statement2,
            );
            tx_p_accept_delete.send(res).unwrap();
        })
        .join()
        .unwrap();
    
        let has_and_accept_delete = rx_p_accept_delete.try_recv().unwrap();
        assert!(has_and_accept_delete);
    
        thread::spawn(move || {
            let res = main_should_not_have_rows(&db_name_copy2, ma3);
            tx_m_no_rows.send(res).unwrap();
        })
        .join()
        .unwrap();
    
        let should_have_no_rows = rx_m_no_rows.try_recv().unwrap();
    
        assert!(should_have_no_rows);

        let _ = m_keep_alive.send(false);
        let _ = p_keep_alive.send(false);
    
        test_harness::release_port(ma4.port);
        test_harness::release_port(pa4.port);

        test_harness::shutdown_http(ma4.ip4_addr, ma4.port);
        test_harness::shutdown_http(pa4.ip4_addr, pa4.port);

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
        use rcd_common::rcd_enum::LogicalStoragePolicy;
        use rcd_common::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};
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
            main_client_addr.port
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
    async fn main_execute_coop_write_and_read(db_name: &str, main_client_addr: ServiceAddr) -> bool {
        use rcd_common::rcd_enum::DatabaseType;
    
        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            main_client_addr.ip4_addr,
            main_client_addr.port
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
            participant_client_addr.port
        );
    
        let _ = client.generate_host_info("participant").await.unwrap();
    
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
    async fn participant_changes_delete_behavior(
        db_name: &str,
        participant_client_addr: ServiceAddr,
        behavior: DeletesFromHostBehavior,
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
            participant_client_addr.port
        );
    
        let result = client
            .change_deletes_from_host_behavior(db_name, "EMPLOYEE", behavior)
            .await;
    
        return result.unwrap();
    }
    
    #[cfg(test)]
    #[tokio::main]
    async fn main_delete_should_fail(
        db_name: &str,
        delete_statement: &str,
        where_clause: &str,
        main_client_addr: ServiceAddr,
    ) -> bool {
        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            main_client_addr.ip4_addr,
            main_client_addr.port
        );
    
        let delete_result = client
            .execute_cooperative_write_at_host(db_name, delete_statement, "participant", where_clause)
            .await;
        return delete_result.unwrap();
    }
    
    #[cfg(test)]
    #[tokio::main]
    async fn participant_get_and_approve_pending_deletion(
        db_name: &str,
        participant_client_addr: ServiceAddr,
        delete_statement: &str,
    ) -> bool {
        use log::info;
        use rcd_client::RcdClient;
    
        let mut has_statement: bool = false;
        let mut statement_row_id: u32 = 0;
    
        info!(
            "get_pending_deletions_at_participant attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );
    
        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            participant_client_addr.ip4_addr,
            participant_client_addr.port
        );
    
        let pending_updates = client
            .get_pending_actions_at_participant(db_name, "EMPLOYEE", "DELETE")
            .await
            .unwrap();
    
        for statement in &pending_updates.pending_statements {
            if statement.statement == delete_statement {
                has_statement = true;
                statement_row_id = statement.row_id;
            }
        }
    
        assert!(has_statement);
    
        if has_statement {
            println!("has statement");
    
            let accept_delete_result = client
                .accept_pending_action_at_participant(db_name, "EMPLOYEE", statement_row_id)
                .await
                .unwrap();
    
            println!("{:?}", accept_delete_result);
    
            return accept_delete_result.is_successful;
        }
    
        return false;
    }
    
    #[cfg(test)]
    #[tokio::main]
    async fn main_should_not_have_rows(db_name: &str, main_client_addr: ServiceAddr) -> bool {
        use rcd_common::rcd_enum::DatabaseType;
    
        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            main_client_addr.ip4_addr,
            main_client_addr.port
        );
    
        let result = client
            .execute_read_at_host(
                db_name,
                "SELECT * FROM EMPLOYEE",
                DatabaseType::to_u32(DatabaseType::Sqlite),
            )
            .await;
    
        let rows = result.unwrap().rows.len();
    
        return rows == 0;
    }
    
}