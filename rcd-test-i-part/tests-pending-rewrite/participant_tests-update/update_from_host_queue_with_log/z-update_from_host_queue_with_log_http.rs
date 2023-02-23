use crate::common_http::participant_changes_update_behavior;
use crate::test_common::multi::http::http_main_and_participant_setup;
use crate::test_common::HttpTestSetup;
use crate::test_harness::{self, ServiceAddr};
use log::{info, trace};
use rcd_client::RcdClient;
use rcd_enum::updates_from_host_behavior::UpdatesFromHostBehavior;
use std::sync::{mpsc, Arc};
use std::{thread, time};

/*
# Test Description

*/

#[test]
fn test() {
    let test_name = "updates_from_host_queue_with_log_http";
    let db = Arc::new(format!("{}{}", test_name, ".db"));
    let contract = Arc::new(String::from("insert read remote row"));

    let update_statement = "UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999";

    let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);
    let main_test_config = test_harness::http::start_service_with_http(&db, dirs.main_dir);

    let participant_test_config =
        test_harness::http::start_service_with_http(&db, dirs.participant_dir);

    let mca = Arc::new(main_test_config.http_address.clone());
    let pca = Arc::new(participant_test_config.http_address.clone());

    let test_config = HttpTestSetup {
        main_test_config: main_test_config,
        participant_test_config: participant_test_config,
        database_name: &db,
        contract_description: &contract,
    };

    test_harness::sleep_test();

    let common_setup_complete = http_main_and_participant_setup(test_config);
    assert!(common_setup_complete);

    let new_behavior = UpdatesFromHostBehavior::QueueForReviewAndLog;

    {
        let (tx, rx) = mpsc::channel();
        let db = db.clone();
        let pca = pca.clone();

        thread::spawn(move || {
            let res = participant_changes_update_behavior(&db, &pca, new_behavior);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let update_at_participant_is_successful = rx.try_recv().unwrap();

        assert!(update_at_participant_is_successful);
    }

    // main - attempts to execute update but does not get requested value back (this is intentional)
    thread::spawn(move || {
        let res = main_read_updated_row_should_fail(&db_name_copy, ma2, update_statement);
        tx_h_can_read_fail.send(res).unwrap();
    })
    .join()
    .unwrap();

    let can_read_rows = rx_h_can_read_fail.try_recv().unwrap();
    assert!(!can_read_rows);

    // participant - gets pending updates and later accepts the update
    thread::spawn(move || {
        let res =
            participant_get_and_approve_pending_update(&pdn2, "EMPLOYEE", pa3, update_statement);
        tx_p_has_update.send(res).unwrap();
    })
    .join()
    .unwrap();

    let has_and_accept_update = rx_p_has_update.try_recv().unwrap();
    assert!(has_and_accept_update);

    // main - checks the update value again and should match
    thread::spawn(move || {
        let res = main_read_updated_row_should_succed(&db_name_copy_, ma3);
        tx_h_can_read_success.send(res).unwrap();
    })
    .join()
    .unwrap();

    let can_read_rows = rx_h_can_read_success.try_recv().unwrap();
    assert!(can_read_rows);

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
async fn main_execute_coop_write_and_read(db_name: &str, main_client_addr: ServiceAddr) -> bool {
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

    trace!("{data:?}");

    let value = data
        .rows
        .first()
        .unwrap()
        .values
        .first()
        .unwrap()
        .value
        .clone();

    trace!("{value:?}");

    let expected_value = "999".as_bytes().to_vec();

    trace!("{expected_value:?}");

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

    accepted_contract
}


#[cfg(test)]
#[tokio::main]
async fn participant_get_and_approve_pending_update(
    db_name: &str,
    table_name: &str,
    participant_client_addr: ServiceAddr,
    update_statement: &str,
) -> bool {
    use log::info;
    use rcd_client::RcdClient;
    let mut has_statement = false;
    let mut statement_row_id = 0;

    info!(
        "participant_get_and_approve_pending_update attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let mut client = RcdClient::new_http_client(
        String::from("tester"),
        String::from("123456"),
        60,
        participant_client_addr.ip4_addr,
        participant_client_addr.port,
    );

    let pending_updates = client
        .get_pending_actions_at_participant(db_name, table_name, "UPDATE")
        .await
        .unwrap();

    for statement in &pending_updates.pending_statements {
        if statement.statement == update_statement {
            has_statement = true;
            statement_row_id = statement.row_id;
        }
    }

    assert!(has_statement);

    if has_statement {
        trace!("has statement");

        // need to accept the statement
        let accept_update_result = client
            .accept_pending_action_at_participant(db_name, table_name, statement_row_id)
            .await
            .unwrap();

        return accept_update_result.is_successful;
    }

    false
}
