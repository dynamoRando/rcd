use rcd_enum::{
    deletes_from_host_behavior::DeletesFromHostBehavior,
    deletes_to_host_behavior::DeletesToHostBehavior,
};
use rcd_test_harness::{
    test_common::multi::common_contract_setup::main_and_participant_setup, CoreTestConfig,
};

use log::trace;
use rcd_enum::database_type::DatabaseType;

pub fn test_core(config: CoreTestConfig) {
    go(config)
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);

    let db_name = config.test_db_name.clone();

    let pc = config.participant_client.as_ref().unwrap().clone();
    let mut pc = rcd_test_harness::get_rcd_client(&pc).await;

    let mc = config.main_client.clone();
    let mut mc = rcd_test_harness::get_rcd_client(&mc).await;

    let new_behavior = DeletesFromHostBehavior::Ignore;
    pc.change_deletes_from_host_behavior(&db_name, "EMPLOYEE", new_behavior)
        .await
        .unwrap();

    let behavior = pc
        .get_deletes_from_host_behavior(&db_name, "EMPLOYEE")
        .await
        .unwrap()
        .behavior;
    let behavior = DeletesFromHostBehavior::from_u32(behavior);

    assert_eq!(behavior, new_behavior);

    let cmd = String::from("DELETE FROM EMPLOYEE WHERE Id = 999");
    let should_fail = mc
        .execute_cooperative_write_at_host(&db_name, &cmd, "participant", "Id = 999")
        .await
        .unwrap();

    assert!(!should_fail);

    // reset
    let new_behavior = DeletesFromHostBehavior::AllowRemoval;
    pc.change_deletes_from_host_behavior(&db_name, "EMPLOYEE", new_behavior)
        .await
        .unwrap();

    // test deletes at participant back to host

    let new_behavior = DeletesToHostBehavior::DoNothing;
    pc.change_deletes_to_host_behavior(&db_name, "EMPLOYEE", new_behavior)
        .await
        .unwrap();

    let behavior = pc
        .get_deletes_to_host_behavior(&db_name, "EMPLOYEE")
        .await
        .unwrap()
        .behavior;
    let behavior = DeletesToHostBehavior::from_u32(behavior);

    assert_eq!(behavior, new_behavior);

    // actually delete at particpant, so we dont have the row
    let statement = String::from("DELETE FROM EMPLOYEE WHERE ID = 999");
    pc.execute_write_at_participant(
        &db_name,
        &statement,
        DatabaseType::to_u32(DatabaseType::Sqlite),
        "ID = 999",
    )
    .await
    .unwrap();

    // at the host, we should have zero rows, but we get 1 row back
    let cmd = String::from("SELECT NAME FROM EMPLOYEE WHERE Id = 999");
    let read_result = mc
        .execute_read_at_host(&db_name, &cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
        .await;
    // we should expect to get zero rows back, but we don't

    trace!("{read_result:?}");

    assert!(!read_result.unwrap().rows.is_empty());

    // lets check a normal situation, reset and add a record
    let cmd = "INSERT INTO EMPLOYEE (ID, NAME) VALUES (999, 'TESTER')";
    mc.execute_cooperative_write_at_host(&db_name, &cmd, "participant", "")
        .await
        .unwrap();

    let new_behavior = DeletesToHostBehavior::SendNotification;
    pc.change_deletes_to_host_behavior(&db_name, "EMPLOYEE", new_behavior)
        .await
        .unwrap();

    // delete the record at the participant
    pc.execute_write_at_participant(
        &db_name,
        &statement,
        DatabaseType::to_u32(DatabaseType::Sqlite),
        "ID = 999",
    )
    .await
    .unwrap();

    // we should get 0 records at the host
    let cmd = String::from("SELECT NAME FROM EMPLOYEE WHERE Id = 999");
    let read_result = mc
        .execute_read_at_host(&db_name, &cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
        .await;
    // we should expect to get zero rows back

    trace!("{read_result:?}");

    assert!(read_result.unwrap().rows.is_empty());

    // reset with record
    let cmd = "INSERT INTO EMPLOYEE (ID, NAME) VALUES (999, 'TESTER')";
    mc.execute_cooperative_write_at_host(&db_name, &cmd, "participant", "")
        .await
        .unwrap();

    // check queue for review
    let new_behavior = DeletesFromHostBehavior::QueueForReview;
    pc.change_deletes_from_host_behavior(&db_name, "EMPLOYEE", new_behavior)
        .await
        .unwrap();

    // delete should fail

    let cmd = String::from("DELETE FROM EMPLOYEE WHERE Id = 999");
    let delete_statement = "DELETE FROM EMPLOYEE WHERE Id = 999";
    let mut has_statement = false;
    let mut statement_row_id = 0;

    let should_fail = mc
        .execute_cooperative_write_at_host(&db_name, &cmd, "participant", "Id = 999")
        .await
        .unwrap();

    assert!(!should_fail);

    // participant gets and approves pending delete
    let pending_deletes = pc
        .get_pending_actions_at_participant(&db_name, "EMPLOYEE", "DELETE")
        .await
        .unwrap();

    for statement in &pending_deletes.pending_statements {
        if statement.statement == delete_statement {
            has_statement = true;
            statement_row_id = statement.row_id;
        }
    }

    assert!(has_statement);

    let accept_delete_result = pc
        .accept_pending_action_at_participant(&db_name, "EMPLOYEE", statement_row_id)
        .await
        .unwrap();

    trace!("{accept_delete_result:?}");

    assert!(accept_delete_result.is_successful);

    // should not have rows
    let has_rows = mc
        .execute_read_at_host(
            &db_name,
            "SELECT * FROM EMPLOYEE",
            DatabaseType::to_u32(DatabaseType::Sqlite),
        )
        .await
        .unwrap()
        .rows
        .is_empty();

    assert!(has_rows);

    // reset with record
    let cmd = "INSERT INTO EMPLOYEE (ID, NAME) VALUES (999, 'ASDF')";
    mc.execute_cooperative_write_at_host(&db_name, &cmd, "participant", "")
        .await
        .unwrap();

    // check delete with log
    let new_behavior = DeletesFromHostBehavior::DeleteWithLog;
    pc.change_deletes_from_host_behavior(&db_name, "EMPLOYEE", new_behavior)
        .await
        .unwrap();

    let cmd = String::from("DELETE FROM EMPLOYEE WHERE Id = 999");
    mc.execute_cooperative_write_at_host(&db_name, &cmd, "participant", "Id = 999")
        .await
        .unwrap();

    // read the logs
    let cmd = "SELECT * FROM EMPLOYEE_COOP_DATA_LOG";
    let read_result = pc
        .execute_read_at_participant(&db_name, cmd, DatabaseType::to_u32(DatabaseType::Sqlite))
        .await
        .unwrap();

    trace!("{read_result:?}");

    let row = read_result.rows.first().unwrap();
    let value = &row.values[1].value.clone();

    trace!("{value:?}");

    let expected_value = "ASDF".as_bytes().to_vec();
    trace!("{expected_value:?}");

    assert!(*value == expected_value)
}
