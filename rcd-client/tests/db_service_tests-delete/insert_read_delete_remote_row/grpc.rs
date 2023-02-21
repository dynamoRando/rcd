use super::test_core::test_core;
use crate::test_harness::CoreTestConfig;
use crate::test_harness::{self};
use std::thread;

#[test]
fn test() {
    let test_name = "add_read_delete_remote_gprc";
    let db = format!("{}{}", test_name, ".db");
    let contract = String::from("add read delete remote row");

    let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);
    let main_test_config =
        test_harness::grpc::start_service_with_grpc(&db, dirs.main_dir);
    let participant_test_config =
        test_harness::grpc::start_service_with_grpc(&db, dirs.participant_dir);

    test_harness::sleep_test();

    {
        let mtc = main_test_config.clone();
        let ptc = participant_test_config.clone();

        thread::spawn(move || {
            let mc = test_harness::get_grpc_rcd_client(mtc.client_address.clone());
            let pc = test_harness::get_grpc_rcd_client(ptc.client_address.clone());
            let pda = ptc.database_address.clone();

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

    test_harness::grpc::shutdown_grpc_test(&main_test_config, &participant_test_config);
}
