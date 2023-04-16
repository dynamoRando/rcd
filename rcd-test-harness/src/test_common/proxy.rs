use tracing::debug;
use rcd_proxy::{RcdProxy, RcdProxySettings};

use crate::{get_next_avail_port, ServiceAddr};

#[derive(Debug, Clone)]
pub struct RcdProxyTestSetup {
    pub proxy_info: RcdProxyInfo,
    pub main: RcdProxyTestUser,
    pub part: Option<RcdProxyTestUser>,
}

#[derive(Debug, Clone)]
pub struct RcdProxyInfo {
    pub proxy: RcdProxy,
    pub client_addr: ServiceAddr,
    pub db_addr: Option<ServiceAddr>,
}

#[derive(Debug, Clone)]
pub struct RcdProxyTestUser {
    pub host_id: String,
    pub un: String,
    pub pw: String,
}

#[derive(Debug, Clone)]
pub enum RcdProxyTestType {
    Grpc,
    Http,
}

/// common test code - sets up a test folder and returns a rcd proxy
pub fn configure_proxy_for_test(test_name: &str, proxy_type: RcdProxyTestType) -> RcdProxyInfo {
    use rcd_test_harness_common::get_test_temp_dir;
    let root_dir = get_test_temp_dir(test_name);

    match proxy_type {
        RcdProxyTestType::Grpc => {
            let client_port = get_next_avail_port();
            let client_addr = format!("127.0.0.1:{}", client_port);
            let db_port = get_next_avail_port();
            let db_addr = format!("127.0.0.1:{}", db_port);
            let proxy_http_port = get_next_avail_port();

            let settings = RcdProxySettings {
                use_grpc: true,
                use_http: false,
                grpc_client_addr_port: client_addr,
                grpc_db_addr_port: db_addr,
                http_ip: "127.0.0.1".to_string(),
                http_port: 0,
                database_type: rcd_enum::database_type::DatabaseType::Sqlite,
                database_name: "Proxy.db".to_string(),
                proxy_http_addr: "127.0.0.1".to_string(),
                proxy_http_port: proxy_http_port as usize,
                root_dir: root_dir,
            };

            let proxy = RcdProxy::get_proxy_with_config(settings);
            proxy.start();

            let client_addr = ServiceAddr {
                ip4_addr: "127.0.0.1:".to_string(),
                port: client_port,
                addr_type: crate::AddrType::Client,
            };

            let db_addr = ServiceAddr {
                ip4_addr: "127.0.0.1:".to_string(),
                port: db_port,
                addr_type: crate::AddrType::Database,
            };

            RcdProxyInfo {
                proxy: proxy,
                client_addr: client_addr,
                db_addr: Some(db_addr),
            }
        }
        RcdProxyTestType::Http => {
            let port = get_next_avail_port();

            let settings = RcdProxySettings {
                use_grpc: false,
                use_http: true,
                grpc_client_addr_port: "127.0.0.1:0".to_string(),
                grpc_db_addr_port: "127.0.0.1:0".to_string(),
                http_ip: "127.0.0.1".to_string(),
                http_port: 0,
                database_type: rcd_enum::database_type::DatabaseType::Sqlite,
                database_name: "Proxy.db".to_string(),
                proxy_http_addr: "127.0.0.1".to_string(),
                proxy_http_port: port as usize,
                root_dir: root_dir,
            };

            let proxy = RcdProxy::get_proxy_with_config(settings);
            proxy.start();
            let client_addr = ServiceAddr {
                ip4_addr: "127.0.0.1".to_string(),
                port: port,
                addr_type: crate::AddrType::Client,
            };

            let db_addr = ServiceAddr {
                ip4_addr: "127.0.0.1".to_string(),
                port: port,
                addr_type: crate::AddrType::Database,
            };

            RcdProxyInfo {
                proxy: proxy,
                client_addr: client_addr,
                db_addr: Some(db_addr),
            }
        }
    }
}

fn register_proxy_user(proxy: &RcdProxy, un: &str, pw: &str) -> Option<RcdProxyTestUser> {
    let result_register = proxy.register_user(un, pw);

    if result_register.is_err() {
        assert!(false);
    }

    let result_setup = proxy.setup_user_folder(false);

    match result_setup {
        Ok(root_dir) => {
            let result_setup_rcd = proxy.setup_rcd_service(un, &root_dir);

            match result_setup_rcd {
                Ok(host_id) => {
                    debug!("{host_id:?}");
                    assert!(host_id.len() > 0);

                    return Some(RcdProxyTestUser {
                        host_id,
                        un: un.to_string(),
                        pw: pw.to_string(),
                    });
                }
                Err(_) => assert!(false),
            }
        }
        Err(_) => assert!(false),
    }

    None
}

/// common setup code - sets up the proxy instance and then returns an rcd service for the "test" user
pub fn setup_proxy_with_users(
    test_name: &str,
    main_and_participant: bool,
    proxy_type: RcdProxyTestType,
) -> Option<RcdProxyTestSetup> {
    let proxy = configure_proxy_for_test(test_name, proxy_type);

    if main_and_participant {
        if let Some(main) = register_proxy_user(&proxy.proxy, "tester", "123456") {
            if let Some(part) = register_proxy_user(&proxy.proxy, "part", "123456") {
                return Some(RcdProxyTestSetup {
                    proxy_info: proxy,
                    main: main,
                    part: Some(part),
                });
            }
        };
    }

    if let Some(main) = register_proxy_user(&proxy.proxy, "tester", "123456") {
        return Some(RcdProxyTestSetup {
            proxy_info: proxy,
            main: main,
            part: None,
        });
    }

    None
}
