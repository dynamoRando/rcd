use rcd_test_harness::{CoreTestConfig, RcdClientConfig};

pub fn test_core(config: CoreTestConfig) {
    let mc = config.main_client;
    let response = client(&mc);
    assert!(response);
}

#[tokio::main]
async fn client(config: &RcdClientConfig) -> bool {
    let mut client = rcd_test_harness::get_rcd_client(config).await;
    client.is_online().await
}
