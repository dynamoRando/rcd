use super::get_core;
use rcdproto::rcdp::{GetDatabasesReply, GetDatabasesRequest};
use rocket::{http::Status, post, serde::json::Json};

#[allow(dead_code, unused_variables)]
#[post("/client/databases", format = "application/json", data = "<request>")]
pub fn post_get_databases(request: Json<GetDatabasesRequest>) -> (Status, Json<GetDatabasesReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST

    let result = get_core().get_databases(request.into_inner());

    (Status::Ok, Json(result))
}
