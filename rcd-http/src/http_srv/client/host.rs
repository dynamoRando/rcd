use rcdproto::rcdp::{GenerateHostInfoReply, GenerateHostInfoRequest};
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
