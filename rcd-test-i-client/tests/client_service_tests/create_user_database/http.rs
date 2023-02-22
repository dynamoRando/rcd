use super::test_core::test_core;
use crate::test_common::multi::runner::{RunnerConfig, TestRunner};

#[test]
fn test() {
    let test_name = "create_user_database_positive_http";

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: None,
    };

    TestRunner::run_http_test(config, test_core);
}
