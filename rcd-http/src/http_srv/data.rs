pub mod contract;

use super::get_data;
use rcd_common::defaults;
use rcdproto::rcdp::{TestReply, TestRequest};
use rocket::{get, http::Status, post, serde::json::Json};

#[get("/data/status")]
pub async fn status() -> &'static str {
    "Status From Rocket"
}

#[post("/data/version", format = "application/json", data = "<request>")]
pub fn version(request: Json<TestRequest>) -> (Status, Json<TestReply>) {
    let response = TestReply {
        reply_time_utc: "".to_string(),
        reply_echo_message: request.request_echo_message.clone(),
        rcdx_version: defaults::VERSION.to_string(),
    };

    (Status::Ok, Json(response))
}
