use super::get_core;
use rcdproto::rcdp::{
    CreateUserDatabaseReply, CreateUserDatabaseRequest, GenerateContractReply,
    GenerateContractRequest, GetActiveContractReply, GetActiveContractRequest, GetDatabasesReply,
    GetDatabasesRequest, GetLogicalStoragePolicyReply, GetLogicalStoragePolicyRequest,
    SetLogicalStoragePolicyReply, SetLogicalStoragePolicyRequest,
};
use rocket::{http::Status, post, serde::json::Json};

pub mod participant;

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

#[post(
    "/client/databases/table/policy/get",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_logical_storage_policy(
    request: Json<GetLogicalStoragePolicyRequest>,
) -> (Status, Json<GetLogicalStoragePolicyReply>) {
    let result = get_core()
        .get_logical_storage_policy(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/table/policy/set",
    format = "application/json",
    data = "<request>"
)]
pub async fn set_logical_storage_policy(
    request: Json<SetLogicalStoragePolicyRequest>,
) -> (Status, Json<SetLogicalStoragePolicyReply>) {
    let result = get_core()
        .set_logical_storage_policy(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/contract/generate",
    format = "application/json",
    data = "<request>"
)]
pub async fn generate_contract(
    request: Json<GenerateContractRequest>,
) -> (Status, Json<GenerateContractReply>) {
    let result = get_core().generate_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/new",
    format = "application/json",
    data = "<request>"
)]
pub async fn new_database(
    request: Json<CreateUserDatabaseRequest>,
) -> (Status, Json<CreateUserDatabaseReply>) {
    let result = get_core().create_user_database(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/contract/get",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_active_contact(
    request: Json<GetActiveContractRequest>,
) -> (Status, Json<GetActiveContractReply>) {
    let result = get_core().get_active_contact(request.into_inner()).await;

    (Status::Ok, Json(result))
}
