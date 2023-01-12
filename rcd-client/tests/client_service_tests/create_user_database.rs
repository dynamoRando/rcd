pub mod grpc {

    use log::info;
    use rcd_client::RcdClient;
    use rcdx::rcd_service::get_service_from_config_file;
    use std::sync::mpsc;
    use std::{thread};

    use crate::test_harness;

    #[test]
    fn test() {
        let test_name = "create_user_database_positive_grpc";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = test_harness::get_test_temp_dir(&test_name);
        println!("{}", root_dir);

        let mut service = get_service_from_config_file(None);
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

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

        println!("create_user_database: got: {}", response);

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
            5,
        ).await;
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

        let root_dir = test_harness::get_test_temp_dir(&test_name);
        println!("{}", root_dir);

        let mut service = get_service_from_config_file(None);
        let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

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

        println!("create_user_database: got: {}", response);

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
            5,
        ).await;
        
        return client.create_user_database(db_name).await.unwrap();
    }
}

pub mod http {

    use log::info;
    use rcd_client::RcdClient;
    use rcdx::rcd_service::{get_service_from_config_file, RcdService};
    use std::sync::mpsc;
    use std::{thread};

    use crate::test_harness;

    #[test]
    fn test() {
        let test_name = "create_user_database_positive_http";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = test_harness::get_test_temp_dir(&test_name);
        println!("{}", root_dir);

        let mut service = get_service_from_config_file(None);
        let client_address_port = format!("{}{}", String::from("127.0.0.1:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_http_at_addr_and_dir(
                "127.0.0.1".to_string(),
                port_num as u16,
                root_dir,
            );
        });

       test_harness::sleep_test();

        thread::spawn(move || {
            let res = client(&test_db_name, &target_client_address_port, port_num);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        println!("create_user_database: got: {}", response);

        assert!(response);

        test_harness::release_port(port_num);
        RcdService::shutdown_http("127.0.0.1".to_string(), port_num);
    }

    #[cfg(test)]
    #[tokio::main]
    async fn client(db_name: &str, addr_port: &str, port_num: u32) -> bool {
        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!(
            "client_create_user_database attempting to connect {}",
            addr_port
        );

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            5,
            "127.0.0.1".to_string(),
            port_num,
        );
        return client.create_user_database(db_name).await.unwrap();
    }

    #[test]
    fn negative_test() {
        let test_name = "create_user_database_negative";
        let test_db_name = format!("{}{}", test_name, ".db");

        let (tx, rx) = mpsc::channel();
        let port_num = test_harness::TEST_SETTINGS
            .lock()
            .unwrap()
            .get_next_avail_port();

        let root_dir = test_harness::get_test_temp_dir(&test_name);
        println!("{}", root_dir);

        let mut service = get_service_from_config_file(None);
        let client_address_port = format!("{}{}", String::from("127.0.0.1:"), port_num.to_string());
        let target_client_address_port = client_address_port.clone();
        println!("{:?}", &service);

        service.start_at_dir(&root_dir);

        info!("starting client at {}", &client_address_port);
        info!("starting client service");

        thread::spawn(move || {
            let _service = service.start_http_at_addr_and_dir(
                "127.0.0.1".to_string(),
                port_num as u16,
                root_dir,
            );
        });
    
        test_harness::sleep_test();

        thread::spawn(move || {
            let res = negative_client(&test_db_name, &target_client_address_port, port_num);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        println!("create_user_database: got: {}", response);

        assert!(!response);
        RcdService::shutdown_http("127.0.0.1".to_string(), port_num);
    }

    #[cfg(test)]
    #[tokio::main]
    async fn negative_client(db_name: &str, addr_port: &str, port_num: u32) -> bool {
        let addr_port = format!("{}{}", String::from("http://"), addr_port);
        info!(
            "client_create_user_database attempting to connect {}",
            addr_port
        );

        let mut client = RcdClient::new_http_client(
            String::from("wrong_user"),
            String::from("123456"),
            5,
            "127.0.0.1".to_string(),
            port_num,
        );
        return client.create_user_database(db_name).await.unwrap();
    }
}
