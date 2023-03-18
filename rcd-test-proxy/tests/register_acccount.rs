use log::debug;
use rcd_messages::proxy::server_messages::{RegisterLoginReply, RegisterLoginRequest};
use rcd_proxy::proxy_server::ProxyServer;
use rcd_test_harness::{
    init_log_to_screen_fern, sleep_test,
    test_common::proxy::{configure_proxy_for_test, RcdProxyTestType}, init_log_to_screen,
};
use rcd_test_proxy::get_http_result;

#[tokio::test]
async fn register_account() {
    // init_log_to_screen_fern(log::LevelFilter::Debug);
    init_log_to_screen(log::LevelFilter::Debug);

    let setup = configure_proxy_for_test("proxy-i-register-user", RcdProxyTestType::Grpc);
    let proxy = setup.proxy.clone();

    {
        let proxy = setup.proxy.clone();
        let server = ProxyServer::new(proxy.clone());
        tokio::spawn(async move {
            proxy.start_grpc_client().await;
            proxy.start_grpc_data().await;
            server.start().await.unwrap();
        });
    }

    tokio::spawn(async move {
        let request = RegisterLoginRequest {
            login: "tester".to_string(),
            pw: "1234".to_string(),
        };
    
        let url = format!(
            "http://{}:{}/account/register",
            proxy.http_endpoint_addr(),
            proxy.http_endpoint_port()
        );
    
        debug!("{url:?}");
        let result: RegisterLoginReply = get_http_result(url, request).await;
        assert!(result.is_successful);
        
    }).await.unwrap();

}

