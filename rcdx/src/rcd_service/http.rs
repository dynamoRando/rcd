use rcd_core::{
    comm::{RcdCommunication, RcdRemoteDbClient},
    rcd::Rcd,
    rcd_data::RcdData,
    remote_http::RemoteHttp,
};
use rcd_http::http_srv;
use tokio::task;

use super::RcdService;

pub fn start_http_at_addr(service: &mut RcdService, http_addr: String, http_port: u16) {
    let dbi_settings = service.get_dbi();
    let dbi_core_clone = dbi_settings.clone();

    // start http, need to make this configurable
    let _ = task::spawn_blocking(move || {
        let http = RemoteHttp {
            own_http_addr: http_addr.clone(),
            own_http_port: http_port as u32,
        };

        let remote_client = RcdRemoteDbClient {
            comm_type: RcdCommunication::Http,
            grpc: None,
            http: Some(http),
        };

        let core = Rcd {
            db_interface: Some(dbi_settings),
            remote_client: Some(remote_client),
        };

        let data = RcdData {
            db_interface: Some(dbi_core_clone),
        };

        http_srv::start_http(core, data, http_addr, http_port);
    });
}

pub fn shutdown(addr: String, port: u32) {
    let _ = http_srv::shutdown_http_addr(addr, port);
}

pub fn start_http_at_addr_and_dir(
    service: &mut RcdService,
    http_addr: String,
    http_port: u16,
    root_dir: String,
) {
    let dbi_settings = service.get_dbi();
    dbi_settings.sqlite_config.unwrap().root_folder = root_dir;
    start_http_at_addr(service, http_addr, http_port)
}
