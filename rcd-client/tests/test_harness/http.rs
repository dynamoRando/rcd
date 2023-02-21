use log::{debug, warn};
use rcd_client::client_type::RcdClientType;
use rcdx::rcd_service::{get_service_from_config_file, RcdService};

use crate::test_harness::{TEST_SETTINGS, ServiceAddr, AddrType, delete_test_database, start_keepalive_for_test, sleep_instance, release_port};
use super::TestConfigHttp;


#[allow(dead_code)]
/// returns a tuple for the addr_port of the client service and the db service
pub fn start_service_with_http(
    test_db_name: &str,
    root_dir: String,
) -> TestConfigHttp {
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

    service.start_http_at_addr_and_dir("127.0.0.1".to_string(), http_port_num as u16, root_dir);

    let keep_alive = start_keepalive_for_test(RcdClientType::Grpc, http_addr.clone());
    let _ = keep_alive.send(true);

    sleep_instance();

    TestConfigHttp {
        http_address: http_addr,
        keep_alive: keep_alive
    }

}

#[allow(dead_code)]
pub fn shutdown_http(addr: &str, port: u32) {
    RcdService::shutdown_http(addr, port);
}


#[allow(dead_code)]
pub fn shutdown_http_test(main: &TestConfigHttp, participant: &TestConfigHttp) {
    debug!("shutting down http test...");

    if let Err(e) = main.keep_alive.send(false) {
        warn!("{e}");
    }

    if let Err(e) = participant.keep_alive.send(false) {
        warn!("{e}");
    }
    
    release_port(main.http_address.port);
    release_port(participant.http_address.port);

    shutdown_http(&main.http_address.ip4_addr, main.http_address.port);
    shutdown_http(&participant.http_address.ip4_addr, participant.http_address.port);


    debug!("shutting down test complete.");
}