use crate::http_srv::Core;
use rcdproto::rcdp::{GetLogsByLastNumberReply, GetLogsByLastNumberRequest};
use rocket::{http::Status, post, serde::json::Json, State};

#[post(
    "/client/logs/by-last-entries",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_logs_by_last_entries(
    request: Json<GetLogsByLastNumberRequest>,
    state: &State<Core>,
) -> (Status, Json<GetLogsByLastNumberReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST
    let core = state.get_core();
    let result = core.get_last_log_entries(request.into_inner()).await;

    (Status::Ok, Json(result))
}
