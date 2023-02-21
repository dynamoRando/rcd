use crate::test_harness::ServiceAddr;
use log::{debug};
use rcd_client::RcdClient;

#[cfg(test)]
#[tokio::main]
pub async fn main_service_client(
    db_name: &str,
    main_client: &RcdClient,
    participant_db_addr: &ServiceAddr,
    contract_desc: &str,
) -> bool {
    use rcd_enum::database_type::DatabaseType;
    use rcd_enum::logical_storage_policy::LogicalStoragePolicy;
    use rcd_enum::remote_delete_behavior::RemoteDeleteBehavior;
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    let mut main_client = (*main_client).clone();

    main_client.create_user_database(db_name).await.unwrap();
    main_client
        .enable_cooperative_features(db_name)
        .await
        .unwrap();
    main_client
        .execute_write_at_host(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type, "")
        .await
        .unwrap();

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    main_client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    let logical_storage_policy = LogicalStoragePolicy::ParticpantOwned;

    main_client
        .set_logical_storage_policy(db_name, "EMPLOYEE", logical_storage_policy)
        .await
        .unwrap();

    let behavior = RemoteDeleteBehavior::Ignore;

    main_client
        .generate_contract(db_name, "tester", &contract_desc, behavior)
        .await
        .unwrap();

    main_client
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

    return main_client
        .send_participant_contract(db_name, "participant")
        .await
        .unwrap();
}

#[cfg(test)]
#[tokio::main]
pub async fn participant_service_client(
    participant_client: &RcdClient,
    contract_desc: &str,
) -> bool {
    let mut participant_client = (*participant_client).clone();

    let mut has_contract = false;

    participant_client
        .generate_host_info("participant")
        .await
        .unwrap();

    let pending_contracts = participant_client.view_pending_contracts().await.unwrap();

    for contract in &pending_contracts {
        if contract.description == contract_desc {
            has_contract = true;
            break;
        }
    }

    let mut accepted_contract = false;

    if has_contract {
        accepted_contract = participant_client
            .accept_pending_contract("tester")
            .await
            .unwrap();
    }

    accepted_contract
}

#[cfg(test)]
#[tokio::main]
pub async fn main_execute_coop_write_and_read(db_name: &str, main_client: &RcdClient) -> bool {
    use rcd_enum::database_type::DatabaseType;
    let mut main_client = (*main_client).clone();

    main_client
        .execute_cooperative_write_at_host(
            db_name,
            "INSERT INTO EMPLOYEE ( Id, Name ) VALUES ( 999, 'ASDF');",
            "participant",
            "",
        )
        .await
        .unwrap();

    let data = main_client
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
