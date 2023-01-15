use rcdproto::rcdp::{
    ParticipantAcceptsContractRequest, ParticipantAcceptsContractResult, SaveContractRequest,
    SaveContractResult,
};
use rocket::{http::Status, post, serde::json::Json, State};

use crate::http_srv::Core;

#[post("/data/contract/save", format = "application/json", data = "<request>")]
pub async fn save_contract(
    request: Json<SaveContractRequest>,
    state: &State<Core>,
) -> (Status, Json<SaveContractResult>) {
    let core = state.get_data();
    let result = core.save_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/data/contract/accepted-by-participant",
    format = "application/json",
    data = "<request>"
)]
pub async fn participant_accepts_contract(
    request: Json<ParticipantAcceptsContractRequest>,
    state: &State<Core>,
) -> (Status, Json<ParticipantAcceptsContractResult>) {
    let core = state.get_data();
    let result = core.accept_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}
