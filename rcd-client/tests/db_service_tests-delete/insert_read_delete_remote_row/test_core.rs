use crate::test_common::multi::setup_io::setup_main_and_participant;
use crate::test_harness::CoreTestConfig;
use rcd_client::RcdClient;
use std::sync::mpsc;
use std::thread;

pub fn test_core(config: &CoreTestConfig) {

    setup_main_and_participant(config);


    let mc = config.main_client;
    let db = config.test_db_name.clone();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let res = main_try_delete(&db, &mut mc);
        tx.send(res).unwrap();
    })
    .join()
    .unwrap();

    let is_deleted = rx.try_recv().unwrap();

    assert!(is_deleted);


}

#[cfg(test)]
#[tokio::main]
async fn main_try_delete(db_name: &str, main_client: &RcdClient) -> bool {
    let delete_result = main_client
        .execute_cooperative_write_at_host(
            db_name,
            "DELETE FROM EMPLOYEE WHERE Id = 999",
            "participant",
            "Id = 999",
        )
        .await
        .unwrap();

    delete_result
}
