use super::get_core;
use rcd_common::defaults;
use rcdproto::rcdp::{TestReply, TestRequest};
use rocket::{get, http::Status, post, serde::json::Json};

pub mod database;

#[get("/client/status")]
pub async fn status() -> &'static str {
    "Status From Rocket"
}

#[post("/client/version", format = "application/json", data = "<request>")]
pub fn version(request: Json<TestRequest>) -> (Status, Json<TestReply>) {
    let response = TestReply {
        reply_time_utc: "".to_string(),
        reply_echo_message: request.request_echo_message.clone(),
        rcdx_version: defaults::VERSION.to_string(),
    };

    (Status::Ok, Json(response))
}
