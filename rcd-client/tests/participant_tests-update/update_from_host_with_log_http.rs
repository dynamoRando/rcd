use crate::common_http::{
    get_data_hash_for_changed_row_at_host, get_data_hash_for_changed_row_at_participant,
    get_data_logs_at_participant, get_row_id_at_participant, main_overwrite_and_read_updated_row_should_succeed,
    participant_changes_update_behavior,
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

## Purpose:
This test checks to see if when an UPDATE statement is sent from the host if the participant's settings to
`UpdatesFromHostBehavior::OverwriteWithLog` that rcd copies the row it's about to overwrite to a `_COOP_DATA_LOG`
table before actually executing the overwrite.

## Feature Background
We want to make sure that participants have full authority over their data. This means that if they want to see
a history of changes that are being made to their data, they can do so. In this test, a value is initially set by a host
and later is UPDATEd.

We expect that the UPDATE from the host should succeed, but that we should also have a record of the changed row
in the `EMPLOYEE_COOP_DATA_LOG` table in the partial database that the participant can review.

## Test Steps
- Start an rcd instance for a main (host) and a participant
- Host:
    - Generate a db and tables and a contract to send to particpant
- Participant:
    - Accept contract
- Host:
    - Send one row to participant to be inserted and test to make sure can read from participant
- Participant:
    - Change UpdatesFromHostBehavior to Overwrite With Log
    - Update the newly added row from the previous step
    - Get the row id for the newly updated row
    - Get the data hash for the newly updated row
- Host:
    - Attempt to read previously inserted row, and should returning matching data.
    - Get the data hash saved at the host
    - Check to make sure the hashes match (ensure that the update is correct)
- Participant:
    - Send a query to `SELECT * FROM EMPLOYEE_COOP_DATA_LOG` locally at the participant. There should only
    be one row that is returned.

### Expected Results:
The row returned at the participant should match the previously inserted value instead of the UPDATEd value.

*/

#[test]
fn test() {
    let test_name = "update_from_host_log_http";
    let db = Arc::new(format!("{}{}", test_name, ".db"));
    let contract = Arc::new(String::from("insert read remote row"));

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
        let new_behavior = UpdatesFromHostBehavior::OverwriteWithLog;
        let (tx, rx) = mpsc::channel();
        let db = db.clone();
        let pca = pca.clone();

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
            let res = main_overwrite_and_read_updated_row_should_succeed(&db, &mca);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let can_read_rows = rx.try_recv().unwrap();
        assert!(can_read_rows);
    }

    let participant_row_id: u32;

    {
        let (tx, rx) = mpsc::channel();
        let db = db.clone();
        let pca = pca.clone();

        thread::spawn(move || {
            let res = get_row_id_at_participant(&db, &pca);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let participant_row_id = rx.try_recv().unwrap();
    }

    let participant_data_hash: u64;

    {
        let (tx, rx) = mpsc::channel();
        let db = db.clone();
        let pca = pca.clone();

        thread::spawn(move || {
            let res = get_data_hash_for_changed_row_at_participant(&db, &pca, participant_row_id);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let participant_data_hash = rx.try_recv().unwrap();
    }

    let host_data_hash: u64;

    {
        let (tx, rx) = mpsc::channel();
        let db = db.clone();
        let mca = mca.clone();

        thread::spawn(move || {
            let res = get_data_hash_for_changed_row_at_host(&db, &mca, participant_row_id);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        host_data_hash = rx.try_recv().unwrap();
    }

    assert_eq!(participant_data_hash, host_data_hash);

    {
        let (tx, rx) = mpsc::channel();
        let db = db.clone();
        let pca = pca.clone();

        thread::spawn(move || {
            let res = get_data_logs_at_participant(&db, &pca);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let p_read_data_log_is_correct = rx.try_recv().unwrap();

        assert!(p_read_data_log_is_correct);
    }

    shutdown_http_test(&main_test_config, &participant_test_config);
}


