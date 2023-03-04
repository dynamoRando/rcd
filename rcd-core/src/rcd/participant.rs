use rcdproto::rcdp::{
    AddParticipantReply, AddParticipantRequest, SendParticipantContractReply,
    SendParticipantContractRequest, TryAuthAtParticipantRequest, TryAuthAtPartipantReply,
};

use super::Rcd;

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
    let mut error_message = String::from("");

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
            .send_participant_contract(participant, host_info, active_contract, db_schema)
            .await;

        is_successful = result.0;
        error_message = result.1;
    };

    SendParticipantContractReply {
        authentication_result: Some(auth_result.1),
        is_sent: is_successful,
        message: error_message,
    }
}
