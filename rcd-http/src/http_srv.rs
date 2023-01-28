use lazy_static::lazy_static;
use log::info;
use rcd_core::rcd::Rcd;
use rcd_core::rcd_data::RcdData;
use rocket::fairing::Kind;
use rocket::http::Header;
use rocket::log::LogLevel;
use rocket::{
    fairing::{Fairing, Info},
    get,
    http::Status,
    routes,
};
use rocket::{Config, Shutdown};
use rocket::{Request, Response};
use std::sync::Mutex;
use std::thread;

mod client;
mod data;

#[derive(Debug)]
pub struct Core {
    core: Option<Rcd>,
    data: Option<RcdData>,
    addr: String,
    port: u16,
}

impl Core {
    pub fn set_addr(&mut self, addr: String) {
        self.addr = addr;
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn set_core(&mut self, core: Rcd) {
        self.core = Some(core);
    }

    pub fn get_core(&self) -> Rcd {
        return self.core.as_ref().unwrap().clone();
    }

    pub fn set_data(&mut self, data: RcdData) {
        self.data = Some(data);
    }

    pub fn get_data(&self) -> RcdData {
        return self.data.as_ref().unwrap().clone();
    }

    pub fn get_addr(&mut self) -> String {
        self.addr.clone()
    }

    pub fn get_port(&mut self) -> u16 {
        self.port
    }
}

lazy_static! {
    pub static ref CORE: Mutex<Core> = Mutex::new(Core {
        core: None,
        data: None,
        addr: "".to_string(),
        port: 0,
    });
}

fn get_core() -> Rcd {
    return CORE.lock().unwrap().get_core();
}

fn get_data() -> RcdData {
    return CORE.lock().unwrap().get_data();
}

fn get_addr() -> String {
    return CORE.lock().unwrap().get_addr();
}

fn get_port() -> u16 {
    return CORE.lock().unwrap().get_port();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
pub async fn start() -> Result<(), rocket::Error> {
    // let config = Config {
    //     port: get_port(),
    //     address: get_addr().parse().unwrap(),
    //     log_level: LogLevel::Debug,
    //     ..Config::debug_default()
    // };

    let config = Config {
        port: get_port(),
        address: get_addr().parse().unwrap(),
        log_level: LogLevel::Normal,
        cli_colors: false,
        ..Config::debug_default()
    };

    let core = Core {
        core: Some(get_core()),
        data: Some(get_data()),
        addr: get_addr(),
        port: get_port(),
    };

    let _ = rocket::custom(config)
        .attach(CORS)
        .mount(
            "/",
            routes![
                index,
                client::status,
                client::version,
                shutdown,
                client::host::generate_host_info,
                client::host::get_host_info,
                client::host::get_cooperative_hosts,
                client::change_host_status_id,
                client::change_host_status_name,
                client::try_auth_at_participant,
                client::auth_for_token,
                client::revoke_token,
                client::get_settings,
                client::logs::get_logs_by_last_entries,
                client::database::new_database,
                client::database::has_table,
                client::database::post_get_databases,
                client::database::get_logical_storage_policy,
                client::database::set_logical_storage_policy,
                client::database::get_active_contact,
                client::database::get_row_id_at_participant,
                client::database::get_data_hash_at_participant,
                client::database::get_data_hash_at_host,
                client::database::participant::add_participant,
                client::database::participant::send_contract_to_participant,
                client::database::participant::get_participants,
                client::database::generate_contract,
                client::database::enable_coooperative_features,
                client::database::actions::accept_pending_action_at_participant,
                client::database::actions::get_pending_actions_at_participant,
                client::database::behavior::change_deletes_to_host_behavior,
                client::database::behavior::change_updates_to_host_behavior,
                client::database::behavior::change_deletes_from_host_behavior,
                client::database::behavior::change_updates_from_host_behavior,
                client::database::behavior::get_deletes_to_host_behavior,
                client::database::behavior::get_updates_to_host_behavior,
                client::database::behavior::get_deletes_from_host_behavior,
                client::database::behavior::get_updates_from_host_behavior,
                client::sql::read_at_host,
                client::sql::write_at_host,
                client::sql::cooperative_write_at_host,
                client::sql::write_at_participant,
                client::sql::read_at_participant,
                client::contract::review_pending_contracts,
                client::contract::accept_pending_contract,
                data::status,
                data::version,
                data::try_auth,
                data::contract::save_contract,
                data::contract::participant_accepts_contract,
                data::io::remove_row_at_participant,
                data::io::notify_host_of_removed_row,
                data::io::update_row_at_participant,
                data::io::insert_row_at_participant,
                data::io::get_row_at_participant,
                data::io::notify_host_of_updated_hash,
            ],
        )
        .manage(core)
        .launch()
        .await?;

    Ok(())
}

#[get("/shutdown")]
fn shutdown(shutdown: Shutdown) -> &'static str {
    shutdown.notify();
    let msg = "Shutting down http...";
    info!("{}", msg);
    msg
}

#[tokio::main]
pub async fn start_http(core: Rcd, data: RcdData, addr: String, port: u16) {
    CORE.lock().unwrap().set_core(core);
    CORE.lock().unwrap().set_data(data);
    CORE.lock().unwrap().set_addr(addr);
    CORE.lock().unwrap().set_port(port);

    thread::spawn(move || {
        let _ = start();
    });
}

pub async fn shutdown_http() {
    let http_addr = get_addr().clone();
    let http_port = get_port();
    let url = format!("http://{http_addr}:{http_port}/shutdown");
    let _ = reqwest::get(url).await.unwrap();
}

pub async fn shutdown_http_addr(addr: String, port: u32) {
    let url = format!("http://{addr}:{port}/shutdown");

    info!("Shutdown Request for http://{addr}:{port}");

    let _ = reqwest::get(url).await.unwrap();
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_status(Status::Ok)
    }
}
