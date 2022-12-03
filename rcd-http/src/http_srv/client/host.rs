use super::get_core;
use rcdproto::rcdp::{GenerateHostInfoReply, GenerateHostInfoRequest};
use rocket::{http::Status, post, serde::json::Json};

#[post(
    "/client/host/generate",
    format = "application/json",
    data = "<request>"
)]
pub async fn generate_host_info(
    request: Json<GenerateHostInfoRequest>,
) -> (Status, Json<GenerateHostInfoReply>) {
    let result = get_core().generate_host_info(request.into_inner()).await;

    (Status::Ok, Json(result))
}
