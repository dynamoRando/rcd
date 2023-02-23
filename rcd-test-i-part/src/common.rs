use log::{info, trace};
use rcd_test_harness::test_common::{GrpcTestSetup, HttpTestSetup};
use rcd_test_harness::{RcdClientConfig, ServiceAddr};

pub async fn test_common_grpc(test_config: GrpcTestSetup) {
    let config = test_config.clone();

    let participant_test_config = config.participant_test_config.unwrap();

    let main_client = config.main_client.clone();
    let participant_client = config.participant_client.as_ref().unwrap().clone();
    let participant_db_addr = participant_test_config.database_address.clone();

    let db = config.database_name.clone();
    let contract = config.contract_description.clone();

    {
        let db = db.clone();
        let contract = contract.clone();
        let main_client = main_client.clone();
        let sent_participant_contract =
            main_service_client(&db, &main_client, &participant_db_addr, &contract).await;

        assert!(sent_participant_contract);
    }

    {
        let participant_client = participant_client.clone();
        let contract = contract.clone();

        let participant_accepted_contract =
            participant_service_client(&participant_client, &contract).await;

        info!("participant_accepted_contract: got: {participant_accepted_contract}");

        assert!(participant_accepted_contract);
    }

    {
        let db = db.clone();
        let main_client = main_client.clone();
        let write_and_read_is_successful =
            main_execute_coop_write_and_read(&db, &main_client).await;
        assert!(write_and_read_is_successful);
    }
}

pub async fn test_common_http(test_config: HttpTestSetup) {
    let config = test_config.clone();

    let main_client = config.main_client.clone();
    let participant_client = config.participant_client.as_ref().unwrap().clone();
    let participant_db_addr = config.participant_test_config.http_address.clone();

    let db = config.database_name.clone();
    let contract = config.contract_description.clone();

    {
        let db = db.clone();
        let contract = contract.clone();
        let main_client = main_client.clone();
        let participant_db_addr = participant_db_addr.clone();
        let sent_participant_contract =
            main_service_client(&db, &main_client, &participant_db_addr, &contract).await;

        assert!(sent_participant_contract);
    }

    {
        let participant_client = participant_client.clone();
        let contract = contract.clone();

        let participant_accepted_contract =
            participant_service_client(&participant_client, &contract).await;

        info!("participant_accepted_contract: got: {participant_accepted_contract}");

        assert!(participant_accepted_contract);
    }

    {
        let db = db.clone();
        let main_client = main_client.clone();
        let write_and_read_is_successful =
            main_execute_coop_write_and_read(&db, &main_client).await;
        assert!(write_and_read_is_successful);
    }
}

pub async fn main_service_client(
    db_name: &str,
    config: &RcdClientConfig,
    participant_db_addr: &ServiceAddr,
    contract_desc: &str,
) -> bool {
    use rcd_enum::database_type::DatabaseType;
    use rcd_enum::logical_storage_policy::LogicalStoragePolicy;
    use rcd_enum::remote_delete_behavior::RemoteDeleteBehavior;

    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    let mut client = rcd_test_harness::get_rcd_client(&config).await;

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

pub async fn main_execute_coop_write_and_read(db_name: &str, config: &RcdClientConfig) -> bool {
    use rcd_enum::database_type::DatabaseType;

    let mut client = rcd_test_harness::get_rcd_client(&config).await;

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

pub async fn participant_service_client(config: &RcdClientConfig, contract_desc: &str) -> bool {
    let mut has_contract = false;

    let mut client = rcd_test_harness::get_rcd_client(&config).await;

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
