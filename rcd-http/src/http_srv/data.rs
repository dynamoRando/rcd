pub mod contract;
pub mod io;

use rcd_common::defaults;
use rcdproto::rcdp::{TestReply, TestRequest, TryAuthRequest, TryAuthResult};
use rocket::{get, http::Status, post, serde::json::Json, State};

use crate::http_srv::Core;

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

#[post("/data/try-auth", format = "application/json", data = "<request>")]
pub async fn try_auth(
    request: Json<TryAuthRequest>,
    state: &State<Core>,
) -> (Status, Json<TryAuthResult>) {
    let core = state.get_data();
    let result = core.try_auth(request.into_inner()).await;

    (Status::Ok, Json(result))
}
