use crate::test_harness::{self, ServiceAddr};
use log::info;
use rcdclient::RcdClient;
use std::sync::mpsc;
use std::{thread, time};

/*
# Test Description
*/

#[test]
fn test() {
    let test_name = "get_db_names";
    let test_db_name = format!("{}{}", test_name, ".db");
    let custom_contract_description = String::from("db names");

    let (tx_main, rx_main) = mpsc::channel();
    let (tx_participant, rx_participant) = mpsc::channel();
    let (tx_main_write, rx_main_read) = mpsc::channel();
    let (tx_p_has_dbs, rx_p_has_dbs) = mpsc::channel();
    let (tx_h_has_dbs, rx_h_has_dbs) = mpsc::channel();

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
        "participant_accepted_contract: got: {}",
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
        let res = participant_get_databases(addr_1);
        tx_p_has_dbs.send(res).unwrap();
    })
    .join()
    .unwrap();

    let p_has_all_dbs = rx_p_has_dbs.try_recv().unwrap();

    assert!(p_has_all_dbs);

    thread::spawn(move || {
        let res = main_get_databases(&db_name_copy, addr);
        tx_h_has_dbs.send(res).unwrap();
    })
    .join()
    .unwrap();

    let h_has_all_dbs = rx_h_has_dbs.try_recv().unwrap();

    assert!(h_has_all_dbs);

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

    client.create_user_database(db_name).await.unwrap();
    client
        .create_user_database("get_db_names2.db")
        .await
        .unwrap();
    client
        .create_user_database("get_db_names3.db")
        .await
        .unwrap();
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
#[allow(dead_code, unused_variables)]
async fn participant_service_client(
    db_name: &str,
    participant_client_addr: ServiceAddr,
    contract_desc: String,
) -> bool {
    use log::info;
    use rcdclient::RcdClient;
    use rcd_common::rcd_enum::DatabaseType;

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

    client
        .create_user_database("part_example.db")
        .await
        .unwrap();
    client
        .create_user_database("part_example2.db")
        .await
        .unwrap();
    client
        .create_user_database("part_example3.db")
        .await
        .unwrap();

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
async fn participant_get_databases(participant_client_addr: ServiceAddr) -> bool {
    use rcdclient::RcdClient;

    let has_all_databases = true;

    println!(
        "participant_get_databases attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let client = RcdClient::new(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
    );

    let result = client.get_databases().await;

    let dbs_reply = result.unwrap();

    let mut actual_db_names: Vec<String> = Vec::new();

    println!("actual names");

    for db in &dbs_reply.databases {
        println!("{}", db.database_name.clone());
        actual_db_names.push(db.database_name.clone());
    }

    let mut expected_db_names: Vec<String> = Vec::new();
    expected_db_names.push("part_example.db".to_string());
    expected_db_names.push("part_example2.db".to_string());
    expected_db_names.push("part_example3.db".to_string());
    expected_db_names.push("get_db_names.dbpart".to_string());
    expected_db_names.push("rcd.db".to_string());

    println!("expected names");
    for name in &expected_db_names {
        println!("{}", name);
    }

    for name in &expected_db_names {
        if !actual_db_names.contains(name) {
            return false;
        }
    }

    return has_all_databases;
}

#[cfg(test)]
#[tokio::main]
#[allow(unused_variables)]
async fn main_get_databases(db_name: &str, main_client_addr: ServiceAddr) -> bool {
    let has_all_databases = true;

    let client = RcdClient::new(
        main_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
    );

    println!("main_get_databases");

    let result = client.get_databases().await;

    let dbs_reply = result.unwrap();

    let mut actual_db_names: Vec<String> = Vec::new();

    println!("actual names");

    for db in &dbs_reply.databases {
        println!("{}", db.database_name.clone());
        actual_db_names.push(db.database_name.clone());
    }

    let mut expected_db_names: Vec<String> = Vec::new();
    expected_db_names.push("get_db_names2.db".to_string());
    expected_db_names.push("get_db_names3.db".to_string());
    expected_db_names.push("get_db_names.db".to_string());
    expected_db_names.push("rcd.db".to_string());

    println!("expected names");
    for name in &expected_db_names {
        println!("{}", name);
    }

    for name in &expected_db_names {
        if !actual_db_names.contains(name) {
            return false;
        }
    }

    return has_all_databases;
}
