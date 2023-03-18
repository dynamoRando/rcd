use log::debug;
use rcd_messages::{proxy::{server_messages::{
    ExecuteRequest, RegisterLoginReply, RegisterLoginRequest,
}, request_type::RequestType}, client::{CreateUserDatabaseRequest, AuthRequest}};
use rcd_proxy::proxy_server::ProxyServer;
use rcd_test_harness::{
    init_log_to_screen, init_log_to_screen_fern,
    test_common::proxy::{configure_proxy_for_test, RcdProxyTestType},
};
use rcd_test_proxy::get_http_result;

#[tokio::test]
async fn create_database() {
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

    {
        let proxy = setup.proxy.clone();
        tokio::spawn(async move {
            let proxy = proxy.clone();
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
            debug!("{result:?}");
            assert!(result.is_successful);
        })
        .await
        .unwrap();
    }
    {
        let proxy = setup.proxy.clone();
        let request_type: u16 = RequestType::CreateUserDatabase.into();

        let _auth = AuthRequest {
            user_name: "tester".to_string(),
            pw: "1234".to_string(),
            pw_hash: Vec::new(),
            token: Vec::new(),
            jwt: "".to_string(),
            id: None,
        };

        let _request = CreateUserDatabaseRequest {
            authentication: Some(_auth),
            database_name: "hopeitworks".to_string(),
        };

        tokio::spawn(async move {
            let request = ExecuteRequest {
                login: "tester".to_string(),
                pw: "1234".to_string(),
                request_type: request_type,
                request_json: "".to_string(),
            };

            let url = format!(
                "http://{}:{}/execute",
                proxy.http_endpoint_addr(),
                proxy.http_endpoint_port()
            );

            debug!("{url:?}");
            let result: RegisterLoginReply = get_http_result(url, request).await;
            debug!("{result:?}");
            assert!(result.is_successful);
        })
        .await
        .unwrap();
    }
}
