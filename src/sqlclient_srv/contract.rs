use crate::{
    cdata::{
        AcceptPendingContractReply, AcceptPendingContractRequest, AuthResult, Contract,
        ViewPendingContractsReply, ViewPendingContractsRequest,
    },
    remote_db_srv,
};

use super::SqlClientImpl;

pub async fn review_pending_contracts(
    request: ViewPendingContractsRequest,
    client: &SqlClientImpl,
) -> ViewPendingContractsReply {
    let message = request.clone();
    let a = message.authentication.unwrap();
    let is_authenticated = client.verify_login(&a.user_name, &a.pw);

    let mut pending_contracts: Vec<Contract> = Vec::new();

    if is_authenticated {
        pending_contracts = client.dbi().get_pending_contracts();
    };

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let review_pending_contracts_reply = ViewPendingContractsReply {
        authentication_result: Some(auth_response),
        contracts: pending_contracts,
    };

    return review_pending_contracts_reply;
}

pub async fn accept_pending_contract(
    request: AcceptPendingContractRequest,
    client: &SqlClientImpl,
) -> AcceptPendingContractReply {
    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();
    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let mut is_accepted = false;
    let mut return_message = String::from("");

    if is_authenticated {
        // 1 - we need to update the rcd_db record that we are accepting this contract
        // 2 - then we actually need to create the database with the properties of the
        // contract
        // 3 - we need to notify the host that we have accepted the contract

        let contracts = client.dbi().get_pending_contracts();
        let pending_contract = contracts
            .iter()
            .enumerate()
            .filter(|&(_, c)| {
                c.host_info.as_ref().unwrap().host_name.to_string() == message.host_alias
            })
            .map(|(_, c)| c);

        let param_contract = pending_contract.last().unwrap().clone();

        // 1 - accept the contract
        let is_contract_updated = client.dbi().accept_pending_contract(&message.host_alias);

        // 2 - create the database with the properties of the contract
        // make the database
        let db_is_created = client
            .dbi()
            .create_partial_database_from_contract(&param_contract);

        let self_host_info = client.dbi().rcd_get_host_info();
        // 3 - notify the host that we've accepted the contract
        let is_host_notified = remote_db_srv::notify_host_of_acceptance_of_contract(
            &param_contract,
            &self_host_info,
            client.own_db_addr_port.clone(),
        )
        .await;

        if is_contract_updated && db_is_created && is_host_notified {
            is_accepted = true;
            return_message = String::from("accepted contract successfuly");
        } else if !is_contract_updated {
            return_message = String::from("failed to update contract in rcd db");
        } else if !db_is_created {
            return_message = String::from("failed to to create partial db from contract");
        } else if !is_host_notified {
            return_message = String::from("failed to notify host of acceptance of contract");
        }
    };

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let accepted_reply = AcceptPendingContractReply {
        authentication_result: Some(auth_response),
        is_successful: is_accepted,
        message: return_message,
    };

    return accepted_reply;
}
