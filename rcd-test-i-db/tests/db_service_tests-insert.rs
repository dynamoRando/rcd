use rcd_enum::database_type::DatabaseType;
use rcd_test_harness::test_common::multi::common_contract_setup::main_and_participant_setup;
use rcd_test_harness::CoreTestConfig;

use log::{debug, LevelFilter};
use rcd_test_harness::{
    init_log_to_screen,
    test_common::multi::runner::{RunnerConfig, TestRunner},
};

#[test]
fn grpc() {
    let test_name = "add_read_update_remote_grpc";
    init_log_to_screen(LevelFilter::Info);

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

    let test_name = "add_read_update_remote_http";

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some(String::from("contract")),
        use_internal_logging: false,
    };

    TestRunner::run_http_test_multi(config, test_core);
}

#[test]
fn proxy_grpc() {
    // rcd_test_harness::init_log_to_screen_fern(log::LevelFilter::Debug);

    let test_name = "add_read_update_remote_grpc-proxy";

    let config = RunnerConfig {
        test_name: test_name.to_string(),
        contract_desc: Some("".to_string()),
        use_internal_logging: false,
    };

    TestRunner::run_grpc_proxy_test_multi(config, test_core);
}

fn test_core(config: CoreTestConfig) {
    go(config);
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);
    let db_name = &config.test_db_name;
    let mut client = rcd_test_harness::get_rcd_client(&config.main_client).await;

    let update_result = client
        .execute_cooperative_write_at_host(
            db_name,
            "UPDATE EMPLOYEE SET NAME = 'Bob' WHERE Id = 999;",
            "participant",
            "Id = 999",
        )
        .await
        .unwrap();

    assert!(update_result);

    let new_data = client
        .execute_read_at_host(
            db_name,
            "SELECT Name FROM EMPLOYEE",
            DatabaseType::to_u32(DatabaseType::Sqlite),
        )
        .await
        .unwrap();

    debug!("{new_data:?}");

    let new_value = new_data
        .rows
        .first()
        .unwrap()
        .values
        .last()
        .unwrap()
        .value
        .clone();

    debug!("{new_value:?}");
    let expected_value = "Bob".as_bytes().to_vec();
    debug!("{expected_value:?}");

    assert!(new_value == expected_value);
}
