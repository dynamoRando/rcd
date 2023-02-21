use crate::test_harness::{self, CoreTestConfig};
use std::thread;

use super::test_core::{test_core};

#[test]
fn test() {
    test_harness::init_log_to_screen(log::LevelFilter::Info);

    /*
        We will need to kick off two services, the host and the participant
        and we will need to also kick off two clients, one for each
    */

    let test_name = "insert_remote_row_grpc";
    let test_db_name = format!("{}{}", test_name, ".db");
    let custom_contract_description = String::from("insert remote row");

    let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);

    let main_test_config =
        test_harness::grpc::start_service_with_grpc(&test_db_name, dirs.main_dir);
    let participant_test_config =
        test_harness::grpc::start_service_with_grpc(&test_db_name, dirs.participant_dir);

    test_harness::sleep_test();

    thread::spawn(move || {
        let mc = test_harness::get_grpc_rcd_client(main_test_config.client_address.clone());
        let pc = test_harness::get_grpc_rcd_client(participant_test_config.client_address.clone());
        let pda = participant_test_config.database_address.clone();

        let config = CoreTestConfig {
            main_client: &mut mc,
            participant_client: &mut pc,
            test_db_name: &test_db_name,
            contract_desc: &custom_contract_description,
            participant_db_addr: &pda,
        };

        test_core(&config);
    })
    .join()
    .unwrap();

    test_harness::grpc::shutdown_grpc_test(&main_test_config, &participant_test_config);
}
