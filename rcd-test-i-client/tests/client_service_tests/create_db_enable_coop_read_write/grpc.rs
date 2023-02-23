use log::LevelFilter;
use rcd_test_harness::{
    init_log_to_screen,
    test_common::multi::runner::{RunnerConfig, TestRunner},
};

use super::test_core::test_core;

#[test]
fn test() {
    let test_name = "create_db_enable_coop_read_write_grpc";
    init_log_to_screen(LevelFilter::Info);

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: None,

    };

    TestRunner::run_grpc_test(config, test_core);
}
