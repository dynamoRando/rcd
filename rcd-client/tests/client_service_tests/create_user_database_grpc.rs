use log::{info, debug};
use rcd_client::RcdClient;
use rcdx::rcd_service::get_service_from_config_file;
use std::sync::mpsc;
use std::thread;

use crate::test_harness;

#[test]
fn test() {
    test_harness::init_log_to_screen(log::LevelFilter::Info);

    let test_name = "create_user_database_positive_grpc";
    let test_db_name = format!("{}{}", test_name, ".db");
    let (tx, rx) = mpsc::channel();
    let port_num = test_harness::TEST_SETTINGS
        .lock()
        .unwrap()
        .get_next_avail_port();

    let root_dir = test_harness::get_test_temp_dir(test_name);
    debug!("{root_dir}");

    let mut service = get_service_from_config_file(None);
    let client_address_port = format!("{}{}", String::from("[::1]:"), port_num);
    let target_client_address_port = client_address_port.clone();
    debug!("{:?}", &service);

    service.start_at_dir(&root_dir);

    info!("starting client at {}", &client_address_port);
    info!("starting client service");

    thread::spawn(move || {
        let _service = service.start_grpc_client_service_at_addr(client_address_port, root_dir);
    });

    test_harness::sleep_test();

    thread::spawn(move || {
        let res = client(&test_db_name, &target_client_address_port);
        tx.send(res).unwrap();
    })
    .join()
    .unwrap();

    let response = rx.try_recv().unwrap();

    debug!("create_user_database: got: {response}");

    assert!(response);

    test_harness::release_port(port_num);
}

#[cfg(test)]
#[tokio::main]
async fn client(db_name: &str, addr_port: &str) -> bool {
    let addr_port = format!("{}{}", String::from("http://"), addr_port);
    info!(
        "client_create_user_database attempting to connect {}",
        addr_port
    );

    let mut client = RcdClient::new_grpc_client(
        addr_port,
        String::from("tester"),
        String::from("123456"),
        60,
    )
    .await;
    return client.create_user_database(db_name).await.unwrap();
}

#[test]
fn negative_test() {
    let test_name = "create_user_database_negative_grpc";
    let test_db_name = format!("{}{}", test_name, ".db");

    let (tx, rx) = mpsc::channel();
    let port_num = test_harness::TEST_SETTINGS
        .lock()
        .unwrap()
        .get_next_avail_port();

    let root_dir = test_harness::get_test_temp_dir(test_name);
    debug!("{root_dir}");

    let mut service = get_service_from_config_file(None);
    let client_address_port = format!("{}{}", String::from("[::1]:"), port_num);
    let target_client_address_port = client_address_port.clone();
    debug!("{:?}", &service);

    service.start_at_dir(&root_dir);

    info!("starting client at {}", &client_address_port);
    info!("starting client service");

    thread::spawn(move || {
        let _service = service.start_grpc_client_service_at_addr(client_address_port, root_dir);
    });

    test_harness::sleep_test();

    thread::spawn(move || {
        let res = negative_client(&test_db_name, &target_client_address_port);
        tx.send(res).unwrap();
    })
    .join()
    .unwrap();

    let response = rx.try_recv().unwrap();

    debug!("create_user_database: got: {response}");

    assert!(!response);
}

#[cfg(test)]
#[tokio::main]
async fn negative_client(db_name: &str, addr_port: &str) -> bool {
    let addr_port = format!("{}{}", String::from("http://"), addr_port);
    info!(
        "client_create_user_database attempting to connect {}",
        addr_port
    );

    let mut client = RcdClient::new_grpc_client(
        addr_port,
        String::from("wrong_user"),
        String::from("123456"),
        60,
    )
    .await;

    return client.create_user_database(db_name).await.unwrap();
}
