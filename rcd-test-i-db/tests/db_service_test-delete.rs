use rcd_test_harness::test_common::multi::common_contract_setup::main_and_participant_setup;
use rcd_test_harness::CoreTestConfig;

use log::LevelFilter;
use rcd_test_harness::{
    init_log_to_screen,
    test_common::multi::runner::{RunnerConfig, TestRunner},
};

#[test]
fn grpc() {
    let test_name = "add_read_delete_remote_gprc";
    init_log_to_screen(LevelFilter::Debug);

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some(String::from("contract")),
        use_internal_logging: false,
    };

    TestRunner::run_grpc_test_multi(config, test_core);
}

#[test]
fn http() {
    rcd_test_harness::init_log_to_screen(log::LevelFilter::Debug);

    let test_name = "add_read_delete_remote_http";

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some(String::from("contract")),
        use_internal_logging: false,
    };

    TestRunner::run_http_test_multi(config, test_core);
}

fn test_core(config: CoreTestConfig) {
    go(config);
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);

    let mut client = rcd_test_harness::get_rcd_client(&config.main_client).await;
    let can_delete = client
        .execute_cooperative_write_at_host(
            &config.test_db_name,
            "DELETE FROM EMPLOYEE WHERE Id = 999",
            "participant",
            "Id = 999",
        )
        .await
        .unwrap();

    assert!(can_delete);
}
