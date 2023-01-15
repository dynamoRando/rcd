use rcdproto::rcdp::{
    CreateUserDatabaseReply, CreateUserDatabaseRequest, EnableCoooperativeFeaturesReply,
    EnableCoooperativeFeaturesRequest, GenerateContractReply, GenerateContractRequest,
    GetActiveContractReply, GetActiveContractRequest, GetDataHashReply, GetDataHashRequest,
    GetDatabasesReply, GetDatabasesRequest, GetLogicalStoragePolicyReply,
    GetLogicalStoragePolicyRequest, GetReadRowIdsReply, GetReadRowIdsRequest, HasTableReply,
    HasTableRequest, SetLogicalStoragePolicyReply, SetLogicalStoragePolicyRequest,
};
use rocket::{http::Status, post, serde::json::Json, State};

use crate::http_srv::Core;

pub mod actions;
pub mod behavior;
pub mod participant;

#[post("/client/databases", format = "application/json", data = "<request>")]
pub async fn post_get_databases(
    request: Json<GetDatabasesRequest>,
    state: &State<Core>,
) -> (Status, Json<GetDatabasesReply>) {
    // note: this doesn't make sense for HTTP
    // this should be a GET instead of a POST
    // need to look at HTTP spec and figure out how to send
    // authorization in the header rather than a POST

    let result = state.get_core().get_databases(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/table/policy/get",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_logical_storage_policy(
    request: Json<GetLogicalStoragePolicyRequest>,
    state: &State<Core>,
) -> (Status, Json<GetLogicalStoragePolicyReply>) {
    let result = state
        .get_core()
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
    state: &State<Core>,
) -> (Status, Json<SetLogicalStoragePolicyReply>) {
    let result = state
        .get_core()
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
    state: &State<Core>,
) -> (Status, Json<GenerateContractReply>) {
    let core = state.get_core();
    let result = core.generate_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/new",
    format = "application/json",
    data = "<request>"
)]
pub async fn new_database(
    request: Json<CreateUserDatabaseRequest>,
    state: &State<Core>,
) -> (Status, Json<CreateUserDatabaseReply>) {
    let core = state.get_core();
    let result = core.create_user_database(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/contract/get",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_active_contact(
    request: Json<GetActiveContractRequest>,
    state: &State<Core>,
) -> (Status, Json<GetActiveContractReply>) {
    let core = state.get_core();
    let result = core.get_active_contact(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/enable-cooperative-features",
    format = "application/json",
    data = "<request>"
)]
pub async fn enable_coooperative_features(
    request: Json<EnableCoooperativeFeaturesRequest>,
    state: &State<Core>,
) -> (Status, Json<EnableCoooperativeFeaturesReply>) {
    let core = state.get_core();
    let result = core
        .enable_coooperative_features(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/participant/io/get",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_row_id_at_participant(
    request: Json<GetReadRowIdsRequest>,
    state: &State<Core>,
) -> (Status, Json<GetReadRowIdsReply>) {
    let core = state.get_core();
    let result = core.read_row_id_at_participant(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/participant/io/get-hash",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_data_hash_at_participant(
    request: Json<GetDataHashRequest>,
    state: &State<Core>,
) -> (Status, Json<GetDataHashReply>) {
    let core = state.get_core();
    let result = core
        .get_data_hash_at_participant(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/host/io/get-hash",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_data_hash_at_host(
    request: Json<GetDataHashRequest>,
    state: &State<Core>,
) -> (Status, Json<GetDataHashReply>) {
    let core = state.get_core();
    let result = core.get_data_hash_at_host(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/has_table",
    format = "application/json",
    data = "<request>"
)]
pub async fn has_table(
    request: Json<HasTableRequest>,
    state: &State<Core>,
) -> (Status, Json<HasTableReply>) {
    let core = state.get_core();
    let result = core.has_table(request.into_inner()).await;

    (Status::Ok, Json(result))
}
