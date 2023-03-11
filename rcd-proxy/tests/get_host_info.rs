use crate::proxy_test_harness::harness_common_setup;
use ignore_result::Ignore;
use log::debug;
use rcd_test_harness::{get_next_avail_port, sleep_test};
use simple_logger::SimpleLogger;
mod proxy_test_harness;
use rcd_client::RcdClient;

#[tokio::test]
pub async fn test_get_host_info_grpc() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .ignore();

    let setup = harness_common_setup("proxy-t-i-get-host-info").unwrap();
    let port = get_next_avail_port();
    let addr = format!("127.0.0.1:{}", port);
    let setup_host_id = setup.test_host_id.clone();
    let proxy = setup.proxy.clone();

    {
        let addr = addr.clone();
        tokio::spawn(async move {
            debug!("starting proxy");
            proxy.start_grpc_client_at_addr(&addr).await;
            debug!("ending proxy");
        });
    }

    sleep_test();

    {
        let addr = addr.clone();
        tokio::spawn(async move {
            let addr = format!("http://{}", addr);
            let mut client =
                RcdClient::new_grpc_client(addr, "test".to_string(), "1234".to_string(), 60).await;
            client.set_host_id(&setup_host_id);
            let info = client.get_host_info().await.unwrap();
            debug!("{info:?}");
            let reply_id = info.host_info.unwrap().host_guid.clone();
            assert_eq!(reply_id, setup_host_id);
        })
        .await
        .unwrap();
    }
}
