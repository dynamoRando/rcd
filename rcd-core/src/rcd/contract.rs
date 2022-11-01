use rcdproto::rcdp::{
    AcceptPendingContractReply, AcceptPendingContractRequest, Contract, ViewPendingContractsReply,
    ViewPendingContractsRequest,
};

use super::Rcd;

pub async fn accept_pending_contract(
    core: &Rcd,
    request: AcceptPendingContractRequest,
) -> AcceptPendingContractReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let mut is_accepted = false;
    let mut return_message = String::from("");

    if auth_result.0 {
        // 1 - we need to update the rcd_db record that we are accepting this contract
        // 2 - then we actually need to create the database with the properties of the
        // contract
        // 3 - we need to notify the host that we have accepted the contract

        let contracts = core.dbi().get_pending_contracts();
        let pending_contract = contracts
            .iter()
            .enumerate()
            .filter(|&(_, c)| {
                c.host_info.as_ref().unwrap().host_name.to_string() == request.host_alias
            })
            .map(|(_, c)| c);

        let param_contract = pending_contract.last().unwrap().clone();

        // 1 - accept the contract
        let is_contract_updated = core.dbi().accept_pending_contract(&request.host_alias);

        // 2 - create the database with the properties of the contract
        // make the database
        let db_is_created = core
            .dbi()
            .create_partial_database_from_contract(&param_contract);

        let self_host_info = core.dbi().rcd_get_host_info();
        // 3 - notify the host that we've accepted the contract
        let is_host_notified = core
            .remote()
            .notify_host_of_acceptance_of_contract(&param_contract, &self_host_info)
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

    let accepted_reply = AcceptPendingContractReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_accepted,
        message: return_message,
    };

    return accepted_reply;
}

pub async fn review_pending_contracts(
    core: &Rcd,
    request: ViewPendingContractsRequest,
) -> ViewPendingContractsReply {
    let auth_result = core.verify_login(request.authentication.unwrap());
    let mut pending_contracts: Vec<Contract> = Vec::new();

    if auth_result.0 {
        pending_contracts = core.dbi().get_pending_contracts();
    };

    let review_pending_contracts_reply = ViewPendingContractsReply {
        authentication_result: Some(auth_result.1),
        contracts: pending_contracts,
    };

    return review_pending_contracts_reply;
}
