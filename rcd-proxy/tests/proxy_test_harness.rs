use log::debug;
use rcd_proxy::RcdProxy;

#[derive(Debug, Clone)]
pub struct RcdProxyTestSetup {
    pub proxy: RcdProxy,
    pub test_host_id: String,
    pub test_un: String,
    pub test_pw: String,
}

#[cfg(test)]
/// common test code - sets up a test folder and returns a rcd proxy
fn test_setup(test_name: &str) -> RcdProxy {
    use rcd_test_harness::get_test_temp_dir;
    use std::env;

    let root_dir = get_test_temp_dir(test_name);
    let config_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let proxy = RcdProxy::get_proxy_from_config_with_dir(&config_dir, &root_dir).unwrap();
    proxy.start();
    proxy
}

#[cfg(test)]
/// common setup code - sets up the proxy instance and then returns an rcd service for the "test" user
pub fn harness_common_setup(test_name: &str) -> Option<RcdProxyTestSetup> {
    let proxy = test_setup(test_name);
    let result_register = proxy.register_user("test", "1234");

    if result_register.is_err() {
        assert!(false);
    }

    let result_setup = proxy.setup_user_folder(false);

    match result_setup {
        Ok(root_dir) => {
            let result_setup_rcd = proxy.setup_rcd_service("test", &root_dir);

            match result_setup_rcd {
                Ok(host_id) => {
                    debug!("{host_id:?}");
                    assert!(host_id.len() > 0);

                    let setup = RcdProxyTestSetup{
                        proxy: proxy, 
                        test_host_id: host_id.clone(),
                        test_un: "test".to_string(),
                        test_pw: "1234".to_string()
                    };

                    return Some(setup);
                }
                Err(_) => assert!(false),
            }
        }
        Err(_) => assert!(false),
    }

    None
}
