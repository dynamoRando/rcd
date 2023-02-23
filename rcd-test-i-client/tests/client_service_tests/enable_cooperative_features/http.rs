use rcd_test_harness::test_common::multi::runner::{RunnerConfig, TestRunner};

use super::test_core::test_core;

#[test]
fn test() {
    let test_name = "enable_coooperative_features_http";

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: None,
    };

    TestRunner::run_http_test(config, test_core);
}
