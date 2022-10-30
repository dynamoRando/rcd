use rcd_common::rcd_enum::PartialDataResultAction;
use rcdproto::rcdp::{
    AcceptPendingActionReply, AcceptPendingActionRequest, ChangeHostStatusReply,
    ChangeHostStatusRequest, GenerateHostInfoReply, GenerateHostInfoRequest,
    GetPendingActionsReply, GetPendingActionsRequest, PendingStatement,
};

use super::Rcd;

pub async fn generate_host_info(
    core: &Rcd,
    request: GenerateHostInfoRequest,
) -> GenerateHostInfoReply {
    let mut is_generate_successful = false;

    let host_name = request.host_name.clone();

    let auth_result = core.verify_login(request.authentication.unwrap());

    if auth_result.0 {
        core.dbi().rcd_generate_host_info(&host_name);
        is_generate_successful = true;
    }

    let generate_host_info_result = GenerateHostInfoReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_generate_successful,
    };

    return generate_host_info_result;
}

pub async fn change_host_status(
    core: &Rcd,
    request: ChangeHostStatusRequest,
) -> ChangeHostStatusReply {
    let host_name = request.host_alias.clone();
    let host_id = request.host_id.clone();
    let status = request.status;

    let mut name_result = false;
    let mut id_result = false;

    let auth_result = core.verify_login(request.authentication.unwrap());

    if auth_result.0 {
        name_result = core.dbi().change_host_status_by_name(&host_name, status);

        if !name_result {
            id_result = core.dbi().change_host_status_by_id(&host_id, status);
        }
    }

    let result = ChangeHostStatusReply {
        authentication_result: Some(auth_result.1),
        is_successful: name_result || id_result,
        status,
    };

    return result;
}

pub async fn get_pending_updates_at_participant(
    core: &Rcd,
    request: GetPendingActionsRequest,
) -> GetPendingActionsReply {
    let db_name = &request.database_name;
    let table_name = &request.table_name;
    let action = &request.action;

    let auth_result = core.verify_login(request.authentication.unwrap());
    let mut pending_statements: Vec<PendingStatement> = Vec::new();

    if auth_result.0 {
        pending_statements = core.dbi().get_pending_actions(db_name, table_name, &action);
    }

    let result = GetPendingActionsReply {
        authentication_result: Some(auth_result.1),
        pending_statements: pending_statements,
    };

    return result;
}

pub async fn accept_pending_action_at_participant(
    core: &Rcd,
    request: AcceptPendingActionRequest,
) -> AcceptPendingActionReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let mut is_local_update_successful = false;
    let mut is_remote_update_successful = false;

    if auth_result.0 {
        let db_name = &request.database_name;
        let table_name = &request.table_name;
        let row_id = request.row_id;

        let data_result = core
            .dbi()
            .accept_pending_action_at_participant(db_name, table_name, row_id);

        println!("{:?}", data_result);
        println!(
            "is_local_update_successful: {}",
            is_local_update_successful.to_string()
        );

        if data_result.is_successful {
            is_local_update_successful = true;

            let remote_host = core.dbi().get_cds_host_for_part_db(&db_name).unwrap();
            let own_host_info = core.dbi().rcd_get_host_info().clone();
            let hash = data_result.data_hash;

            let is_deleted = match data_result.action {
                Some(action) => match action {
                    PartialDataResultAction::Unknown => false,
                    PartialDataResultAction::Insert => false,
                    PartialDataResultAction::Update => false,
                    PartialDataResultAction::Delete => true,
                },
                None => false,
            };

            let notify_is_successful = core
                .remote()
                .notify_host_of_updated_hash(
                    &remote_host,
                    &own_host_info,
                    db_name,
                    table_name,
                    row_id,
                    hash,
                    is_deleted,
                )
                .await;

            println!("notify_is_successful: {}", notify_is_successful.to_string());

            if notify_is_successful {
                is_remote_update_successful = true;
            }
        }
    } else {
        println!("not authenticated");
    }

    let result = AcceptPendingActionReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_local_update_successful && is_remote_update_successful,
    };

    return result;
}
