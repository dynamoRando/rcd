pub mod grpc {

    use crate::test_harness::{self, ServiceAddr};
    use log::{debug, info};
    use rcd_client::RcdClient;
    use std::sync::mpsc;
    use std::{thread};

    /*
    # Test Description

    ## Purpose:
    This test checks to see if the setting for authenticating a host is respected. This is the value in the rcd db for HOST_STATUS in
    the table CDS_HOSTS.

    ## Feature Background
    We want to make sure the participants have full authority with who they interact with. In this test, after a host and a participant
    agree to participate (by the participant accepting a contract) we want to deny the host from taking any further actions.

    ## Test Steps
    - Start an rcd instance for a main (host) and a participant
    - Host:
        - Generate a db and tables and a contract to send to particpant
    - Participant:
        - Accept contract
    - Host:
        - Send one row to participant to be inserted and test to make sure can read from participant
    - Participant:
        - Change the host status to Deny
    - Host:
        - Attempt to check authentication status.

    ### Expected Results:
    The authentication status should return a failure.

    */

    #[test]
    fn test() {
        let test_name = "reject_host_grpc";
        let test_db_name = format!("{}{}", test_name, ".db");
        let custom_contract_description = String::from("insert read remote row");

        let (tx_main, rx_main) = mpsc::channel();
        let (tx_participant, rx_participant) = mpsc::channel();
        let (tx_main_write, rx_main_read) = mpsc::channel();
        let (tx_p_deny_write, rx_p_deny_write) = mpsc::channel();
        let (tx_h_auth_fail, rx_h_auth_fail) = mpsc::channel();

        let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);

        let main_test_config = test_harness::grpc::start_service_with_grpc(&test_db_name, dirs.main_dir);
        let participant_test_config =
            test_harness::grpc::start_service_with_grpc(&test_db_name, dirs.participant_dir);

        test_harness::sleep_test();

        let participant_contract_desc = custom_contract_description.clone();

        let main_db_name = test_db_name.clone();

        let main_db_name_write = main_db_name.clone();
        let db_name_copy = main_db_name_write.clone();

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

            thread::spawn(move || {
                let res =
                    participant_service_client(participant_client_addr, participant_contract_desc);
                tx_participant.send(res).unwrap();
            })
            .join()
            .unwrap();
        }

        let participant_accepted_contract = rx_participant.try_recv().unwrap();
        debug!("participant_accpeted_contract: got: {participant_accepted_contract}");

        assert!(participant_accepted_contract);

        {
            let main_client_addr = main_test_config.client_address.clone();

            thread::spawn(move || {
                let res = main_execute_coop_write_and_read(&main_db_name_write, main_client_addr);
                tx_main_write.send(res).unwrap();
            })
            .join()
            .unwrap();
        }

        let write_and_read_is_successful = rx_main_read.try_recv().unwrap();

        assert!(write_and_read_is_successful);

        {
            let participant_client_addr = participant_test_config.client_address.clone();

            thread::spawn(move || {
                let res = participant_rejects_host(participant_client_addr);
                tx_p_deny_write.send(res).unwrap();
            })
            .join()
            .unwrap();
        }

        let status_change_is_successful = rx_p_deny_write.try_recv().unwrap();

        assert!(status_change_is_successful);

        {
            let main_client_addr = main_test_config.client_address.clone();
            thread::spawn(move || {
                let res = main_read_should_fail(&db_name_copy, main_client_addr);
                tx_h_auth_fail.send(res).unwrap();
            })
            .join()
            .unwrap();
        }

        let should_fail = rx_h_auth_fail.try_recv().unwrap();

        assert!(!should_fail);
    }

    #[cfg(test)]
    #[tokio::main]

    async fn main_service_client(
        db_name: &str,
        main_client_addr: &ServiceAddr,
        participant_db_addr: &ServiceAddr,
        contract_desc: &str,
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
        main_client_addr: ServiceAddr,
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

    #[cfg(test)]
    #[tokio::main]
    async fn participant_rejects_host(participant_client_addr: ServiceAddr) -> bool {
        use log::info;

        use rcd_client::RcdClient;
        use rcd_enum::host_status::HostStatus;

        info!(
            "participant_rejects_host attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_grpc_client(
            participant_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            60,
        )
        .await;

        let host_status = HostStatus::Deny;

        let reject_host_result = client
            .change_host_status_by_name("tester", HostStatus::to_u32(host_status))
            .await;

        reject_host_result.unwrap()
    }

    #[cfg(test)]
    #[tokio::main]

    async fn main_read_should_fail(db_name: &str, main_client_addr: ServiceAddr) -> bool {
        let mut client = RcdClient::new_grpc_client(
            main_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            60,
        )
        .await;

        let attempt = client
            .try_auth_at_participant("participant", "", db_name)
            .await;

        attempt
    }
}

pub mod http {

    use crate::test_harness::{self, ServiceAddr};
    use log::{debug, info};
    use rcd_client::RcdClient;
    use std::sync::mpsc;
    use std::{thread, time};

