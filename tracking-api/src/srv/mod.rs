use log::{debug, info};
use rcd_client::RcdClient;
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
use crate::srv::shark_event::get_events;
mod shark_event;

pub struct TrackingServer {
    port: u16,
    addr: String,
}

impl TrackingServer {
    pub fn new(addr: &str, port: u16) -> Self {
        Self {
            port: port,
            addr: addr.to_string(),
        }
    }

    pub async fn start(&self) -> Result<(), rocket::Error> {
        let config = Config {
            port: self.port,
            address: self.addr.parse().unwrap(),
            log_level: LogLevel::Normal,
            cli_colors: false,
            ..Config::debug_default()
        };
        let _ = rocket::custom(config)
            .attach(CORS)
            .mount("/", routes![index, get_events])
            .launch()
            .await?;

        Ok(())
    }
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

pub async fn get_client() -> RcdClient {
    let id = "59B2C8F5-9136-DBAC-F8A9-0903257B77D1";

    let mut client = RcdClient::new_grpc_client(
        "http://proxy.home:50051".to_string(),
        "shark".to_string(),
        "shark".to_string(),
        60,
    )
    .await;

    client.set_host_id(id);

    client
}
