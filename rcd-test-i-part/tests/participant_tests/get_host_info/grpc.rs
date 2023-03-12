use super::test_core::test_core;
use rcd_test_harness::test_common::multi::runner::{RunnerConfig, TestRunner};
use simple_logger::SimpleLogger;

#[test]
fn test() {
    let test_name = "get_host_info_grpc";

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: None,
        use_internal_logging: false,
    };

    TestRunner::run_grpc_test(config, test_core);
}


#[test]
fn test_proxy() {
    use ignore_result::Ignore;

    SimpleLogger::new()
    .with_level(log::LevelFilter::Debug)
    .init()
    .ignore();


    let test_name = "get_host_info_grpc-proxy";

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: None,
        use_internal_logging: false,
    };

    TestRunner::run_grpc_proxy_test(config, test_core);
}
