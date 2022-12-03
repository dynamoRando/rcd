use super::get_core;
use rcdproto::rcdp::{AddParticipantReply, AddParticipantRequest, SendParticipantContractRequest, SendParticipantContractReply};
use rocket::{http::Status, post, serde::json::Json};

#[post(
    "/client/databases/participant/add",
    format = "application/json",
    data = "<request>"
)]
pub async fn add_participant(
    request: Json<AddParticipantRequest>,
) -> (Status, Json<AddParticipantReply>) {
    let result = get_core().add_participant(request.into_inner()).await;

    (Status::Ok, Json(result))
}

#[post(
    "/client/databases/participant/send-contract",
    format = "application/json",
    data = "<request>"
)]
pub async fn send_contract_to_participant(
    request: Json<SendParticipantContractRequest>,
) -> (Status, Json<SendParticipantContractReply>) {
    let result = get_core().send_participant_contract(request.into_inner()).await;

    (Status::Ok, Json(result))
}
