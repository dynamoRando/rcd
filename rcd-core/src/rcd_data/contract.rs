use rcd_enum::contract_status::ContractStatus;
use rcdproto::rcdp::{
    ParticipantAcceptsContractRequest, ParticipantAcceptsContractResult, SaveContractRequest,
    SaveContractResult,
};

use super::RcdData;
use log::trace;

pub async fn accept_contract(
    core: &RcdData,
    request: ParticipantAcceptsContractRequest,
) -> ParticipantAcceptsContractResult {
    let debug_message_info = &request.message_info.as_ref().unwrap().clone();

    trace!("{debug_message_info:?}");

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

    ParticipantAcceptsContractResult {
        contract_acceptance_is_acknowledged: is_successful,
        error_message: String::from(""),
    }
}

pub async fn save_contract(core: &RcdData, request: SaveContractRequest) -> SaveContractResult {
    let contract = request.contract.unwrap();

    let save_result = core.dbi().save_contract(contract);
    let status = ContractStatus::to_u32(save_result.contract_status);

    SaveContractResult {
        is_saved: save_result.is_successful,
        contract_status: status,
        participant_info: save_result.participant_information,
    }
}
