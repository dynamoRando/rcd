use crate::test_harness::{self, ServiceAddr};
use log::info;
use rcdclient::RcdClient;
use rcdx::rcd_enum::DeletesFromHostBehavior;
use std::sync::mpsc;
use std::{thread, time};

/*
# Test Description

## Purpose:
This test checks to see if the setting at the participant for DELETES_FROM_HOST_BEHAVIOR in the rcd table CDS_CONTRACTS_TABLES
is being respected.

## Feature Background
We want to make sure the participants have full authority over their data. This means that they have the option to change
how modifications being sent from the host are handled. In this test, if a host sends an DELETE statmement to be processed
at the participant, we want to ignore it.

## Test Steps
- Start an rcd instance for a main (host) and a participant
- Host:
    - Generate a db and tables and a contract to send to particpant
- Participant:
    - Accept contract
- Host:
    - Send one row to participant to be inserted and test to make sure can read from participant
- Participant:
    - Change DeletesFromHostBehavior to Ignore
- Host:
    - Attempt to delete previously inserted row

### Expected Results:
The delete should fail.

*/

#[test]
fn test() {
    let test_name = "delta_delete_from_host";
    let test_db_name = format!("{}{}", test_name, ".db");
    let custom_contract_description = String::from("insert read remote row");

    let (tx_main, rx_main) = mpsc::channel();
    let (tx_participant, rx_participant) = mpsc::channel();
    let (tx_main_write, rx_main_read) = mpsc::channel();
    let (tx_p_deny_write, rx_p_deny_write) = mpsc::channel();
    let (tx_h_auth_fail, rx_h_auth_fail) = mpsc::channel();

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
    let main_db_name_write = main_db_name.clone();
    let db_name_copy = main_db_name_write.clone();

    let addr_1 = participant_addrs.0.clone();

    let main_srv_addr = main_addrs.0.clone();
    let addr = main_srv_addr.clone();

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

    let new_behavior = DeletesFromHostBehavior::Ignore;

    thread::spawn(move || {
        let res = participant_changes_delete_behavior(&pdn, addr_1, new_behavior);
        tx_p_deny_write.send(res).unwrap();
    })
    .join()
    .unwrap();

    let status_change_is_successful = rx_p_deny_write.try_recv().unwrap();

    assert!(status_change_is_successful);

    thread::spawn(move || {
        let res = main_delete_should_fail(&db_name_copy, addr);
        tx_h_auth_fail.send(res).unwrap();
    })
    .join()
    .unwrap();

    let should_fail = rx_h_auth_fail.try_recv().unwrap();

    assert!(!should_fail);

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
async fn participant_changes_delete_behavior(
    db_name: &str,
    participant_client_addr: ServiceAddr,
    behavior: DeletesFromHostBehavior,
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

    let result = client
        .change_deletes_from_host_behavior(db_name, "EMPLOYEE", behavior)
        .await;

    return result.unwrap();
}

#[cfg(test)]
#[tokio::main]
#[allow(unused_variables)]
async fn main_delete_should_fail(db_name: &str, main_client_addr: ServiceAddr) -> bool {
    let client = RcdClient::new(
        main_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
    );

    let cmd = String::from("DELETE FROM EMPLOYEE WHERE Id = 999");
    let update_result = client
        .execute_cooperative_write_at_host(db_name, &cmd, "participant", "Id = 999")
        .await;
    return update_result.unwrap();
}
