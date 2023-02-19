pub mod grpc {

    use crate::test_harness::{self, ServiceAddr};
    use log::{debug, info};
    use std::sync::mpsc;
    use std::thread;

    /*
    # Test Description

    */

    #[test]
    fn test() {
        let test_name = "get_host_info_grpc";
        let test_db_name = format!("{}{}", test_name, ".db");
        let (tx_main, rx_main) = mpsc::channel();
        let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);
        let main_test_config = test_harness::start_service_with_grpc(&test_db_name, dirs.main_dir);

        test_harness::sleep_test();

        let main_db_name = test_db_name;

        {
            let main_client_addr = main_test_config.client_address.clone();
            thread::spawn(move || {
                let res = main_service_client(&main_db_name, main_client_addr);
                tx_main.send(res).unwrap();
            })
            .join()
            .unwrap();
        }

        let has_host_name = rx_main.try_recv().unwrap();
        debug!("has_host_name: got: {has_host_name}");

        assert!(has_host_name);
    }

    #[cfg(test)]
    #[tokio::main]
    async fn main_service_client(db_name: &str, main_client_addr: ServiceAddr) -> bool {
        use rcd_client::RcdClient;

        info!(
            "main_service_client attempting to connect {}",
            main_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_grpc_client(
            main_client_addr.to_full_string_with_http(),
            String::from("tester"),
            String::from("123456"),
            60,
        )
        .await;

        client.create_user_database(db_name).await.unwrap();
        client.enable_cooperative_features(db_name).await.unwrap();
        client.generate_host_info("main").await.unwrap();

        let host_info = client.get_host_info().await.unwrap();

        host_info.host_info.unwrap().host_name == "main"
    }
}

pub mod http {

    use crate::test_harness::{self, ServiceAddr};
    use log::{debug, info};
    use std::sync::mpsc;
    use std::{thread, time};

    /*
    # Test Description

    */

    #[test]
    fn test() {
        let test_name = "get_host_info_http";
        let test_db_name = format!("{}{}", test_name, ".db");

        let (tx_main, rx_main) = mpsc::channel();

        let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);

        let main_addrs = test_harness::start_service_with_http(&test_db_name, dirs.main_dir);

        let m_keep_alive = main_addrs.1;
        let main_addrs = main_addrs.0;

        let time = time::Duration::from_secs(1);
        info!("sleeping for 1 seconds...");
        thread::sleep(time);

        let main_db_name = test_db_name;
        let ma3 = main_addrs.clone();

        thread::spawn(move || {
            let res = main_service_client(&main_db_name, main_addrs);
            tx_main.send(res).unwrap();
        })
        .join()
        .unwrap();

        let has_host_info = rx_main.try_recv().unwrap();
        debug!("has_host_info: got: {has_host_info}");

        assert!(has_host_info);

        let _ = m_keep_alive.send(false);
        test_harness::release_port(ma3.port);
        test_harness::shutdown_http(ma3.ip4_addr, ma3.port);
    }

    #[cfg(test)]
    #[tokio::main]

    async fn main_service_client(db_name: &str, main_client_addr: ServiceAddr) -> bool {
        use rcd_client::RcdClient;

        info!(
            "main_service_client attempting to connect {}",
            main_client_addr.to_full_string_with_http()
        );

        let mut client = RcdClient::new_http_client(
            String::from("tester"),
            String::from("123456"),
            60,
            main_client_addr.ip4_addr,
            main_client_addr.port,
        );
        client.create_user_database(db_name).await.unwrap();
        client.enable_cooperative_features(db_name).await.unwrap();
        client.generate_host_info("main").await.unwrap();

        let host_info = client.get_host_info().await.unwrap();

        host_info.host_info.unwrap().host_name == "main"
    }
}
