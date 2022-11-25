use super::get_core;
use rcdproto::rcdp::{
    GetDatabasesReply, GetDatabasesRequest, GetLogicalStoragePolicyReply,
    GetLogicalStoragePolicyRequest, SetLogicalStoragePolicyReply, SetLogicalStoragePolicyRequest,
};
use rocket::{http::Status, post, serde::json::Json};

#[post("/client/databases", format = "application/json", data = "<request>")]
pub async fn post_get_databases(
    request: Json<GetDatabasesRequest>,
) -> (Status, Json<GetDatabasesReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST

    let result = get_core().get_databases(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[allow(dead_code, unused_variables)]
#[post(
    "/client/databases/table/policy/get",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_logical_storage_policy(
    request: Json<GetLogicalStoragePolicyRequest>,
) -> (Status, Json<GetLogicalStoragePolicyReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST

    let result = get_core()
        .get_logical_storage_policy(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[allow(dead_code, unused_variables)]
#[post(
    "/client/databases/table/policy/set",
    format = "application/json",
    data = "<request>"
)]
pub async fn set_logical_storage_policy(
    request: Json<SetLogicalStoragePolicyRequest>,
) -> (Status, Json<SetLogicalStoragePolicyReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST

    let result = get_core()
        .set_logical_storage_policy(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}
