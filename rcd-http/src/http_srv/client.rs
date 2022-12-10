use crate::http_srv::Core;
use rcd_common::defaults;
use rcdproto::rcdp::{ChangeHostStatusReply, ChangeHostStatusRequest, TestReply, TestRequest, TryAuthAtParticipantRequest, TryAuthAtPartipantReply};
use rocket::{get, http::Status, post, serde::json::Json, State};

pub mod contract;
pub mod database;
pub mod host;
pub mod sql;

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

#[post(
    "/client/change-host-status-id",
    format = "application/json",
    data = "<request>"
)]
pub async fn change_host_status_id(
    request: Json<ChangeHostStatusRequest>,
    state: &State<Core>,
) -> (Status, Json<ChangeHostStatusReply>) {
    let core = state.get_core();

    let response = core.change_host_status(request.into_inner()).await;

    (Status::Ok, Json(response))
}

#[post(
    "/client/change-host-status-name",
    format = "application/json",
    data = "<request>"
)]
pub async fn change_host_status_name(
    request: Json<ChangeHostStatusRequest>,
    state: &State<Core>,
) -> (Status, Json<ChangeHostStatusReply>) {
    let core = state.get_core();

    let response = core.change_host_status(request.into_inner()).await;

    (Status::Ok, Json(response))
}


#[post(
    "/client/try-auth-participant",
    format = "application/json",
    data = "<request>"
)]
pub async fn try_auth_at_participant(
    request: Json<TryAuthAtParticipantRequest>,
    state: &State<Core>,
) -> (Status, Json<TryAuthAtPartipantReply>) {
    let core = state.get_core();

    let response = core.try_auth_at_participant(request.into_inner()).await;

    (Status::Ok, Json(response))
}
