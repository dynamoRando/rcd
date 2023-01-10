use rcdproto::rcdp::{
    ParticipantAcceptsContractRequest, ParticipantAcceptsContractResult, SaveContractRequest,
    SaveContractResult,
};

use super::RcdData;

pub async fn accept_contract(
    core: &RcdData,
    request: ParticipantAcceptsContractRequest,
) -> ParticipantAcceptsContractResult {
    let debug_message_info = &request.message_info.as_ref().unwrap().clone();

    println!("{:?}", debug_message_info);

    let participant_message = request.participant.as_ref().unwrap().clone();

    let accepted_participant = core
        .dbi()
        .get_participant_by_alias(
            &request.database_name,
            &request.participant.as_ref().unwrap().alias,
        )
        .unwrap();

    let is_successful = core.dbi().update_participant_accepts_contract(
        &request.database_name,
        accepted_participant,
        participant_message,
        &request.contract_version_guid,
    );

    let result = ParticipantAcceptsContractResult {
        contract_acceptance_is_acknowledged: is_successful,
        error_message: String::from(""),
    };

    return result;
}

pub async fn save_contract(core: &RcdData, request: SaveContractRequest) -> SaveContractResult {
    let contract = request.contract.unwrap().clone();

    let save_is_successful = core.dbi().save_contract(contract);

    let result = SaveContractResult {
        is_saved: save_is_successful.0,
        error_message: save_is_successful.1.to_string(),
    };

    return result;
}
