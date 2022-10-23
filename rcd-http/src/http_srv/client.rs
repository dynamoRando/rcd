use rcdproto::rcdp::{TestRequest, TestReply};
use rocket::{serde::{json::Json}, get, post};
use rcd_common::defaults;

#[get("/client/status")]
pub async fn status() -> &'static str {
    "Status"
}

#[post("/client/version", format = "application/json", data = "<request>")]
pub fn version(request: Json<TestRequest>) -> Json<TestReply> { 
    let response = TestReply {
        reply_time_utc: "".to_string(),
        reply_echo_message: request.request_echo_message.clone(),
        rcdx_version: defaults::VERSION.to_string()
    };

    Json(response)
 }