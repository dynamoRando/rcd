
use log::{debug, info};
use rcd_client::RcdClient;

use rcdx::rcd_service::get_service_from_config_file;
use std::sync::mpsc;
use std::{thread, time};

use crate::test_harness;

#[tokio::main]
async fn client(test_message: &str, http_addr: &str, http_port: u32) -> String {
    let mut client = RcdClient::new_http_client(
        "".to_string(),
        "".to_string(),
        0,
        http_addr.to_string(),
        http_port,
    );

    let result = client.is_online_reply(test_message.to_string()).await;

    result.reply_echo_message
}

#[test]
fn test() {
    test_harness::init_log_to_screen(log::LevelFilter::Info);
    let test_message: &str = "is_online_http";

    let port_num = test_harness::get_next_avail_port();

    let (tx, rx) = mpsc::channel();

    let root_dir = test_harness::get_test_temp_dir(test_message);
    debug!("{root_dir}");

    let mut service = get_service_from_config_file(None);

    let http_addr = service.rcd_settings.http_addr.clone();
    let http_port = port_num;

    let addr1 = http_addr.clone();
    let addr2 = http_addr;

    debug!("{:?}", &service);
    service.start_at_dir(&root_dir);

    info!("starting client service");

    thread::spawn(move || {
        service.start_http_at_addr_and_dir(addr1, http_port as u16, root_dir);
    });

    let time = time::Duration::from_secs(1);

    info!("sleeping for 1 seconds...");

    thread::sleep(time);

    thread::spawn(move || {
        let res = client(test_message, &addr2, http_port);
        tx.send(res).unwrap();
    })
    .join()
    .unwrap();

    let response = rx.try_recv().unwrap();

    debug!("test_is_online: got: {response} sent: {test_message}");

    assert_eq!(response, test_message);
}
