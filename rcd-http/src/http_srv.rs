use lazy_static::lazy_static;
use log::info;
use rcd_core::rcd::Rcd;
use rocket::fairing::Kind;
use rocket::http::Header;
use rocket::Shutdown;
use rocket::{
    fairing::{Fairing, Info},
    get,
    http::Status,
    routes,
};
use rocket::{Request, Response};
use std::sync::Mutex;
use std::thread;

mod client;

pub struct Core {
    core: Option<Rcd>,
}

impl Core {
    pub fn set(&mut self, core: Rcd) {
        self.core = Some(core.clone());
    }

    pub fn get(&self) -> Rcd {
        return self.core.as_ref().unwrap().clone();
    }
}

lazy_static! {
    pub static ref CORE: Mutex<Core> = Mutex::new(Core { core: None });
}

pub fn get_core() -> Rcd {
    return CORE.lock().unwrap().get();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
pub async fn start() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(CORS)
        .mount(
            "/",
            routes![
                index,
                client::status,
                client::version,
                shutdown,
                client::database::post_get_databases,
                client::database::get_logical_storage_policy,
                client::database::set_logical_storage_policy,
                client::sql::post_read_at_host,
                client::sql::post_write_at_host,
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
pub async fn start_http(core: Rcd) {
    CORE.lock().unwrap().set(core);

    thread::spawn(move || {
        let _ = start();
    });
}

pub async fn shutdown_http() {
    let _ = reqwest::get("http://127.0.0.1:8000/shutdown")
        .await
        .unwrap();
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
