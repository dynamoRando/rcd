use crate::{
    cdata::{
        AddParticipantReply, AddParticipantRequest, AuthResult, SendParticipantContractReply,
        SendParticipantContractRequest,
    },
    host_info::HostInfo,
    remote_db_srv,
};

use super::SqlClientImpl;

pub async fn add_participant(
    request: AddParticipantRequest,
    client: &SqlClientImpl,
) -> AddParticipantReply {
    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let alias = message.alias;
    let ip4addr = message.ip4_address;
    let db_port: u32 = message.port;

    let reply_message = String::from("");
    let mut is_successful = false;

    if is_authenticated {
        is_successful = client
            .dbi()
            .add_participant(&db_name, &alias, &ip4addr, db_port);
    };

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let add_participant_reply = AddParticipantReply {
        authentication_result: Some(auth_response),
        is_successful: is_successful,
        message: reply_message,
    };

    return add_participant_reply;
}

pub async fn send_participant_contract(
    request: SendParticipantContractRequest,
    client: &SqlClientImpl,
) -> SendParticipantContractReply {
    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();
    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let participant_alias = message.participant_alias;

    let reply_message = String::from("");
    let mut is_successful = false;

    if is_authenticated {
        if client.dbi().has_participant(&db_name, &participant_alias) {
            let participant = client
                .dbi()
                .get_participant_by_alias(&db_name, &participant_alias);
            let active_contract = client.dbi().get_active_contract(&db_name);
            let db_schema = client.dbi().get_database_schema(&db_name);
            let host_info = HostInfo::get(&client.dbi());
            is_successful = remote_db_srv::send_participant_contract(
                participant,
                host_info,
                active_contract,
                client.own_db_addr_port.clone(),
                db_schema,
            )
            .await;
        }
    };

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let send_participant_contract_reply = SendParticipantContractReply {
        authentication_result: Some(auth_response),
        is_sent: is_successful,
        message: reply_message,
    };

    return send_participant_contract_reply;
}
