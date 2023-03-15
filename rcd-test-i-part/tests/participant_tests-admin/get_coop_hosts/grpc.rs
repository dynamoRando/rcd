use super::test_core::test_core;
use rcd_test_harness::test_common::multi::runner::{RunnerConfig, TestRunner};

#[test]
fn test() {
    rcd_test_harness::init_log_to_screen(log::LevelFilter::Debug);

    let test_name = "get_coop_hosts_gprc";
    let contract = String::from("insert read remote row");
    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some(contract),
        use_internal_logging: false,
    };

    TestRunner::run_grpc_test_multi(config, test_core);
}


#[test]
fn proxy() {
    rcd_test_harness::init_log_to_screen(log::LevelFilter::Debug);

    let test_name = "get_coop_hosts_grpc-proxy";

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some("".to_string()),
        use_internal_logging: false,
    };

    TestRunner::run_grpc_proxy_test_multi(config, test_core);
}
