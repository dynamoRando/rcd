use lazy_static::lazy_static;
use log::info;
use rcd_core::dbi::Dbi;
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

pub struct HttpDbi {
    dbi: Option<Dbi>,
}

impl HttpDbi {
    pub fn set(&mut self, dbi: Dbi) {
        self.dbi = Some(dbi.clone());
    }

    pub fn get(&self) -> Dbi {
        return self.dbi.as_ref().unwrap().clone();
    }
}

lazy_static! {
    pub static ref HTTP_DBI: Mutex<HttpDbi> = Mutex::new(HttpDbi { dbi: None });
}

pub fn get_dbi() -> Dbi {
    return HTTP_DBI.lock().unwrap().get();
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
                client::database::post_get_databases
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
pub async fn start_http(dbi: Dbi) {
    HTTP_DBI.lock().unwrap().set(dbi);

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
