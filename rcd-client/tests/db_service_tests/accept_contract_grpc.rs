use crate::test_harness::{self, ServiceAddr};
use log::{debug, info};
use std::sync::mpsc;
use std::thread;

/*
# Test Description

## Purpose:
This test checks to see if we can have a participant accept a contract generated by a host.

## Feature Background
We want to make sure that rcd's cooperative features are working. In this case, this means that we can successfully generate a contract for a database
and also send it to a participant to be accepted.

## Test Steps
- Start an rcd instance for a main (host) and a participant
- Host:
    - Generate a db and tables and a contract to send to particpant
- Participant:
    - Accept contract

### Expected Results:
The participant should be able to accept the pending contract.

*/

#[test]
fn test() {
    test_harness::init_log_to_screen(log::LevelFilter::Info);

    let test_name = "sa_contract_grpc";
    let test_db_name = format!("{}{}", test_name, ".db");
    let custom_contract_description = String::from("This is a custom description from test");

    let (tx_main, rx_main) = mpsc::channel();
    let (tx_participant, rx_participant) = mpsc::channel();

    let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);

    let main_test_config = test_harness::start_service_with_grpc(&test_db_name, dirs.main_dir);
    let participant_test_config =
        test_harness::start_service_with_grpc(&test_db_name, dirs.participant_dir);

    test_harness::sleep_test();

    {
        let main_client_addr = main_test_config.client_address.clone();
        let participant_db_addr = participant_test_config.database_address.clone();
        let main_contract_desc = custom_contract_description.clone();
        let main_db_name = test_db_name.clone();
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
        let participant_contract_desc = custom_contract_description.clone();
        let participant_client_addr = participant_test_config.client_address.clone();
        thread::spawn(move || {
            let res =
                participant_service_client(&participant_client_addr, &participant_contract_desc);
            tx_participant.send(res).unwrap();
        })
        .join()
        .unwrap();
    }

    let participant_accepted_contract = rx_participant.try_recv().unwrap();
    debug!("participant_accpeted_contract: got: {participant_accepted_contract}");

    assert!(participant_accepted_contract);

    test_harness::shutdown_grpc_test(&main_test_config, &participant_test_config);
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
async fn participant_service_client(
    participant_client_addr: &ServiceAddr,
    contract_desc: &str,
) -> bool {
    use rcd_client::RcdClient;
    let mut has_contract = false;

    info!(
        "main_service_client attempting to connect {}",
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
