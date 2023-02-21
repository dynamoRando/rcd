use crate::common_http::{
    main_read_updated_row_should_fail, main_read_updated_row_should_succed,
    participant_changes_update_behavior, participant_get_and_approve_pending_update,
};
use crate::test_common::multi::http::http_main_and_participant_setup;
use crate::test_common::HttpTestSetup;
use crate::test_harness::http::shutdown_http_test;
use crate::test_harness::{self};
use rcd_enum::updates_from_host_behavior::UpdatesFromHostBehavior;
use std::sync::{mpsc, Arc};
use std::thread;

/*
# Test Description

*/

#[test]
fn test() {
    let test_name = "update_from_host_queue_http";
    let db = Arc::new(format!("{}{}", test_name, ".db"));
    let contract = Arc::new(String::from("insert read remote row"));
    let update_statement = "UPDATE EMPLOYEE SET NAME = 'TESTER' WHERE ID = 999";

    let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);
    let main_test_config = test_harness::http::start_service_with_http(&db, dirs.main_dir);

    let participant_test_config =
        test_harness::http::start_service_with_http(&db, dirs.participant_dir);

    let mca = Arc::new(main_test_config.http_address.clone());
    let pca = Arc::new(participant_test_config.http_address.clone());

    let test_config = HttpTestSetup {
        main_test_config: main_test_config,
        participant_test_config: participant_test_config,
        database_name: &db,
        contract_description: &contract,
    };

    test_harness::sleep_test();

    let common_setup_complete = http_main_and_participant_setup(test_config);
    assert!(common_setup_complete);

    {
        let new_behavior = UpdatesFromHostBehavior::QueueForReview;
        let db = db.clone();
        let pca = pca.clone();

        let (tx, rx) = mpsc::channel();

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

        // main - attempts to execute update but does not get requested value back (this is intentional)
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
        let db = db.clone();
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

    shutdown_http_test(&main_test_config, &participant_test_config);
}
