pub mod grpc {

    #[cfg(test)]
    use log::info;
    use rcdx::rcd_service::get_service_from_config_file;
    extern crate futures;
    extern crate tokio;
    #[cfg(test)]
    use crate::test_harness;
    #[cfg(test)]
    use std::sync::mpsc;
    #[cfg(test)]
    use std::{thread, time};

    #[test]
    fn test() {
        let test_name = "enable_coooperative_features";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = test_harness::get_test_temp_dir(test_name);
        println!("{}", root_dir);

        let mut service = get_service_from_config_file();
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_grpc_client_service_at_addr(client_address_port, root_dir);
        });

        let time = time::Duration::from_secs(1);

        info!("sleeping for 1 seconds...");

        thread::sleep(time);

        thread::spawn(move || {
            let res = client(&test_db_name, &target_client_address_port);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        println!("create_enable_cooperative_features: got: {}", response);

        assert!(response);

        test_harness::release_port(port_num);
    }

    #[cfg(test)]
    #[tokio::main]
    async fn client(db_name: &str, addr_port: &str) -> bool {
        use rcd_client::client_type::RcdClientType;
        use rcd_client::RcdClient;

        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!(
            "client_create_enable_cooperative_features attempting to connect {}",
            addr_port
        );

        let client = RcdClient::new(
            addr_port,
            String::from("tester"),
            String::from("123456"),
            5,
            "".to_string(),
            0,
            RcdClientType::Grpc,
        );
        return client.enable_cooperative_features(db_name).await.unwrap();
    }
}
