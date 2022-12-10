use rcdproto::rcdp::{
    AcceptPendingActionReply, AcceptPendingActionRequest, GetPendingActionsReply,
    GetPendingActionsRequest,
};
use rocket::{http::Status, post, serde::json::Json, State};

use crate::http_srv::Core;

#[post(
    "/client/databases/actions/accept-pending",
    format = "application/json",
    data = "<request>"
)]
pub async fn accept_pending_action_at_participant(
    request: Json<AcceptPendingActionRequest>,
    state: &State<Core>,
) -> (Status, Json<AcceptPendingActionReply>) {
    let result = state
        .get_core()
        .accept_pending_action_at_participant(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/actions/get-pending",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_pending_actions_at_participant(
    request: Json<GetPendingActionsRequest>,
    state: &State<Core>,
) -> (Status, Json<GetPendingActionsReply>) {
    let result = state
        .get_core()
        .get_pending_actions_at_participant(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}
