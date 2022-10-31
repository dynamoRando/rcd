use rcd_common::rcd_enum::{
    PartialDataResultAction, RcdGenerateContractError, RemoteDeleteBehavior,
};
use rcdproto::rcdp::{
    AcceptPendingActionReply, AcceptPendingActionRequest, ChangeHostStatusReply,
    ChangeHostStatusRequest, CreateUserDatabaseReply, CreateUserDatabaseRequest,
    GenerateContractReply, GenerateContractRequest, GenerateHostInfoReply, GenerateHostInfoRequest,
    GetDataHashReply, GetDataHashRequest, GetPendingActionsReply, GetPendingActionsRequest,
    HasTableReply, HasTableRequest, PendingStatement, ChangeUpdatesFromHostBehaviorRequest, ChangesUpdatesFromHostBehaviorReply, GetDatabasesReply, GetDatabasesRequest, DatabaseSchema,
};

use super::Rcd;

pub async fn create_user_database(
    core: &Rcd,
    request: CreateUserDatabaseRequest,
) -> CreateUserDatabaseReply {
    let mut is_database_created = false;

    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;

    if auth_result.0 {
        let result = core.dbi().create_database(&db_name);
        if !result.is_err() {
            is_database_created = true;
        }
    }

    let create_db_result = CreateUserDatabaseReply {
        authentication_result: Some(auth_result.1),
        is_created: is_database_created,
        message: String::from(""),
    };

    return create_db_result;
}

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

pub async fn has_table(core: &Rcd, request: HasTableRequest) -> HasTableReply {
    let mut has_table = false;

    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let table_name = request.table_name;

    if auth_result.0 {
        has_table = core.dbi().has_table(&db_name, table_name.as_str())
    }

    let has_table_reply = HasTableReply {
        authentication_result: Some(auth_result.1),
        has_table: has_table,
    };

    return has_table_reply;
}

pub async fn generate_contract(
    core: &Rcd,
    request: GenerateContractRequest,
) -> GenerateContractReply {
    let mut is_successful = false;

    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let desc = request.description;
    let i_remote_delete_behavior = request.remote_delete_behavior;
    let host_name = request.host_name;

    let mut reply_message = String::from("");

    if auth_result.0 {
        let result = core.dbi().generate_contract(
            &db_name,
            &host_name,
            &desc,
            RemoteDeleteBehavior::from_u32(i_remote_delete_behavior),
        );

        match result {
            Ok(r) => is_successful = r,
            Err(e) => {
                is_successful = false;
                if let RcdGenerateContractError::NotAllTablesSet(msg) = e {
                    reply_message = msg;
                }
            }
        }
    };

    let generate_contract_reply = GenerateContractReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_successful,
        message: reply_message,
    };

    return generate_contract_reply;
}

pub async fn get_data_hash_at_participant(
    core: &Rcd,
    request: GetDataHashRequest,
) -> GetDataHashReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let table_name = request.table_name;
    let requested_row_id = request.row_id;
    let mut row_hash: u64 = 0;

    if auth_result.0 {
        row_hash = core
            .dbi()
            .get_data_hash_at_participant(&db_name, &table_name, requested_row_id);
    }

    let reply = GetDataHashReply {
        authentication_result: Some(auth_result.1),
        data_hash: row_hash,
    };

    return reply;
}

pub async fn change_updates_from_host_behavior(core: &Rcd, request: ChangeUpdatesFromHostBehaviorRequest) ->
ChangesUpdatesFromHostBehaviorReply {

    let auth_result = core.verify_login(request.authentication.unwrap());
    let db_name = request.database_name;
    let table_name = request.table_name;
    let behavior = request.behavior;
    let mut is_successful = false;

    if auth_result.0 {
        is_successful =
            core
                .dbi()
                .change_updates_from_host_behavior(&db_name, &table_name, behavior);
    }

    let reply = ChangesUpdatesFromHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_successful,
        message: String::from(""),
    };

    return reply;
}

pub async fn get_databases(core: &Rcd, request: GetDatabasesRequest) -> GetDatabasesReply {
    let mut db_result: Vec<DatabaseSchema> = Vec::new();

    let auth_result = core.verify_login(request.authentication.unwrap());

    if auth_result.0 {
        let db_names = core.dbi().get_database_names();
        for name in &db_names {
            let db_schema = core.dbi().get_database_schema(&name);
            db_result.push(db_schema);
        }
    }

    let result = GetDatabasesReply {
        authentication_result: Some(auth_result.1),
        databases: db_result,
    };

    return result;
}