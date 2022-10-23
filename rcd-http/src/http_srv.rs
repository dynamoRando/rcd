use rocket::{routes, get, fairing::{Fairing, Info}, http::Status};
use std::{thread};
use rocket::Shutdown;
use log::info;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Kind};

mod client;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
pub async fn start() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(CORS)
        .mount("/", routes![index, client::status, client::version, shutdown, client::database::get])
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
pub async fn start_http() {
    thread::spawn(move || {
        let _ = start();
    });
}

pub async fn shutdown_http() {
    let _ = reqwest::get("http://127.0.0.1:8000/shutdown").await.unwrap();
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_status(Status::Ok)
    }
}