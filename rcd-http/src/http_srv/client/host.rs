use rcdproto::rcdp::{
    AuthRequest, GenerateHostInfoReply, GenerateHostInfoRequest, GetCooperativeHostsReply,
    GetCooperativeHostsRequest, HostInfoReply,
};
use rocket::{http::Status, post, serde::json::Json, State};

use crate::http_srv::Core;

#[post(
    "/client/host/generate",
    format = "application/json",
    data = "<request>"
)]
pub async fn generate_host_info(
    request: Json<GenerateHostInfoRequest>,
    state: &State<Core>,
) -> (Status, Json<GenerateHostInfoReply>) {
    let result = state
        .get_core()
        .generate_host_info(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post("/client/host/get", format = "application/json", data = "<request>")]
pub async fn get_host_info(
    request: Json<AuthRequest>,
    state: &State<Core>,
) -> (Status, Json<HostInfoReply>) {
    let result = state.get_core().get_host_info(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/host/get-coop-hosts",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_cooperative_hosts(
    request: Json<GetCooperativeHostsRequest>,
    state: &State<Core>,
) -> (Status, Json<GetCooperativeHostsReply>) {
    let result = state
        .get_core()
        .get_cooperative_hosts(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}
