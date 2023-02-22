use crate::test_common::multi::runner::{RunnerConfig, TestRunner};
use super::test_core::test_core;

#[test]
fn test() {
    /*
        We will need to kick off two services, the host and the participant
        and we will need to also kick off two clients, one for each
    */

    let test_name = "insert_remote_row_http";
    let contract = String::from("insert remote row");
    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some(contract),
    };

    TestRunner::run_http_test_multi(config, test_core);
}
