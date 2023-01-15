use rcdproto::rcdp::{
    DeleteDataRequest, DeleteDataResult, GetRowFromPartialDatabaseRequest,
    GetRowFromPartialDatabaseResult, InsertDataRequest, InsertDataResult,
    NotifyHostOfRemovedRowRequest, NotifyHostOfRemovedRowResponse, UpdateDataRequest,
    UpdateDataResult, UpdateRowDataHashForHostRequest, UpdateRowDataHashForHostResponse,
};
use rocket::{http::Status, post, serde::json::Json, State};

use crate::http_srv::Core;

#[post("/data/io/remove-row", format = "application/json", data = "<request>")]
pub async fn remove_row_at_participant(
    request: Json<DeleteDataRequest>,
    state: &State<Core>,
) -> (Status, Json<DeleteDataResult>) {
    let core = state.get_data();
    let result = core.delete_command_into_table(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/data/io/notify-host-removed-row",
    format = "application/json",
    data = "<request>"
)]
pub async fn notify_host_of_removed_row(
    request: Json<NotifyHostOfRemovedRowRequest>,
    state: &State<Core>,
) -> (Status, Json<NotifyHostOfRemovedRowResponse>) {
    let core = state.get_data();
    let result = core.notify_host_of_removed_row(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post("/data/io/update-row", format = "application/json", data = "<request>")]
pub async fn update_row_at_participant(
    request: Json<UpdateDataRequest>,
    state: &State<Core>,
) -> (Status, Json<UpdateDataResult>) {
    let core = state.get_data();
    let result = core.update_command_into_table(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post("/data/io/insert-row", format = "application/json", data = "<request>")]
pub async fn insert_row_at_participant(
    request: Json<InsertDataRequest>,
    state: &State<Core>,
) -> (Status, Json<InsertDataResult>) {
    let core = state.get_data();
    let result = core.insert_command_into_table(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post("/data/io/get-row", format = "application/json", data = "<request>")]
pub async fn get_row_at_participant(
    request: Json<GetRowFromPartialDatabaseRequest>,
    state: &State<Core>,
) -> (Status, Json<GetRowFromPartialDatabaseResult>) {
    let core = state.get_data();
    let result = core
        .get_row_from_partial_database(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/data/io/notify-host-updated-hash",
    format = "application/json",
    data = "<request>"
)]
pub async fn notify_host_of_updated_hash(
    request: Json<UpdateRowDataHashForHostRequest>,
    state: &State<Core>,
) -> (Status, Json<UpdateRowDataHashForHostResponse>) {
    let core = state.get_data();
    let result = core
        .update_row_data_hash_for_host(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}
