use rcd_test_harness::{
    test_common::multi::common_contract_setup::main_and_participant_setup, CoreTestConfig,
    RcdClientConfig,
};

pub fn test_core(config: CoreTestConfig) {
    go(config)
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);

    let db = config.test_db_name.clone();
    let pc = config.participant_client.as_ref().unwrap().clone();
    let mc = config.main_client.clone();
    let reject = participant_rejects_host(&pc).await;
    assert!(reject);

    let should_fail = main_read_should_fail(&db, &mc).await;

    assert!(!should_fail);
}

async fn participant_rejects_host(config: &RcdClientConfig) -> bool {
    use rcd_enum::host_status::HostStatus;

    let mut client = rcd_test_harness::get_rcd_client(config).await;

    let host_status = HostStatus::Deny;

    let reject_host_result = client
        .change_host_status_by_name("tester", HostStatus::to_u32(host_status))
        .await;

    reject_host_result.unwrap()
}

async fn main_read_should_fail(db_name: &str, config: &RcdClientConfig) -> bool {
    let mut client = rcd_test_harness::get_rcd_client(config).await;

    let attempt = client
        .try_auth_at_participant("participant", "", db_name)
        .await;

    attempt
}
