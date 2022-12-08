use lazy_static::lazy_static;
use log::info;
use rcd_core::rcd::Rcd;
use rcd_core::rcd_data::RcdData;
use rocket::fairing::Kind;
use rocket::http::Header;
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

pub struct Core {
    core: Option<Rcd>,
    data: Option<RcdData>,
    addr: String,
    port: u16,
}

impl Core {
    pub fn set_addr(&mut self, addr: String) {
        self.addr = addr.clone();
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn set_core(&mut self, core: Rcd) {
        self.core = Some(core.clone());
    }

    pub fn get_core(&self) -> Rcd {
        return self.core.as_ref().unwrap().clone();
    }

    pub fn set_data(&mut self, data: RcdData) {
        self.data = Some(data.clone());
    }

    pub fn get_data(&self) -> RcdData {
        return self.data.as_ref().unwrap().clone();
    }

    pub fn get_addr(&mut self) -> String {
        return self.addr.clone();
    }

    pub fn get_port(&mut self) -> u16 {
        return self.port;
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

pub fn get_core() -> Rcd {
    return CORE.lock().unwrap().get_core();
}

pub fn get_data() -> RcdData {
    return CORE.lock().unwrap().get_data();
}

pub fn get_addr() -> String {
    return CORE.lock().unwrap().get_addr();
}

pub fn get_port() -> u16 {
    return CORE.lock().unwrap().get_port();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
pub async fn start() -> Result<(), rocket::Error> {
    let config = Config {
        port: get_port(),
        address: get_addr().parse().unwrap(),
        ..Config::debug_default()
    };

    let _rocket = rocket::custom(config)
        .attach(CORS)
        .mount(
            "/",
            routes![
                index,
                client::status,
                client::version,
                shutdown,
                client::host::generate_host_info,
                client::database::new_database,
                client::database::post_get_databases,
                client::database::get_logical_storage_policy,
                client::database::set_logical_storage_policy,
                client::database::get_active_contact,
                client::database::participant::add_participant,
                client::database::participant::send_contract_to_participant,
                client::database::participant::get_participants,
                client::database::generate_contract,
                client::sql::read_at_host,
                client::sql::write_at_host,
                data::status,
                data::version,
                data::contract::save_contract,
            ],
        )
        .launch()
        .await?;

    Ok(())
}

#[get("/shutdown")]
fn shutdown(shutdown: Shutdown) -> &'static str {
    shutdown.notify();
    let msg = "Shutting down http...";
    info!("{}", msg);
    return msg;
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
    let url = format!("http://{}:{}/shutdown", http_addr, http_port);
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
