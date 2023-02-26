use log::trace;
use rcd_enum::updates_from_host_behavior::UpdatesFromHostBehavior;
use rcd_test_harness::{
    test_common::multi::common_contract_setup::main_and_participant_setup, CoreTestConfig,
    RcdClientConfig,
};

pub fn test_core(config: CoreTestConfig) {
    go(config)
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);

    let db = config.test_db_name.clone();
    let mca = config.main_client.clone();
    let pca = config.participant_client.as_ref().unwrap().clone();

    let update_statement = "UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999";

    {
        let new_behavior = UpdatesFromHostBehavior::QueueForReviewAndLog;

        let db = db.clone();
        let pca = pca.clone();

        let update_at_participant_is_successful =
            participant_changes_update_behavior(&db, &pca, new_behavior).await;
        assert!(update_at_participant_is_successful);
    }

    {
        let db = db.clone();
        let mca = mca.clone();

        let can_read_rows = main_read_updated_row_should_fail(&db, &mca, update_statement).await;

        assert!(!can_read_rows);
    }

    {
        let db = db.clone();
        let pca = pca.clone();

        let has_and_accept_update =
            participant_get_and_approve_pending_update(&db, "EMPLOYEE", &pca, update_statement)
                .await;
        assert!(has_and_accept_update);
    }

    {
        let mca = mca.clone();
        let can_read_rows = main_read_updated_row_should_succed(&db, &mca).await;

        assert!(can_read_rows);
    }
}

async fn participant_changes_update_behavior(
    db_name: &str,
    participant_client_addr: &RcdClientConfig,
    behavior: UpdatesFromHostBehavior,
) -> bool {
    let mut client = rcd_test_harness::get_rcd_client(participant_client_addr).await;

    let change_update_behavior = client
        .change_updates_from_host_behavior(db_name, "EMPLOYEE", behavior)
        .await;

    change_update_behavior.unwrap()
}

async fn main_read_updated_row_should_fail(
    db_name: &str,
    main_client_addr: &RcdClientConfig,
    update_statement: &str,
) -> bool {
    use rcd_enum::database_type::DatabaseType;

    let mut client = rcd_test_harness::get_rcd_client(main_client_addr).await;

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

async fn participant_get_and_approve_pending_update(
    db_name: &str,
    table_name: &str,
    participant_client_addr: &RcdClientConfig,
    update_statement: &str,
) -> bool {
    let mut has_statement = false;
    let mut statement_row_id = 0;
    let mut client = rcd_test_harness::get_rcd_client(participant_client_addr).await;

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

async fn main_read_updated_row_should_succed(
    db_name: &str,
    main_client_addr: &RcdClientConfig,
) -> bool {
    use rcd_enum::database_type::DatabaseType;

    let mut client = rcd_test_harness::get_rcd_client(main_client_addr).await;

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