    /*
    # Test Description

    ## Purpose:
    This test checks to see if the setting for authenticating a host is respected. This is the value in the rcd db for HOST_STATUS in
    the table CDS_HOSTS.

    ## Feature Background
    We want to make sure the participants have full authority with who they interact with. In this test, after a host and a participant
    agree to participate (by the participant accepting a contract) we want to deny the host from taking any further actions.

    ## Test Steps
    - Start an rcd instance for a main (host) and a participant
    - Host:
        - Generate a db and tables and a contract to send to particpant
    - Participant:
        - Accept contract
    - Host:
        - Send one row to participant to be inserted and test to make sure can read from participant
    - Participant:
        - Change the host status to Deny
    - Host:
        - Attempt to check authentication status.

    ### Expected Results:
    The authentication status should return a failure.

    */

    #[test]
    fn test() {
        let test_name = "reject_host_http";
        let test_db_name = format!("{}{}", test_name, ".db");
        let custom_contract_description = String::from("insert read remote row");

        let (tx_main, rx_main) = mpsc::channel();
        let (tx_participant, rx_participant) = mpsc::channel();
        let (tx_main_write, rx_main_read) = mpsc::channel();
        let (tx_p_deny_write, rx_p_deny_write) = mpsc::channel();
        let (tx_h_auth_fail, rx_h_auth_fail) = mpsc::channel();

        let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);

        let main_addrs = test_harness::http::start_service_with_http(&test_db_name, dirs.main_dir);

        let m_keep_alive = main_addrs.1;
        let main_addrs = main_addrs.0;

        let participant_addrs = test_harness::http::start_service_with_http(&test_db_name, dirs.participant_dir);

        let p_keep_alive = participant_addrs.1;
        let participant_addrs = participant_addrs.0;

        let time = time::Duration::from_secs(1);
        info!("sleeping for 1 seconds...");
        thread::sleep(time);

        let main_contract_desc = custom_contract_description.clone();
        let participant_contract_desc = custom_contract_description;

        let main_db_name = test_db_name;

        let main_db_name_write = main_db_name.clone();
        let db_name_copy = main_db_name_write.clone();

        let ma1 = main_addrs.clone();
        let ma2 = main_addrs.clone();
        let ma3 = main_addrs.clone();

        let pa1 = participant_addrs.clone();
        let pa2 = participant_addrs.clone();
        let pa3 = participant_addrs.clone();

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
            let res = participant_service_client(pa1, participant_contract_desc);
            tx_participant.send(res).unwrap();
        })
        .join()
        .unwrap();

        let participant_accepted_contract = rx_participant.try_recv().unwrap();
        debug!("participant_accpeted_contract: got: {participant_accepted_contract}");

        assert!(participant_accepted_contract);

        thread::spawn(move || {
            let res = main_execute_coop_write_and_read(&main_db_name_write, ma1);
            tx_main_write.send(res).unwrap();
        })
        .join()
        .unwrap();

        let write_and_read_is_successful = rx_main_read.try_recv().unwrap();

        assert!(write_and_read_is_successful);

        thread::spawn(move || {
            let res = participant_rejects_host(pa2);
            tx_p_deny_write.send(res).unwrap();
        })
        .join()
        .unwrap();

        let status_change_is_successful = rx_p_deny_write.try_recv().unwrap();

        assert!(status_change_is_successful);

        thread::spawn(move || {
            let res = main_read_should_fail(&db_name_copy, ma2);
            tx_h_auth_fail.send(res).unwrap();
        })
        .join()
        .unwrap();

        let should_fail = rx_h_auth_fail.try_recv().unwrap();

        assert!(!should_fail);

        let _ = m_keep_alive.send(false);
        let _ = p_keep_alive.send(false);

        test_harness::release_port(ma3.port);
        test_harness::release_port(pa3.port);

        test_harness::shutdown_http(ma3.ip4_addr, ma3.port);
        test_harness::shutdown_http(pa3.ip4_addr, pa3.port);
    }

    #[cfg(test)]
    #[tokio::main]
    async fn main_service_client(
        db_name: &str,
        main_client_addr: &ServiceAddr,
        participant_db_addr: &ServiceAddr,
        contract_desc: &str,
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
    async fn main_execute_coop_write_and_read(
        db_name: &str,
        main_client_addr: ServiceAddr,
    ) -> bool {
        use rcd_enum::database_type::DatabaseType;

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            60,
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

    #[cfg(test)]
    #[tokio::main]
    async fn participant_rejects_host(participant_client_addr: ServiceAddr) -> bool {
        use log::info;

        use rcd_client::RcdClient;
        use rcd_enum::host_status::HostStatus;

        info!(
            "participant_rejects_host attempting to connect {}",
            participant_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            60,
            participant_client_addr.ip4_addr,
            participant_client_addr.port,
        );

        let host_status = HostStatus::Deny;

        let reject_host_result = client
            .change_host_status_by_name("tester", HostStatus::to_u32(host_status))
            .await;

        reject_host_result.unwrap()
    }

    #[cfg(test)]
    #[tokio::main]

    async fn main_read_should_fail(db_name: &str, main_client_addr: ServiceAddr) -> bool {
        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            60,
            main_client_addr.ip4_addr,
            main_client_addr.port,
        );

        let attempt = client
            .try_auth_at_participant("participant", "", db_name)
            .await;

        attempt
    }
}
