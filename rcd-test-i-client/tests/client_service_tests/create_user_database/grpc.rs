use super::test_core::test_core;
use rcd_test_harness::test_common::multi::runner::{RunnerConfig, TestRunner};

#[test]
fn test() {
    let test_name = "create_user_database_positive_grpc";

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: None,
        use_internal_logging: false,
    };

    TestRunner::run_grpc_test(config, test_core);
}
