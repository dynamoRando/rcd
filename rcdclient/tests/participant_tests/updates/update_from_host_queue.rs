use crate::test_harness::{self, ServiceAddr};
use log::info;
use rcdclient::RcdClient;
use rcd_common::rcd_enum::UpdatesFromHostBehavior;
use std::sync::mpsc;
use std::{thread, time};

/*
# Test Description

*/

#[test]
fn test() {
    let test_name = "update_from_host_queue";
    let test_db_name = format!("{}{}", test_name, ".db");
    let custom_contract_description = String::from("insert read remote row");

    let update_statement = "UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999";
    let update_statement2 = update_statement.clone();
    let update_statement3 = update_statement.clone();

    let (tx_main, rx_main) = mpsc::channel();
    let (tx_participant, rx_participant) = mpsc::channel();
    let (tx_main_write, rx_main_read) = mpsc::channel();
    let (tx_p_change_update, rx_p_change_update) = mpsc::channel();
    let (tx_h_can_read_fail, rx_h_can_read_fail) = mpsc::channel();
    let (tx_h_can_read_success, rx_h_can_read_success) = mpsc::channel();

    let (tx_p_has_update, rx_p_has_update) = mpsc::channel();

    let dirs = test_harness::get_test_temp_dir_main_and_participant(&test_name);

    let main_addrs = test_harness::start_service(&test_db_name, dirs.1);

    let main_addr_client_port = main_addrs.2;
    let main_addr_db_port = main_addrs.3;

    let main_client_shutdown_trigger = main_addrs.4;
    let main_db_shutdown_triger = main_addrs.5;

    let participant_addrs = test_harness::start_service(&test_db_name, dirs.2);

    let part_addr_client_port = participant_addrs.2;
    let part_addr_db_port = participant_addrs.3;

    let part_client_shutdown_trigger = participant_addrs.4;
    let part_db_shutdown_trigger = participant_addrs.5;

    let time = time::Duration::from_secs(1);

    info!("sleeping for 1 seconds...");

    thread::sleep(time);

    let main_contract_desc = custom_contract_description.clone();
    let participant_contract_desc = custom_contract_description.clone();
    let main_db_name = test_db_name.clone();
    let participant_db_name = test_db_name.clone();
    let pdn = participant_db_name.clone();
    let pdn2 = pdn.clone();
    let main_db_name_write = main_db_name.clone();
    let db_name_copy = main_db_name_write.clone();
    let db_name_copy_ = db_name_copy.clone();
    let addr_1 = participant_addrs.0.clone();
    let addr_1_1 = addr_1.clone();
    let main_srv_addr = main_addrs.0.clone();
    let addr = main_srv_addr.clone();
    let addr_ = main_srv_addr.clone();

    // main - normal database setup
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

    // main - setup contract
    let sent_participant_contract = rx_main.try_recv().unwrap();
    println!(
        "send_participant_contract: got: {}",
        sent_participant_contract
    );

    assert!(sent_participant_contract);

    // participant - accept contract
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

    // main - inserts remote row and tests to make sure it works
    thread::spawn(move || {
        let res = main_execute_coop_write_and_read(&main_db_name_write, main_srv_addr);
        tx_main_write.send(res).unwrap();
    })
    .join()
    .unwrap();

    let write_and_read_is_successful = rx_main_read.try_recv().unwrap();

    assert!(write_and_read_is_successful);

    let new_behavior = UpdatesFromHostBehavior::QueueForReview;

    // participant - changes behavior to log updates but not execute them
    thread::spawn(move || {
        let res = participant_changes_update_behavior(&pdn, addr_1, new_behavior);
        tx_p_change_update.send(res).unwrap();
    })
    .join()
    .unwrap();

    let update_at_participant_is_successful = rx_p_change_update.try_recv().unwrap();

    assert!(update_at_participant_is_successful);

    // main - attempts to execute update but does not get requested value back (this is intentional)
    thread::spawn(move || {
        let res = main_read_updated_row_should_fail(&db_name_copy, addr, update_statement2);
        tx_h_can_read_fail.send(res).unwrap();
    })
    .join()
    .unwrap();

    let can_read_rows = rx_h_can_read_fail.try_recv().unwrap();
    assert!(!can_read_rows);

    // participant - gets pending updates and later accepts the update
    thread::spawn(move || {
        let res = participant_get_and_approve_pending_update(
            &pdn2,
            "EMPLOYEE",
            addr_1_1,
            update_statement3,
        );
        tx_p_has_update.send(res).unwrap();
    })
    .join()
    .unwrap();

    let has_and_accept_update = rx_p_has_update.try_recv().unwrap();
    assert!(has_and_accept_update);

    // main - checks the update value again and should match
    thread::spawn(move || {
        let res = main_read_updated_row_should_succed(&db_name_copy_, addr_, update_statement3);
        tx_h_can_read_success.send(res).unwrap();
    })
    .join()
    .unwrap();

    let can_read_rows = rx_h_can_read_success.try_recv().unwrap();
    assert!(can_read_rows);

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
    use rcdclient::RcdClient;
    use rcd_common::rcd_enum::LogicalStoragePolicy;
    use rcd_common::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};

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

    let client = RcdClient::new(
        main_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
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
    use rcdclient::RcdClient;

    let mut has_contract = false;

    info!(
        "participant_service_client attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let client = RcdClient::new(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
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
async fn participant_changes_update_behavior(
    db_name: &str,
    participant_client_addr: ServiceAddr,
    behavior: UpdatesFromHostBehavior,
) -> bool {
    use log::info;
    use rcdclient::RcdClient;

    info!(
        "participant_changes_update_behavior attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let client = RcdClient::new(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
    );

    let change_update_behavior = client
        .change_updates_from_host_behavior(db_name, "EMPLOYEE", behavior)
        .await;

    return change_update_behavior.unwrap();
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
    use rcdclient::RcdClient;
    let mut has_statement = false;
    let mut statement_row_id = 0;

    info!(
        "participant_get_and_approve_pending_update attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let client = RcdClient::new(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
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
        println!("has statement");

        // need to accept the statement
        let accept_update_result = client
            .accept_pending_action_at_participant(db_name, table_name, statement_row_id)
            .await
            .unwrap();

        return accept_update_result.is_successful;
    }

    return false;
}

#[cfg(test)]
#[tokio::main]
#[allow(unused_variables)]
async fn main_read_updated_row_should_fail(
    db_name: &str,
    main_client_addr: ServiceAddr,
    update_statement: &str,
) -> bool {
    use rcd_common::rcd_enum::DatabaseType;

    let client = RcdClient::new(
        main_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
    );

    let update_result = client
        .execute_cooperative_write_at_host(db_name, &update_statement, "participant", "ID = 999")
        .await;

    println!("{:?}", update_result);

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

    return *value == expected_value;
}

#[cfg(test)]
#[tokio::main]
#[allow(unused_variables)]
async fn main_read_updated_row_should_succed(
    db_name: &str,
    main_client_addr: ServiceAddr,
    update_statement: &str,
) -> bool {
    use rcd_common::rcd_enum::DatabaseType;

    let client = RcdClient::new(
        main_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
    );

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

    return *value == expected_value;
}
