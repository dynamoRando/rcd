use tracing::debug;
use rcd_test_harness::CoreTestConfig;
use rcd_test_harness::RcdClientConfig;

pub fn test_core(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let db = config.test_db_name;

    let response = client(&db, &mc);
    debug!("create_user_database: got: {response}");

    assert!(response);
}

#[tokio::main]
async fn client(db_name: &str, client: &RcdClientConfig) -> bool {
    let mut client = rcd_test_harness::get_rcd_client(client).await;
    return client.create_user_database(db_name).await.unwrap();
}
