use log::debug;
use rcd_client::RcdClient;
use rcd_test_harness::CoreTestConfig;

use std::sync::{mpsc, Arc};
use std::thread;

pub fn test_core(config: CoreTestConfig) {
    let mc = Arc::new(config.main_client.clone());
    let db = Arc::new(config.test_db_name.clone());

    {
        let (tx, rx) = mpsc::channel();
        let db = db.clone();
        thread::spawn(move || {
            let res = client(&db, &mc);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        debug!("create_user_database: got: {response}");

        assert!(response);
    }
}

#[cfg(test)]
#[tokio::main]
async fn client(db_name: &str, client: &RcdClient) -> bool {
    let mut client = (*client).clone();
    return client.create_user_database(db_name).await.unwrap();
}
