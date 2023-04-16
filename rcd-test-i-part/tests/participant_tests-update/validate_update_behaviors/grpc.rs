use super::test_core::test_core;
use rcd_test_harness::test_common::multi::runner::{RunnerConfig, TestRunner};

#[test]
fn test() {
    // rcd_test_harness::init_log_to_screen(tracing::LevelFilter::Debug);

    let test_name = "get_update_from_part_gprc";
    let contract = String::from("");
    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some(contract),
        use_internal_logging: false,
    };

    TestRunner::run_grpc_test_multi(config, test_core);
}

#[test]
fn proxy() {
    // rcd_test_harness::init_log_to_screen_fern(tracing::LevelFilter::Debug);

    let test_name = "get_update_from_part_gprc-proxy";

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some("".to_string()),
        use_internal_logging: false,
    };

    TestRunner::run_grpc_proxy_test_multi(config, test_core);
}
