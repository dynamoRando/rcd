use crate::test_harness::ServiceAddr;
use log::info;
use rcd::rcd_sql_client::RcdClient;
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
    let test_name = "reject_host";
    let test_db_name = format!("{}{}", test_name, ".db");
    let custom_contract_description = String::from("insert read remote row");

    let (tx_main, rx_main) = mpsc::channel();
    let (tx_participant, rx_participant) = mpsc::channel();
    let (tx_main_write, rx_main_read) = mpsc::channel();
    let (tx_p_deny_write, rx_p_deny_write) = mpsc::channel();
    let (tx_h_auth_fail, rx_h_auth_fail) = mpsc::channel();

    let dirs = super::test_harness::get_test_temp_dir_main_and_participant(&test_name);

    let main_addrs = super::test_harness::start_service(&test_db_name, dirs.1);
    let participant_addrs = super::test_harness::start_service(&test_db_name, dirs.2);

    let time = time::Duration::from_secs(5);

    info!("sleeping for 5 seconds...");

    thread::sleep(time);

    let main_contract_desc = custom_contract_description.clone();
    let participant_contract_desc = custom_contract_description.clone();
    let pcd = participant_contract_desc.clone();
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

    thread::spawn(move || {
        let res = participant_rejects_host(&pdn, addr_1, pcd);
        tx_p_deny_write.send(res).unwrap();
    })
    .join()
    .unwrap();

    let status_change_is_successful = rx_p_deny_write.try_recv().unwrap();

    assert!(status_change_is_successful);

    thread::spawn(move || {
        let res = main_read_should_fail(&db_name_copy, addr);
        tx_h_auth_fail.send(res).unwrap();
    })
    .join()
    .unwrap();

    let should_fail = rx_h_auth_fail.try_recv().unwrap();

    assert!(!should_fail);
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
    use rcd::rcd_enum::LogicalStoragePolicy;
    use rcd::rcd_sql_client::RcdClient;
    use rcd::{rcd_enum::DatabaseType, rcd_enum::RemoteDeleteBehavior};

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
        .execute_write(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type)
        .await
        .unwrap();

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    client
        .execute_write(db_name, &create_table_statement, database_type)
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
    use rcd::rcd_enum::DatabaseType;

    let client = RcdClient::new(
        main_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
    );

    client
        .execute_cooperative_write(
            db_name,
            "INSERT INTO EMPLOYEE ( Id, Name ) VALUES ( 999, 'ASDF');",
            "participant",
            "",
        )
        .await
        .unwrap();

    let data = client
        .execute_read(
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
    use rcd::rcd_enum::DatabaseType;
    use rcd::rcd_sql_client::RcdClient;

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
async fn participant_rejects_host(
    db_name: &str,
    participant_client_addr: ServiceAddr,
    contract_desc: String,
) -> bool {
    use log::info;
    use rcd::rcd_enum::{DatabaseType, HostStatus};
    use rcd::rcd_sql_client::RcdClient;

    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    info!(
        "participant_rejects_host attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let client = RcdClient::new(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
    );

    let host_status = HostStatus::Deny;

    let reject_host_result = client
        .change_host_status_by_name("tester", HostStatus::to_u32(host_status))
        .await;

    return reject_host_result.unwrap();
}

#[cfg(test)]
#[tokio::main]
#[allow(unused_variables)]
async fn main_read_should_fail(db_name: &str, main_client_addr: ServiceAddr) -> bool {
    let client = RcdClient::new(
        main_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
    );

    let attempt = client.try_auth_at_participant("participant", "", db_name).await;

    return attempt;
}
