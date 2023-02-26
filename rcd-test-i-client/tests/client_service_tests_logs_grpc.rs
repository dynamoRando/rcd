use rcd_test_harness::{
    test_common::multi::runner::{RunnerConfig, TestRunner},
};
use crate::client_service_tests_logs_core::test_core;
mod client_service_tests_logs_core;

#[test]
fn grpc() {

    let test_name = "get_logs_grpc";
    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: None,
        use_internal_logging: true,
    };

    TestRunner::run_grpc_test(config, test_core);
}
