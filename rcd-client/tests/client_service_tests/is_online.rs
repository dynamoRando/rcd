pub mod grpc {

    use log::info;
    use rcdproto::rcdp::sql_client_client::SqlClientClient;
    use rcdproto::rcdp::TestRequest;
    use rcdx::rcd_service::get_service_from_config_file;
    use std::sync::mpsc;
    use std::{thread, time};

    use crate::test_harness;

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
        let test_message: &str = "is_online_grpc";
        let (tx, rx) = mpsc::channel();

        let root_dir = test_harness::get_test_temp_dir(test_message);
        println!("{}", root_dir);

        let mut service = get_service_from_config_file();
        let client_address_port = service.rcd_settings.grpc_client_service_addr_port.clone();
        println!("{:?}", &service);
        service.start_at_dir(&root_dir);

        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_grpc_client_service_alt();
        });

        let time = time::Duration::from_secs(1);

        info!("sleeping for 1 seconds...");

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
}

pub mod http {

    use log::info;
    use rcd_client::RcdClient;

    use rcdx::rcd_service::get_service_from_config_file;
    use std::sync::mpsc;
    use std::{thread, time};

    use crate::test_harness;

    #[tokio::main]
    async fn client(test_message: &str, http_addr: &str, http_port: u32) -> String {
        let client = RcdClient::new_http_client(
            "".to_string(),
            "".to_string(),
            0,
            http_addr.to_string(),
            http_port,
        );

        let result = client.is_online_reply(test_message.to_string()).await;

        return result.reply_echo_message;
    }

    #[test]
    fn test() {
        let test_message: &str = "is_online_http";
        let (tx, rx) = mpsc::channel();

        let root_dir = test_harness::get_test_temp_dir(test_message);
        println!("{}", root_dir);

        let mut service = get_service_from_config_file();

        let http_addr = service.rcd_settings.http_addr.clone();
        let http_port = service.rcd_settings.http_port;

        let addr1 = http_addr.clone();
        let addr2 = http_addr.clone();

        println!("{:?}", &service);
        service.start_at_dir(&root_dir);

        info!("starting client service");

        thread::spawn(move || {
            let _service =
                service.start_http_at_addr_and_dir(addr1, http_port, root_dir);
        });

        let time = time::Duration::from_secs(1);

        info!("sleeping for 1 seconds...");

        thread::sleep(time);

        thread::spawn(move || {
            let res = client(test_message, &addr2, http_port.into());
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        println!("test_is_online: got: {} sent: {}", response, test_message);

        assert_eq!(response, test_message);
    }
}
