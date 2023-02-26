use log::{debug, warn};
use rcd_client::client_type::RcdClientType;
use rcdx::rcd_service::get_service_from_config_file;

use crate::{
    delete_test_database, release_port, start_keepalive_for_test, AddrType, ServiceAddr,
    TEST_SETTINGS,
};

use super::TestConfigGrpc;

#[allow(dead_code)]
/// returns a tuple for the addr_port of the client service and the db service
pub fn start_service_with_grpc(
    test_db_name: &str,
    root_dir: String,
    use_internal_logging: bool,
) -> TestConfigGrpc {
    let (client_trigger, client_listener) = triggered::trigger();
    let (db_trigger, db_listener) = triggered::trigger();

    let client_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();
    let db_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();

    let mut service = get_service_from_config_file(None);

    let client_address_port = format!("{}{}", String::from("127.0.0.1:"), client_port_num);

    let client_addr = ServiceAddr {
        ip4_addr: "127.0.0.1:".to_string(),
        port: client_port_num,
        addr_type: AddrType::Client,
    };

    let db_address_port = format!("{}{}", String::from("127.0.0.1:"), db_port_num);

    let db_addr = ServiceAddr {
        ip4_addr: "127.0.0.1:".to_string(),
        port: db_port_num,
        addr_type: AddrType::Database,
    };

    debug!("{:?}", &service);
    debug!("{:?}", &root_dir);

    service.start_at_dir(root_dir.as_str());

    if use_internal_logging {
        service.enable_internal_logging(&root_dir, log::LevelFilter::Debug);
    }

    let db_name = service.rcd_settings.backing_database_name.clone();

    let cwd = service.cwd();
    delete_test_database(test_db_name, &cwd);

    let dir = root_dir.clone();

    let settings = Some(service.rcd_settings.clone());

    service
        .start_grpc_at_addrs_with_shutdown(
            db_name,
            client_address_port,
            db_address_port,
            dir,
            client_listener,
            db_listener,
            5,
            settings,
        )
        .unwrap();

    let keep_alive = start_keepalive_for_test(RcdClientType::Grpc, client_addr.clone());
    let _ = keep_alive.send(true);

    // sleep_instance();

    TestConfigGrpc {
        client_address: client_addr,
        database_address: db_addr,
        client_service_shutdown_trigger: client_trigger,
        database_service_shutdown_trigger: db_trigger,
        client_keep_alive: keep_alive,
    }
}

#[allow(dead_code)]
pub fn shutdown_grpc_tests(instances: Vec<&TestConfigGrpc>) {
    debug!("shutting down grpc test...");

    for instance in instances {
        if let Err(e) = instance.client_keep_alive.send(false) {
            warn!("{e}")
        }

        release_port(instance.client_address.port);
        release_port(instance.database_address.port);

        instance.client_service_shutdown_trigger.trigger();
        instance.database_service_shutdown_trigger.trigger();
    }

    debug!("shutting down test complete.");
}
