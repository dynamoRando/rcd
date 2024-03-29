use rcd_test_harness::test_common::multi::runner::{RunnerConfig, TestRunner};
use super::test_core::test_core;

#[test]
fn test() {
    rcd_test_harness::init_log_to_screen(log::LevelFilter::Debug);

    let test_name = "updates_from_host_queue_with_log_http";
    let contract = String::from("insert read remote row");

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some(contract),
        use_internal_logging: false,
    };

    TestRunner::run_http_test_multi(config, test_core);
}
