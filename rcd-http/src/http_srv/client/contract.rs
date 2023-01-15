use rcdproto::rcdp::{
    AcceptPendingContractReply, AcceptPendingContractRequest, ViewPendingContractsReply,
    ViewPendingContractsRequest,
};
use rocket::{http::Status, post, serde::json::Json, State};

use crate::http_srv::Core;

#[post(
    "/client/contract/review",
    format = "application/json",
    data = "<request>"
)]
pub async fn review_pending_contracts(
    request: Json<ViewPendingContractsRequest>,
    state: &State<Core>,
) -> (Status, Json<ViewPendingContractsReply>) {
    let core = state.get_core();
    let result = core.review_pending_contracts(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/contract/accept",
    format = "application/json",
    data = "<request>"
)]
pub async fn accept_pending_contract(
    request: Json<AcceptPendingContractRequest>,
    state: &State<Core>,
) -> (Status, Json<AcceptPendingContractReply>) {
    let core = state.get_core();
    let result = core.accept_pending_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}
