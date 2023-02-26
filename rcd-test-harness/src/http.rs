use log::{debug, warn};
use rcd_client::client_type::RcdClientType;
use rcdx::rcd_service::{get_service_from_config_file, RcdService};

use super::TestConfigHttp;
use crate::{
    delete_test_database, release_port, sleep_instance, start_keepalive_for_test, AddrType,
    ServiceAddr, TEST_SETTINGS,
};

#[allow(dead_code)]
/// returns a tuple for the addr_port of the client service and the db service
pub fn start_service_with_http(
    test_db_name: &str,
     root_dir: String,
    use_internal_logging: bool) -> TestConfigHttp {
    let http_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();
    let mut service = get_service_from_config_file(None);

    let http_addr = ServiceAddr {
        ip4_addr: "127.0.0.1".to_string(),
        port: http_port_num,
        addr_type: AddrType::Client,
    };

    debug!("{:?}", &service);
    debug!("{:?}", &root_dir);

    service.start_at_dir(root_dir.as_str());

    let cwd = service.cwd();
    delete_test_database(test_db_name, &cwd);

    debug!("{:?}", &test_db_name);
    debug!("{:?}", &cwd);

    if use_internal_logging {
        service.enable_internal_logging(&root_dir, log::LevelFilter::Debug);
    }

    service.start_http_at_addr_and_dir("127.0.0.1".to_string(), http_port_num as u16, root_dir);

   
    let keep_alive = start_keepalive_for_test(RcdClientType::Grpc, http_addr.clone());
    let _ = keep_alive.send(true);

    sleep_instance();

    TestConfigHttp {
        http_address: http_addr,
        keep_alive: keep_alive,
    }
}

#[allow(dead_code)]
pub fn shutdown_http(addr: &str, port: u32) {
    RcdService::shutdown_http(addr, port);
}

#[allow(dead_code)]
pub fn shutdown_http_tests(instances: Vec<&TestConfigHttp>) {
    debug!("shutting down http test...");

    for instance in instances {
        if let Err(e) = instance.keep_alive.send(false) {
            warn!("{e}");
        }

        release_port(instance.http_address.port);
        shutdown_http(&instance.http_address.ip4_addr, instance.http_address.port);
    }
    debug!("shutting down test complete.");
}
