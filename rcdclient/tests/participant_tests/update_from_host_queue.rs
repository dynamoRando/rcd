use crate::test_harness::ServiceAddr;
use log::info;
use rcdclient::RcdClient;
use rcdx::rcd_enum::UpdatesFromHostBehavior;
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

    let (tx_main, rx_main) = mpsc::channel();
    let (tx_participant, rx_participant) = mpsc::channel();
    let (tx_main_write, rx_main_read) = mpsc::channel();
    let (tx_p_change_update, rx_p_change_update) = mpsc::channel();
    let (tx_h_can_read, rx_h_can_read) = mpsc::channel();

    let (tx_h_data_hash, rx_h_data_hash) = mpsc::channel();
    let (tx_p_data_hash, rx_p_data_hash) = mpsc::channel();

    let (tx_p_row_id, rx_p_row_id) = mpsc::channel();
    let (tx_p_read_data_log, rx_p_read_data_log) = mpsc::channel();

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
    let pdn = participant_db_name.clone();
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
    let p_addr = participant_addrs.clone().0;
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

    thread::spawn(move || {
        let res = main_execute_coop_write_and_read(&main_db_name_write, main_srv_addr);
        tx_main_write.send(res).unwrap();
    })
    .join()
    .unwrap();

    let write_and_read_is_successful = rx_main_read.try_recv().unwrap();

    assert!(write_and_read_is_successful);

    let new_behavior = UpdatesFromHostBehavior::QueueForReview;

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
        let res = get_data_logs_at_participant(&db_pname3, p_addr3, rid);
        tx_p_read_data_log.send(res).unwrap();
    })
    .join()
    .unwrap();

    let p_read_data_log_is_correct = rx_p_read_data_log.try_recv().unwrap();

    assert!(p_read_data_log_is_correct);
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
    use rcdclient::RcdClient;
    use rcdx::rcd_enum::LogicalStoragePolicy;
    use rcdx::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};

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
#[allow(unused_variables)]
async fn main_execute_coop_write_and_read(db_name: &str, main_client_addr: ServiceAddr) -> bool {
    use rcdx::rcd_enum::DatabaseType;

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
#[allow(dead_code, unused_variables)]
async fn participant_service_client(
    db_name: &str,
    participant_client_addr: ServiceAddr,
    contract_desc: String,
) -> bool {
    use log::info;
    use rcdclient::RcdClient;
    use rcdx::rcd_enum::DatabaseType;

    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);
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

    let is_generated_host = client.generate_host_info("participant").await.unwrap();

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
#[allow(dead_code, unused_variables)]
async fn participant_changes_update_behavior(
    db_name: &str,
    participant_client_addr: ServiceAddr,
    behavior: UpdatesFromHostBehavior,
) -> bool {
    use log::info;
    use rcdclient::RcdClient;
    use rcdx::rcd_enum::DatabaseType;

    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

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
#[allow(dead_code, unused_variables)]
async fn get_row_id_at_participant(db_name: &str, participant_client_addr: ServiceAddr) -> u32 {
    use log::info;
    use rcdclient::RcdClient;
    use rcdx::rcd_enum::DatabaseType;

    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    info!(
        "get_data_hash_for_changed_row_at_participant attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let client = RcdClient::new(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
    );

    let row_ids = client
        .get_row_id_at_participant(db_name, "EMPLOYEE", "NAME = 'TESTER'")
        .await
        .unwrap();
    let row_id = row_ids.first().unwrap().clone();

    return row_id;
}

#[cfg(test)]
#[tokio::main]
#[allow(dead_code, unused_variables)]
async fn get_data_hash_for_changed_row_at_participant(
    db_name: &str,
    participant_client_addr: ServiceAddr,
    row_id: u32,
) -> u64 {
    use log::info;
    use rcdclient::RcdClient;
    use rcdx::rcd_enum::DatabaseType;

    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    info!(
        "get_data_hash_for_changed_row_at_participant attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let client = RcdClient::new(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
    );

    let data_hash_result = client
        .get_data_hash_at_participant(db_name, "EMPLOYEE", row_id)
        .await
        .unwrap();

    return data_hash_result;
}

#[cfg(test)]
#[tokio::main]
#[allow(dead_code, unused_variables)]
async fn get_data_hash_for_changed_row_at_host(
    db_name: &str,
    participant_client_addr: ServiceAddr,
    row_id: u32,
) -> u64 {
    use log::info;
    use rcdclient::RcdClient;
    use rcdx::rcd_enum::DatabaseType;

    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    info!(
        "get_data_hash_for_changed_row_at_participant attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let client = RcdClient::new(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
    );

    let data_hash_result = client
        .get_data_hash_at_host(db_name, "EMPLOYEE", row_id)
        .await
        .unwrap();

    return data_hash_result;
}

#[cfg(test)]
#[tokio::main]
#[allow(unused_variables)]
async fn main_read_updated_row_should_succeed(
    db_name: &str,
    main_client_addr: ServiceAddr,
) -> bool {
    use rcdx::rcd_enum::DatabaseType;

    let client = RcdClient::new(
        main_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
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

    return *value == expected_value;
}

#[cfg(test)]
#[tokio::main]
#[allow(dead_code, unused_variables)]
async fn get_data_logs_at_participant(
    db_name: &str,
    participant_client_addr: ServiceAddr,
    row_id: u32,
) -> bool {
    use log::info;
    use rcdclient::RcdClient;
    use rcdx::rcd_enum::DatabaseType;

    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    info!(
        "get_data_logs_at_participant attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let client = RcdClient::new(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
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

    return *value == expected_value;
}
