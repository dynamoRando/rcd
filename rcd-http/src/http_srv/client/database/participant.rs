use rcdproto::rcdp::{
    AddParticipantReply, AddParticipantRequest, GetParticipantsReply, GetParticipantsRequest,
    SendParticipantContractReply, SendParticipantContractRequest,
};
use rocket::{http::Status, post, serde::json::Json, State};

use crate::http_srv::Core;

#[post(
    "/client/databases/participant/add",
    format = "application/json",
    data = "<request>"
)]
pub async fn add_participant(
    request: Json<AddParticipantRequest>,
    state: &State<Core>,
) -> (Status, Json<AddParticipantReply>) {
    let result = state.get_core().add_participant(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/participant/send-contract",
    format = "application/json",
    data = "<request>"
)]
pub async fn send_contract_to_participant(
    request: Json<SendParticipantContractRequest>,
    state: &State<Core>,
) -> (Status, Json<SendParticipantContractReply>) {
    let result = state
        .get_core()
        .send_participant_contract(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/participant/get",
    format = "application/json",
    data = "<request>"
)]
pub async fn get_participants(
    request: Json<GetParticipantsRequest>,
    state: &State<Core>,
) -> (Status, Json<GetParticipantsReply>) {
    let result = state
        .get_core()
        .get_participants(request.into_inner())
        .await;

    (Status::Ok, Json(result))
}
