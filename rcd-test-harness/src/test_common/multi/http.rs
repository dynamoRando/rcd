use crate::test_common::HttpTestSetup;
use crate::ServiceAddr;
use log::{info, trace};
use rcd_client::RcdClient;
use std::sync::{mpsc, Arc};
use std::thread;

pub fn http_main_and_participant_setup(test_config: HttpTestSetup) -> bool {
    let config = test_config.clone();

    let main_test_config = config.main_test_config;
    let participant_test_config = config.participant_test_config;

    let main_client_addr = Arc::new(main_test_config.http_address.clone());
    let participant_client_addr = Arc::new(participant_test_config.http_address.clone());
    let participant_db_addr = participant_client_addr.clone();

    let db = Arc::new(config.database_name.clone());
    let contract = Arc::new(config.contract_description.clone());

    {
        let (tx, rx) = mpsc::channel();

        let db = db.clone();
        let contract = contract.clone();
        let main_client_addr = main_client_addr.clone();

        thread::spawn(move || {
            let res = main_service_client(&db, &main_client_addr, &participant_db_addr, &contract);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let sent_participant_contract = rx.try_recv().unwrap();
        trace!("send_participant_contract: got: {sent_participant_contract}");

        assert!(sent_participant_contract);
    }

    {
        let (tx, rx) = mpsc::channel();
        let participant_client_addr = participant_client_addr.clone();
        let contract = contract.clone();

        thread::spawn(move || {
            let res = participant_service_client(&participant_client_addr, &contract);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let participant_accepted_contract = rx.try_recv().unwrap();
        trace!("participant_accpeted_contract: got: {participant_accepted_contract}");

        assert!(participant_accepted_contract);
    }

    {
        let (tx, rx) = mpsc::channel();

        let db = db.clone();
        let main_client_addr = main_client_addr.clone();

        thread::spawn(move || {
            let res = main_execute_coop_write_and_read(&db, &main_client_addr);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let write_and_read_is_successful = rx.try_recv().unwrap();
        assert!(write_and_read_is_successful);
    }

    true
}

#[tokio::main]
pub async fn main_service_client(
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

#[tokio::main]
pub async fn main_execute_coop_write_and_read(
    db_name: &str,
    main_client_addr: &ServiceAddr,
) -> bool {
    use rcd_enum::database_type::DatabaseType;

    let mut client = RcdClient::new_http_client(
        String::from("tester"),
        String::from("123456"),
        60,
        main_client_addr.ip4_addr.clone(),
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

#[tokio::main]
pub async fn participant_service_client(
    participant_client_addr: &ServiceAddr,
    contract_desc: &str,
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
        participant_client_addr.ip4_addr.clone(),
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
