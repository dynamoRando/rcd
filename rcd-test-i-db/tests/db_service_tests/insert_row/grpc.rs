use super::test_core::test_core;
use crate::test_common::multi::runner::{RunnerConfig, TestRunner};
use crate::test_harness::{self};

#[test]
fn test() {
    test_harness::init_log_to_screen(log::LevelFilter::Info);

    /*
        We will need to kick off two services, the host and the participant
        and we will need to also kick off two clients, one for each
    */

    let test_name = "insert_remote_row_grpc";
    let custom_contract_description = String::from("insert remote row");
    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some(custom_contract_description),
    };

    TestRunner::run_grpc_test(config, test_core);
}
