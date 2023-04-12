use log::{debug, trace};
use rcd_client::RcdClient;
use rocket::fairing::Kind;
use rocket::http::Header;
use rocket::log::LogLevel;
use rocket::Config;
use rocket::{
    fairing::{Fairing, Info},
    get,
    http::Status,
    routes,
};
use rocket::{Request, Response};

use self::user::create::create_account;

use crate::srv::shark_event::create::add_associated_event;
use crate::srv::shark_event::create::add_event;
use crate::srv::shark_event::delete::delete_associated_event;
use crate::srv::shark_event::delete::delete_event;
use crate::srv::shark_event::get::get_events;
use crate::srv::shark_event::update::update_associated_event;
use crate::srv::shark_event::update::update_event;
use crate::srv::user::get::auth_for_token;
use crate::srv::user::get::logout;
use crate::ApiSettings;
use crate::srv::user::get::user_id;

mod shark_event;
mod user;
mod util;

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

    pub async fn start(&self, settings: ApiSettings) -> Result<(), rocket::Error> {
        let config = Config {
            port: self.port,
            address: self.addr.parse().unwrap(),
            log_level: LogLevel::Normal,
            cli_colors: false,
            ..Config::debug_default()
        };
        let _ = rocket::custom(config)
            .attach(CORS)
            .mount(
                "/",
                routes![
                    index,
                    get_events,
                    update_event,
                    update_associated_event,
                    add_event,
                    add_associated_event,
                    delete_event,
                    delete_associated_event,
                    create_account,
                    version,
                    auth_for_token,
                    logout,
                    user_id
                ],
            )
            .manage(settings)
            .launch()
            .await?;

        Ok(())
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/version")]
fn version() -> &'static str {
    return env!("CARGO_PKG_VERSION");
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

pub async fn get_client(settings: &ApiSettings) -> RcdClient {
    // trace!("{settings:?}");

    let mut client = RcdClient::new_grpc_client(
        settings.proxy_addr.clone(),
        settings.proxy_user.clone(),
        settings.proxy_auth.clone(),
        60,
    )
    .await;

    client.set_host_id(&settings.id);

    client
}
