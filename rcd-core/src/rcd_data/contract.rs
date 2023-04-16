use rcd_common::coop_database_participant::CoopDatabaseParticipant;
use rcd_enum::contract_status::ContractStatus;
use rcdproto::rcdp::{
    ParticipantAcceptsContractRequest, ParticipantAcceptsContractResult, SaveContractRequest,
    SaveContractResult,
};

use super::RcdData;
use tracing::{error, trace};

pub async fn accept_contract(
    core: &RcdData,
    request: ParticipantAcceptsContractRequest,
) -> ParticipantAcceptsContractResult {
    let debug_message_info = &request.message_info.as_ref().unwrap().clone();

    trace!("{debug_message_info:?}");
    trace!("{request:?}");

    let participant_message = request.participant.as_ref().unwrap().clone();

    let coop_db_participant: CoopDatabaseParticipant;

    let accepted_participant = core.dbi().get_participant_by_alias(
        &request.database_name,
        &request.participant.as_ref().unwrap().alias,
    );

    if accepted_participant.is_none() {
        let _participant = core.dbi().get_participant_by_id(
            &request.database_name,
            &request.participant.as_ref().unwrap().participant_guid,
        );

        if _participant.is_some() {
            trace!(
                "found participant: {:?}",
                _participant.as_ref().unwrap().clone()
            );
            coop_db_participant = _participant.unwrap();
        } else {
            error!("could not find participant by alias or id, about to panic");
            panic!();
        }
    } else {
        trace!(
            "found participant: {:?}",
            accepted_participant.as_ref().unwrap().clone()
        );
        coop_db_participant = accepted_participant.unwrap();
    }

    let is_successful = core.dbi().update_participant_accepts_contract(
        &request.database_name,
        coop_db_participant,
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
