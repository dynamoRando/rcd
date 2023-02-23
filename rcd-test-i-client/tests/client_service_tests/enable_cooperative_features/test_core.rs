use log::debug;
use rcd_test_harness::{CoreTestConfig, RcdClientConfig};

pub fn test_core(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let db = config.test_db_name.clone();
    let response = client(&db, &mc);

    debug!("create_enable_cooperative_features: got: {response}");

    assert!(response);
}

#[tokio::main]
async fn client(db_name: &str, client: &RcdClientConfig) -> bool {
    let mut client = rcd_test_harness::get_rcd_client(client).await;
    return client.enable_cooperative_features(db_name).await.unwrap();
}
