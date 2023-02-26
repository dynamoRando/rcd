use log::debug;
use rcd_test_harness::CoreTestConfig;
use rcd_test_harness::RcdClientConfig;

pub fn test_core(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let db = config.test_db_name.clone();

    let response = client(&db, &mc);
    debug!("create_user_database: got: {response}");

    assert!(response);
}

#[cfg(test)]
#[tokio::main]
async fn client(db_name: &str, config: &RcdClientConfig) -> bool {
    let mut client = rcd_test_harness::get_rcd_client(config).await;

    client.create_user_database(db_name).await.unwrap();
    client.enable_cooperative_features(db_name).await.unwrap();
    client.generate_host_info("main").await.unwrap();

    let host_info = client.get_host_info().await.unwrap();

    host_info.host_info.unwrap().host_name == "main"
}
