use log::debug;
use rcd_enum::contract_status::ContractStatus;
use rcdproto::rcdp::{
    AddParticipantReply, AddParticipantRequest, Participant, SendParticipantContractReply,
    SendParticipantContractRequest, TryAuthAtParticipantRequest, TryAuthAtPartipantReply,
};

use super::Rcd;

const AUTO_UPDATE_PARTICIPANT_STATUS: bool = true;

pub async fn try_auth_at_participant(
    core: &Rcd,
    request: TryAuthAtParticipantRequest,
) -> TryAuthAtPartipantReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_participant = core
        .dbi()
        .get_participant_by_alias(&request.db_name, &request.participant_alias)
        .unwrap();

    let result = core
        .remote()
        .try_auth_at_participant(
            db_participant,
            &core.dbi().rcd_get_host_info().expect("no host info is set"),
        )
        .await;

    TryAuthAtPartipantReply {
        authentication_result: Some(auth_result.1),
        is_successful: result,
        message: String::from(""),
    }
}

pub async fn add_participant(core: &Rcd, request: AddParticipantRequest) -> AddParticipantReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let alias = request.alias;
    let ip4addr = request.ip4_address;
    let db_port: u32 = request.port;
    let http_addr = request.http_addr;
    let http_port = request.http_port;
    let id = request.id;

    let reply_message = String::from("");
    let mut is_successful = false;

    if auth_result.0 {
        is_successful = core.dbi().add_participant(
            &db_name,
            &alias,
            &ip4addr,
            db_port,
            http_addr,
            http_port as u16,
            id,
        );
    };

    AddParticipantReply {
        authentication_result: Some(auth_result.1),
        is_successful,
        message: reply_message,
    }
}

pub async fn send_participant_contract(
    core: &Rcd,
    request: SendParticipantContractRequest,
) -> SendParticipantContractReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let participant_alias = request.participant_alias;

    let mut is_successful = false;
    let mut contract_status: u32 = 0;

    if auth_result.0 && core.dbi().has_participant(&db_name, &participant_alias) {
        let participant = core
            .dbi()
            .get_participant_by_alias(&db_name, &participant_alias)
            .unwrap();
        let active_contract = core.dbi().get_active_contract(&db_name);
        let db_schema = core.dbi().get_database_schema(&db_name);
        let host_info = core.dbi().rcd_get_host_info().expect("no host info is set");
        let result = core
            .remote()
            .send_participant_contract(
                participant.clone(),
                host_info,
                active_contract.clone(),
                db_schema,
            )
            .await;

        debug!("send participant contract result: {result:?}");

        is_successful = result.is_successful;
        contract_status = ContractStatus::to_u32(result.contract_status);

        let participant_contract_status = ContractStatus::from_u32(contract_status);

        if AUTO_UPDATE_PARTICIPANT_STATUS
            && !is_successful
            && (participant_contract_status != ContractStatus::Pending)
        {
            debug!("saving updated contract status for participant: {result:?}");

            let _participant = participant.clone();

            let mut p = result.participant_information.unwrap().clone();
            p.ip4_address = _participant.ip4addr;
            p.ip6_address = _participant.ip6addr;
            p.http_addr = _participant.http_addr;
            p.http_port = _participant.http_port as u32;

            core.dbi().update_participant_accepts_contract(
                &db_name,
                participant.clone(),
                p,
                &active_contract.contract_id.to_string(),
            );
        }
    };

    SendParticipantContractReply {
        authentication_result: Some(auth_result.1),
        is_sent: is_successful,
        contract_status: contract_status,
    }
}
