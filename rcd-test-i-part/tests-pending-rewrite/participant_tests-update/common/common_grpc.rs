use crate::test_harness::ServiceAddr;
use log::{info, trace};
use rcd_client::RcdClient;
use rcd_enum::updates_from_host_behavior::UpdatesFromHostBehavior;

#[cfg(test)]
#[cfg(test)]
#[tokio::main]
pub async fn participant_changes_update_behavior(
    db_name: &str,
    participant_client_addr: &ServiceAddr,
    behavior: UpdatesFromHostBehavior,
) -> bool {
    use log::info;

    use rcd_client::RcdClient;

    info!(
        "participant_changes_update_behavior attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let mut client = RcdClient::new_grpc_client(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
        60,
    )
    .await;

    let change_update_behavior = client
        .change_updates_from_host_behavior(db_name, "EMPLOYEE", behavior)
        .await;

    change_update_behavior.unwrap()
}

#[cfg(test)]
#[tokio::main]
pub async fn main_read_updated_row_should_succeed(
    db_name: &str,
    main_client_addr: &ServiceAddr,
) -> bool {
    use rcd_enum::database_type::DatabaseType;

    let mut client = RcdClient::new_grpc_client(
        main_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
        60,
    )
    .await;

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

    trace!("{value:?}");

    let expected_value = "TESTER".as_bytes().to_vec();

    trace!("{expected_value:?}");

    *value == expected_value
}

#[cfg(test)]
#[tokio::main]
pub async fn get_row_id_at_participant(
    db_name: &str,
    participant_client_addr: &ServiceAddr,
) -> u32 {
    use log::info;

    use rcd_client::RcdClient;

    info!(
        "get_data_hash_for_changed_row_at_participant attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let mut client = RcdClient::new_grpc_client(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
        60,
    )
    .await;

    let row_ids = client
        .get_row_id_at_participant(db_name, "EMPLOYEE", "NAME = 'TESTER'")
        .await
        .unwrap();
    let row_id = *row_ids.first().unwrap();

    row_id
}

#[cfg(test)]
#[tokio::main]
pub async fn get_data_hash_for_changed_row_at_participant(
    db_name: &str,
    participant_client_addr: &ServiceAddr,
    row_id: u32,
) -> u64 {
    use log::info;

    use rcd_client::RcdClient;

    info!(
        "get_data_hash_for_changed_row_at_participant attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let mut client = RcdClient::new_grpc_client(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
        60,
    )
    .await;

    let data_hash_result = client
        .get_data_hash_at_participant(db_name, "EMPLOYEE", row_id)
        .await
        .unwrap();

    data_hash_result
}

#[cfg(test)]
#[tokio::main]
pub async fn get_data_hash_for_changed_row_at_host(
    db_name: &str,
    main_client_addr: &ServiceAddr,
    row_id: u32,
) -> u64 {
    use log::info;

    use rcd_client::RcdClient;

    info!(
        "get_data_hash_for_changed_row_at_participant attempting to connect {}",
        main_client_addr.to_full_string_with_http()
    );

    let mut client = RcdClient::new_grpc_client(
        main_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
        60,
    )
    .await;

    let data_hash_result = client
        .get_data_hash_at_host(db_name, "EMPLOYEE", row_id)
        .await
        .unwrap();

    data_hash_result
}

#[cfg(test)]
#[tokio::main]
pub async fn get_data_logs_at_participant(
    db_name: &str,
    participant_client_addr: &ServiceAddr,
) -> bool {
    use log::info;
    use rcd_client::RcdClient;
    use rcd_enum::database_type::DatabaseType;

    info!(
        "get_data_logs_at_participant attempting to connect {}",
        participant_client_addr.to_full_string_with_http()
    );

    let mut client = RcdClient::new_grpc_client(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
        60,
    )
    .await;

    let cmd = "SELECT * FROM EMPLOYEE_COOP_DATA_LOG";
    let read_result = client
        .execute_read_at_participant(db_name, cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
        .await
        .unwrap();

    trace!("{read_result:?}");

    let row = read_result.rows.first().unwrap();
    let value = &row.values[1].value.clone();

    trace!("{value:?}");

    let expected_value = "ASDF".as_bytes().to_vec();
    trace!("{expected_value:?}");

    *value == expected_value
}

#[cfg(test)]
#[tokio::main]

pub async fn main_read_updated_row_should_fail(
    db_name: &str,
    main_client_addr: &ServiceAddr,
    update_statement: &str,
) -> bool {
    use rcd_enum::database_type::DatabaseType;

    let mut client = RcdClient::new_grpc_client(
        main_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
        60,
    )
    .await;

    let update_result = client
        .execute_cooperative_write_at_host(db_name, update_statement, "participant", "ID = 999")
        .await;

    trace!("{update_result:?}");

    assert!(update_result.unwrap());

    let cmd = String::from("SELECT NAME FROM EMPLOYEE WHERE Id = 999");
    let read_result = client
        .execute_read_at_host(db_name, &cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
        .await;

    let results = read_result.unwrap();

    let row = results.rows.first().unwrap();

    let value = &row.values[1].value.clone();

    trace!("{value:?}");

    let expected_value = "TESTER".as_bytes().to_vec();

    trace!("{expected_value:?}");

    *value == expected_value
}

#[cfg(test)]
#[tokio::main]
pub async fn participant_get_and_approve_pending_update(
    db_name: &str,
    table_name: &str,
    participant_client_addr: &ServiceAddr,
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

    let mut client = RcdClient::new_grpc_client(
        participant_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
        60,
    )
    .await;

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

#[cfg(test)]
#[tokio::main]

pub async fn main_read_updated_row_should_succed(
    db_name: &str,
    main_client_addr: &ServiceAddr,
) -> bool {
    use rcd_enum::database_type::DatabaseType;

    let mut client = RcdClient::new_grpc_client(
        main_client_addr.to_full_string_with_http(),
        String::from("tester"),
        String::from("123456"),
        60,
    )
    .await;

    let cmd = String::from("SELECT NAME FROM EMPLOYEE WHERE Id = 999");
    let read_result = client
        .execute_read_at_host(db_name, &cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
        .await;

    let results = read_result.unwrap();

    let row = results.rows.first().unwrap();

    let value = &row.values[1].value.clone();

    trace!("{value:?}");

    let expected_value = "TESTER".as_bytes().to_vec();

    trace!("{expected_value:?}");

    *value == expected_value
}
