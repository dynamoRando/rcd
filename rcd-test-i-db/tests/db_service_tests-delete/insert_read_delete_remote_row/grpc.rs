use super::test_core::test_core;
use crate::test_common::multi::runner::{RunnerConfig, TestRunner};

#[test]
fn test() {
    let test_name = "add_read_delete_remote_gprc";
    let contract = String::from("add read delete remote row");
    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some(contract),
    };

    TestRunner::run_grpc_test(config, test_core);
}
