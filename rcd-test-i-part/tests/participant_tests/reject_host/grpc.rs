use log::LevelFilter;
use rcd_test_harness::{
    init_log_to_screen,
    test_common::multi::runner::{RunnerConfig, TestRunner},
};

use super::test_core::test_core;

#[test]
fn test() {
    // init_log_to_screen(LevelFilter::Debug);

    let test_name = "reject_host_grpc";
    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some("".to_string()),
        use_internal_logging: false,
    };

    TestRunner::run_grpc_test_multi(config, test_core);
}

#[test]
fn test_proxy() {
    init_log_to_screen(LevelFilter::Debug);

    let test_name = "reject_host_grpc-proxy";

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some("".to_string()),
        use_internal_logging: false,
    };

    TestRunner::run_grpc_proxy_test_multi(config, test_core);
}
