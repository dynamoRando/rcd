use rcdproto::rcdp::{ChangeDeletesToHostBehaviorRequest, ChangeDeletesToHostBehaviorReply, ChangeUpdatesToHostBehaviorRequest, ChangeUpdatesToHostBehaviorReply, ChangeDeletesFromHostBehaviorRequest, ChangeDeletesFromHostBehaviorReply, ChangeUpdatesFromHostBehaviorRequest, ChangesUpdatesFromHostBehaviorReply};
use rocket::{http::Status, post, serde::json::Json, State};
use crate::http_srv::Core;


#[post(
    "/client/databases/behavior/change-deletes-to-host",
    format = "application/json",
    data = "<request>"
)]
pub async fn change_deletes_to_host_behavior(
    request: Json<ChangeDeletesToHostBehaviorRequest>,
    state: &State<Core>,
) -> (Status, Json<ChangeDeletesToHostBehaviorReply>) {
    let result = state
        .get_core()
        .change_deletes_to_host_behavior(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/behavior/change-updates-to-host",
    format = "application/json",
    data = "<request>"
)]
pub async fn change_updates_to_host_behavior(
    request: Json<ChangeUpdatesToHostBehaviorRequest>,
    state: &State<Core>,
) -> (Status, Json<ChangeUpdatesToHostBehaviorReply>) {
    let result = state
        .get_core()
        .change_updates_to_host_behavior(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/behavior/change-deletes-from-host",
    format = "application/json",
    data = "<request>"
)]
pub async fn change_deletes_from_host_behavior(
    request: Json<ChangeDeletesFromHostBehaviorRequest>,
    state: &State<Core>,
) -> (Status, Json<ChangeDeletesFromHostBehaviorReply>) {
    let result = state
        .get_core()
        .change_deletes_from_host_behavior(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/behavior/change-updates-from-host",
    format = "application/json",
    data = "<request>"
)]
pub async fn change_updates_from_host_behavior(
    request: Json<ChangeUpdatesFromHostBehaviorRequest>,
    state: &State<Core>,
) -> (Status, Json<ChangesUpdatesFromHostBehaviorReply>) {
    let result = state
        .get_core()
        .change_updates_from_host_behavior(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}