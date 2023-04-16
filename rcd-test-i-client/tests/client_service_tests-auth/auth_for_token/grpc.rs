use rcd_test_harness::{
    init_log_to_screen,
    test_common::multi::runner::{RunnerConfig, TestRunner},
};

use super::test_core::test_core;

#[test]
fn test() {
    init_log_to_screen(log::LevelFilter::Debug);

    let test_name = "auth_token_grpc";
    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: None,
        use_internal_logging: false,
    };

    TestRunner::run_grpc_test(config, test_core);
}

#[test]
fn proxy() {
    // rcd_test_harness::init_log_to_screen_fern(tracing::LevelFilter::Debug);

    let test_name = "auth_token_grpc-proxy";

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some("".to_string()),
        use_internal_logging: false,
    };

    TestRunner::run_grpc_proxy_test(config, test_core);
}
