use rcd_test_harness::CoreTestConfig;
use log::debug;
use rcd_client::RcdClient;
use std::sync::{mpsc, Arc};
use std::thread;

pub fn test_core(config: CoreTestConfig) {
    let mc = Arc::new(config.main_client.clone());
    
    {
        let (tx, rx) = mpsc::channel();
        let db = config.test_db_name.clone();

        thread::spawn(move || {
            let res = client(&db, &mc);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        debug!("create_enable_cooperative_features: got: {response}");

        assert!(response);
    }
}


#[tokio::main]
async fn client(db_name: &str, client: &RcdClient) -> bool {
    let mut client = (*client).clone();
    return client.enable_cooperative_features(db_name).await.unwrap();
}
