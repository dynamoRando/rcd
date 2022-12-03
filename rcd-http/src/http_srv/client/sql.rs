use super::get_core;
use rcdproto::rcdp::{
    ExecuteReadReply, ExecuteReadRequest, ExecuteWriteReply, ExecuteWriteRequest,
};
use rocket::{http::Status, post, serde::json::Json};

#[post(
    "/client/sql/host/read",
    format = "application/json",
    data = "<request>"
)]
pub async fn read_at_host(
    request: Json<ExecuteReadRequest>,
) -> (Status, Json<ExecuteReadReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST

    let result = get_core().execute_read_at_host(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/sql/host/write",
    format = "application/json",
    data = "<request>"
)]
pub async fn write_at_host(
    request: Json<ExecuteWriteRequest>,
) -> (Status, Json<ExecuteWriteReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST

    let result = get_core().execute_write_at_host(request.into_inner()).await;

    (Status::Ok, Json(result))
}
