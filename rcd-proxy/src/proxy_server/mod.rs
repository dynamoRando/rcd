use log::{info, debug};
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

use crate::RcdProxy;
use crate::proxy_server::account::register;

mod account;

#[derive(Debug, Clone)]
pub struct ProxyServer {
    port: u16,
    addr: String,
    proxy: RcdProxy,
}

impl ProxyServer {
    pub fn new(proxy: RcdProxy) -> Self {
        Self {
            port: proxy.settings.proxy_http_port as u16,
            addr: proxy.settings.proxy_http_addr.clone(),
            proxy: proxy,
        }
    }

    pub async fn start(&self) -> Result<(), rocket::Error> {
        debug!("starting ProxyServer with RcdProxy: {:?}", self.proxy);

        let proxy = self.proxy.clone();
        let config = Config {
            port: self.port,
            address: self.addr.parse().unwrap(),
            log_level: LogLevel::Normal,
            cli_colors: false,
            ..Config::debug_default()
        };
        let _ = rocket::custom(config)
            .attach(CORS)
            .mount("/", routes![index, register])
            .manage(proxy)
            .launch()
            .await?;

        Ok(())
    }
}

#[get("/shutdown")]
fn shutdown(shutdown: Shutdown) -> &'static str {
    shutdown.notify();
    let msg = "Shutting down http...";
    info!("{}", msg);
    msg
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
