use super::test_core::test_core;
use crate::test_harness::CoreTestConfig;
use crate::test_harness::{self};
use std::{sync::Arc, thread};

#[test]
fn test() {
    let test_name = "add_read_delete_remote_http";
    let db = format!("{}{}", test_name, ".db");
    let contract = String::from("add read delete remote row");
    let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);
    let main_test_config = test_harness::http::start_service_with_http(&db, dirs.main_dir);
    let participant_test_config =
        test_harness::http::start_service_with_http(&db, dirs.participant_dir);

    thread::spawn(move || {
        let mc = test_harness::get_http_rcd_client(main_test_config.http_address.clone());
        let pc = test_harness::get_http_rcd_client(participant_test_config.http_address.clone());
        let pda = participant_test_config.http_address.clone();

        let config = CoreTestConfig {
            main_client: &mc,
            participant_client: &pc,
            test_db_name: &db,
            contract_desc: &contract,
            participant_db_addr: &pda,
        };

        test_core(&config);
    })
    .join()
    .unwrap();
}
