use log::LevelFilter;
use rcd_test_harness::{
    init_log_to_screen,
    test_common::multi::runner::{RunnerConfig, TestRunner},
};

use super::test_core::test_core;

#[test]
fn test() {
    let test_name = "sa_contract_grpc";
    init_log_to_screen(LevelFilter::Info);

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some(String::from("contract")),
        use_internal_logging: false,
    };

    TestRunner::run_grpc_test_multi(config, test_core);
}
