use rocket::{routes, get};
use std::{env, thread};
use rocket::Shutdown;
use log::info;

mod client;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
pub async fn start() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, client::status, client::version, shutdown])
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