use crate::test_harness::{self, CoreTestConfig};
use std::thread;

use super::test_core::test_core;

#[test]
fn test() {
    /*
        We will need to kick off two services, the host and the participant
        and we will need to also kick off two clients, one for each
    */

    let test_name = "insert_remote_row_http";
    let db = format!("{}{}", test_name, ".db");
    let contract = String::from("insert remote row");

    let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);
    let main_test_config = test_harness::http::start_service_with_http(&db, dirs.main_dir);
    let participant_test_config =
        test_harness::http::start_service_with_http(&db, dirs.participant_dir);

    test_harness::sleep_test();

    {
        let mtc = main_test_config.clone();
        let ptc = participant_test_config.clone();

        thread::spawn(move || {
            let mc = test_harness::get_http_rcd_client(mtc.http_address.clone());
            let pc =
                test_harness::get_http_rcd_client(ptc.http_address.clone());
            let pda = ptc.http_address.clone();

            let config = CoreTestConfig {
                main_client: mc,
                participant_client: pc,
                test_db_name: db,
                contract_desc: contract,
                participant_db_addr: pda,
            };

            test_core(config);
        })
        .join()
        .unwrap();
    }

    test_harness::http::shutdown_http_test(&main_test_config, &participant_test_config);
}
