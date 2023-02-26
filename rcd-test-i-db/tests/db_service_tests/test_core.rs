use rcd_test_harness::test_common::multi::common_contract_setup::main_and_participant_setup;
use rcd_test_harness::CoreTestConfig;

pub fn test_core(config: CoreTestConfig) {
    go(config);
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let result = main_and_participant_setup(config).await;
    assert!(result);
}
