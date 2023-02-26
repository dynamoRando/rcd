use rcd_test_harness::{
    test_common::multi::common_contract_setup::main_and_participant_setup, CoreTestConfig,
};

pub fn test_core(config: CoreTestConfig) {
    go(config)
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let pc = config.participant_client.as_ref().unwrap().clone();
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);

    let mut client = rcd_test_harness::get_rcd_client(&pc).await;

    let hosts = client.get_cooperative_hosts().await.unwrap();

    let mut has_host: bool = false;

    for host in &hosts.hosts {
        if host.host.as_ref().unwrap().host_name == "tester" {
            has_host = true;
        }
    }

    assert!(has_host);
}
