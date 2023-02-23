
use crate::test_common::multi::grpc::grpc_main_and_participant_setup;
use crate::test_common::GrpcTestSetup;
use crate::test_harness::{self, ServiceAddr};
use log::{info, trace};
use rcd_client::RcdClient;
use rcd_enum::updates_from_host_behavior::UpdatesFromHostBehavior;
use std::sync::{mpsc, Arc};
use std::thread;

/*
# Test Description

*/

#[test]
fn test() {
    let test_name = "update_from_host_queue_gprc";
    let test_db_name = Arc::new(format!("{}{}", test_name, ".db"));
    let custom_contract_description = Arc::new(String::from("insert read remote row"));
    let update_statement = "UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999";

    let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);

    let main_test_config =
        test_harness::grpc::start_service_with_grpc(&test_db_name, dirs.main_dir);
    let participant_test_config =
        test_harness::grpc::start_service_with_grpc(&test_db_name, dirs.participant_dir);

    let main_client_addr = Arc::new(main_test_config.client_address.clone());
    let participant_client_addr = Arc::new(participant_test_config.client_address.clone());

    let test_config = GrpcTestSetup {
        main_test_config: main_test_config,
        participant_test_config: participant_test_config,
        database_name: &test_db_name,
        contract_description: &custom_contract_description,
    };

    test_harness::sleep_test();

    let common_setup_complete = grpc_main_and_participant_setup(test_config);
    assert!(common_setup_complete);

    let new_behavior = UpdatesFromHostBehavior::QueueForReview;

    {
        let (tx, rx) = mpsc::channel();
        let test_db_name = test_db_name.clone();
        let participant_client_addr = participant_client_addr.clone();

        thread::spawn(move || {
            let res = participant_changes_update_behavior(
                &test_db_name,
                &participant_client_addr,
                new_behavior,
            );
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let update_at_participant_is_successful = rx.try_recv().unwrap();

        assert!(update_at_participant_is_successful);
    }

    {
        let (tx, rx) = mpsc::channel();
        let test_db_name = test_db_name.clone();
        let main_client_addr = main_client_addr.clone();

        // main - attempts to execute update but does not get requested value back (this is intentional)
        thread::spawn(move || {
            let res = main_read_updated_row_should_fail(
                &test_db_name,
                &main_client_addr,
                update_statement,
            );
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let can_read_rows = rx.try_recv().unwrap();
        assert!(!can_read_rows);
    }

    {
        let (tx, rx) = mpsc::channel();
        let test_db_name = test_db_name.clone();

        // participant - gets pending updates and later accepts the update
        thread::spawn(move || {
            let res = participant_get_and_approve_pending_update(
                &test_db_name,
                "EMPLOYEE",
                &participant_client_addr,
                update_statement,
            );
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let has_and_accept_update = rx.try_recv().unwrap();
        assert!(has_and_accept_update);
    }

    {
        let (tx, rx) = mpsc::channel();
        let test_db_name = test_db_name.clone();
        let main_client_addr = main_client_addr.clone();

        // main - checks the update value again and should match
        thread::spawn(move || {
            let res = main_read_updated_row_should_succed(&test_db_name, &main_client_addr);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let can_read_rows = rx.try_recv().unwrap();
        assert!(can_read_rows);
    }

    test_harness::grpc::shutdown_grpc_test(&main_test_config, &participant_test_config);
}

#[cfg(test)]
#[tokio::main]
async fn participant_changes_update_behavior(
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
async fn participant_get_and_approve_pending_update(
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

async fn main_read_updated_row_should_fail(
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

async fn main_read_updated_row_should_succed(
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
