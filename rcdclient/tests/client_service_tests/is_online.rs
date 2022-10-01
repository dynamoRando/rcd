use log::info;
use rcdproto::rcdp::sql_client_client::SqlClientClient;
use rcdproto::rcdp::TestRequest;
use rcdx::get_service_from_config_file;
use std::sync::mpsc;
use std::{thread, time};

#[tokio::main]
async fn client(test_message: &str, addr_port: &str) -> String {
    let addr_port = format!("{}{}", String::from("http://"), addr_port);
    info!("client_if_online attempting to connect {}", addr_port);

    let endpoint = tonic::transport::Channel::builder(addr_port.parse().unwrap());
    let channel = endpoint.connect().await.unwrap();

    let mut client = SqlClientClient::new(channel);

    info!("created channel and client");

    let request = tonic::Request::new(TestRequest {
        request_echo_message: test_message.to_string(),
        request_time_utc: String::from(""),
        request_origin_url: String::from(""),
        request_origin_ip4: String::from(""),
        request_origin_ip6: String::from(""),
        request_port_number: 1234,
    });

    info!("sending request");

    let response = client.is_online(request).await.unwrap().into_inner();
    println!("RESPONSE={:?}", response);
    info!("response back");

    return String::from(&response.reply_echo_message);
}

#[test]
fn test() {
    let test_message: &str = "is_online";
    let (tx, rx) = mpsc::channel();

    let root_dir = super::test_harness::get_test_temp_dir(test_message);
    println!("{}", root_dir);

    let mut service = get_service_from_config_file();
    let client_address_port = service.rcd_settings.client_service_addr_port.clone();
    println!("{:?}", &service);
    service.start_at_dir(&root_dir);

    info!("starting client service");

    thread::spawn(move || {
        let _service = service.start_client_service_alt();
    });

    let time = time::Duration::from_secs(5);

    info!("sleeping for 5 seconds...");

    thread::sleep(time);

    thread::spawn(move || {
        let res = client(test_message, &client_address_port);
        tx.send(res).unwrap();
    })
    .join()
    .unwrap();

    let response = rx.try_recv().unwrap();

    println!("test_is_online: got: {} sent: {}", response, test_message);

    assert_eq!(response, test_message);
}
