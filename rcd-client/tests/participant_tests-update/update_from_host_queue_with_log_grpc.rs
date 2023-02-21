
use crate::common_grpc::{
    main_read_updated_row_should_fail, main_read_updated_row_should_succed,
    participant_changes_update_behavior, participant_get_and_approve_pending_update,
};
use crate::test_common::multi::grpc::grpc_main_and_participant_setup;
use crate::test_common::GrpcTestSetup;
use crate::test_harness::{self};

use rcd_enum::updates_from_host_behavior::UpdatesFromHostBehavior;
use std::sync::{mpsc, Arc};
use std::thread;

#[test]
fn test() {
    let test_name = "updates_from_host_queue_with_log_grpc";
    let db = Arc::new(format!("{}{}", test_name, ".db"));
    let contract = Arc::new(String::from("insert read remote row"));

    let update_statement = "UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999";

    let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);

    let main_test_config = test_harness::grpc::start_service_with_grpc(&db, dirs.main_dir);

    let participant_test_config =
        test_harness::grpc::start_service_with_grpc(&db, dirs.participant_dir);

    let main_client_addr = Arc::new(main_test_config.client_address.clone());
    let participant_client_addr = Arc::new(participant_test_config.client_address.clone());

    let mca = main_client_addr.clone();
    let pca = participant_client_addr.clone();

    test_harness::sleep_test();

    let test_config = GrpcTestSetup {
        main_test_config: main_test_config,
        participant_test_config: participant_test_config,
        database_name: &db,
        contract_description: &&contract,
    };

    let common_setup_complete = grpc_main_and_participant_setup(test_config);
    assert!(common_setup_complete);

    {
        let new_behavior = UpdatesFromHostBehavior::QueueForReviewAndLog;
        let (tx, rx) = mpsc::channel();
        let db = db.clone();
        let pca = pca.clone();

        // participant - changes behavior to log updates but not execute them
        thread::spawn(move || {
            let res = participant_changes_update_behavior(&db, &pca, new_behavior);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let update_at_participant_is_successful = rx.try_recv().unwrap();

        assert!(update_at_participant_is_successful);
    }

    {
        let (tx, rx) = mpsc::channel();
        let db = db.clone();
        let mca = mca.clone();

        thread::spawn(move || {
            let res = main_read_updated_row_should_fail(&db, &mca, update_statement);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let can_read_rows = rx.try_recv().unwrap();
        assert!(!can_read_rows);
    }

    {
        let (tx, rx) = mpsc::channel();
        let db = db.clone();
        let pca = pca.clone();

        // participant - gets pending updates and later accepts the update
        thread::spawn(move || {
            let res =
                participant_get_and_approve_pending_update(&db, "EMPLOYEE", &pca, update_statement);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let has_and_accept_update = rx.try_recv().unwrap();
        assert!(has_and_accept_update);
    }

    {
        let (tx, rx) = mpsc::channel();
        let mca = mca.clone();

        // main - checks the update value again and should match
        thread::spawn(move || {
            let res = main_read_updated_row_should_succed(&db, &mca);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let can_read_rows = rx.try_recv().unwrap();
        assert!(can_read_rows);
    }

    test_harness::grpc::shutdown_grpc_test(&main_test_config, &participant_test_config);
}
