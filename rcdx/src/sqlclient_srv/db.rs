use rcdproto::rcdp::{
    AcceptPendingUpdateReply, AcceptPendingUpdateRequest, AuthResult,
    ChangeDeletesFromHostBehaviorReply, ChangeDeletesFromHostBehaviorRequest,
    ChangeDeletesToHostBehaviorReply, ChangeDeletesToHostBehaviorRequest, ChangeHostStatusReply,
    ChangeHostStatusRequest, ChangeUpdatesFromHostBehaviorRequest,
    ChangeUpdatesToHostBehaviorReply, ChangeUpdatesToHostBehaviorRequest,
    ChangesUpdatesFromHostBehaviorReply, CreateUserDatabaseReply, CreateUserDatabaseRequest,
    EnableCoooperativeFeaturesReply, EnableCoooperativeFeaturesRequest, GenerateContractReply,
    GenerateContractRequest, GenerateHostInfoReply, GenerateHostInfoRequest, GetDataHashReply,
    GetDataHashRequest, GetPendingUpdatesReply, GetPendingUpdatesRequest, GetReadRowIdsReply,
    GetReadRowIdsRequest, HasTableReply, HasTableRequest, PendingUpdateStatement,
};

use crate::{
    rcd_enum::{RcdGenerateContractError, RemoteDeleteBehavior},
    remote_db_srv,
};

use super::SqlClientImpl;

pub async fn accept_pending_update_at_participant(
    request: AcceptPendingUpdateRequest,
    client: &SqlClientImpl,
) -> AcceptPendingUpdateReply {
    // 1 - we should execute the update statement
    // 2 - we should clear the row from the queue table
    // 3 - and then send the updated row_id and hash back to the host
    // update_row_data_hash_for_host on the data service

    let message = request.clone();
    let a = message.authentication.unwrap();
    let is_authenticated = client.verify_login(&a.user_name, &a.pw);

    let mut is_local_update_successful = false;
    let mut is_remote_update_successful = false;

    if is_authenticated {
        let db_name = &message.database_name;
        let table_name = &message.table_name;
        let row_id = message.row_id;

        let update_result = client
            .dbi()
            .accept_pending_update_at_participant(db_name, table_name, row_id);

        if update_result.is_successful {
            is_local_update_successful = true;

            let remote_host = client.dbi().get_cds_host_for_part_db(&db_name).unwrap();
            let own_host_info = client.dbi().rcd_get_host_info().clone();
            let own_db_addr_port = client.own_db_addr_port.clone();
            let hash = update_result.data_hash;

            let notify_is_successful = remote_db_srv::notify_host_of_updated_hash(
                &remote_host,
                &own_host_info,
                own_db_addr_port,
                db_name,
                table_name,
                row_id,
                hash.unwrap(),
            )
            .await;

            if notify_is_successful {
                is_remote_update_successful = true;
            }
        }
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let result = AcceptPendingUpdateReply {
        authentication_result: Some(auth_response),
        is_successful: is_local_update_successful && is_remote_update_successful,
    };

    return result;
}

pub async fn get_pending_updates_at_participant(
    request: GetPendingUpdatesRequest,
    client: &SqlClientImpl,
) -> GetPendingUpdatesReply {
    let message = request.clone();
    let a = message.authentication.unwrap();

    let db_name = &message.database_name;
    let table_name = &message.table_name;

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let mut pending_statements: Vec<PendingUpdateStatement> = Vec::new();

    if is_authenticated {
        pending_statements = client.dbi().get_pending_updates(db_name, table_name);
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let result = GetPendingUpdatesReply {
        authentication_result: Some(auth_response),
        pending_statements: pending_statements,
    };

    return result;
}

pub async fn change_host_status(
    request: ChangeHostStatusRequest,
    client: &SqlClientImpl,
) -> ChangeHostStatusReply {
    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();
    let host_name = message.host_alias.clone();
    let host_id = message.host_id.clone();
    let status = message.status;

    let mut name_result = false;
    let mut id_result = false;

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);

    if is_authenticated {
        name_result = client.dbi().change_host_status_by_name(&host_name, status);

        if !name_result {
            id_result = client.dbi().change_host_status_by_id(&host_id, status);
        }
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let result = ChangeHostStatusReply {
        authentication_result: Some(auth_response),
        is_successful: name_result || id_result,
        status,
    };

    return result;
}

pub async fn generate_host_info(
    request: GenerateHostInfoRequest,
    client: &SqlClientImpl,
) -> GenerateHostInfoReply {
    let mut is_generate_successful = false;

    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();
    let host_name = message.host_name.clone();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);

    if is_authenticated {
        client.dbi().rcd_generate_host_info(&host_name);
        is_generate_successful = true;
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let generate_host_info_result = GenerateHostInfoReply {
        authentication_result: Some(auth_response),
        is_successful: is_generate_successful,
    };

    return generate_host_info_result;
}

pub async fn create_user_database(
    request: CreateUserDatabaseRequest,
    client: &SqlClientImpl,
) -> CreateUserDatabaseReply {
    let mut is_database_created = false;

    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;

    if is_authenticated {
        let result = client.dbi().create_database(&db_name);
        if !result.is_err() {
            is_database_created = true;
        }
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let create_db_result = CreateUserDatabaseReply {
        authentication_result: Some(auth_response),
        is_created: is_database_created,
        message: String::from(""),
    };

    return create_db_result;
}

pub async fn has_table(request: HasTableRequest, client: &SqlClientImpl) -> HasTableReply {
    let mut has_table = false;

    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let table_name = message.table_name;

    if is_authenticated {
        has_table = client.dbi().has_table(&db_name, table_name.as_str())
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let has_table_reply = HasTableReply {
        authentication_result: Some(auth_response),
        has_table: has_table,
    };

    return has_table_reply;
}

pub async fn generate_contract(
    request: GenerateContractRequest,
    client: &SqlClientImpl,
) -> GenerateContractReply {
    let mut is_successful = false;

    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();
    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let desc = message.description;
    let i_remote_delete_behavior = message.remote_delete_behavior;
    let host_name = message.host_name;

    let mut reply_message = String::from("");

    if is_authenticated {
        let result = client.dbi().generate_contract(
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

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let generate_contract_reply = GenerateContractReply {
        authentication_result: Some(auth_response),
        is_successful: is_successful,
        message: reply_message,
    };

    return generate_contract_reply;
}

pub async fn enable_coooperative_features(
    request: EnableCoooperativeFeaturesRequest,
    client: &SqlClientImpl,
) -> EnableCoooperativeFeaturesReply {
    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;

    if is_authenticated {
        client.dbi().enable_coooperative_features(&db_name);
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let enable_cooperative_features_reply = EnableCoooperativeFeaturesReply {
        authentication_result: Some(auth_response),
        is_successful: true,
        message: String::from(""),
    };

    return enable_cooperative_features_reply;
}

pub async fn change_updates_from_host_behavior(
    request: ChangeUpdatesFromHostBehaviorRequest,
    client: &SqlClientImpl,
) -> ChangesUpdatesFromHostBehaviorReply {
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let table_name = message.table_name;
    let behavior = message.behavior;
    let mut is_successful = false;

    if is_authenticated {
        is_successful =
            client
                .dbi()
                .change_updates_from_host_behavior(&db_name, &table_name, behavior);
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let reply = ChangesUpdatesFromHostBehaviorReply {
        authentication_result: Some(auth_response),
        is_successful: is_successful,
        message: String::from(""),
    };

    return reply;
}

#[allow(dead_code, unused_variables)]
pub async fn change_deletes_from_host_behavior(
    request: ChangeDeletesFromHostBehaviorRequest,
    client: &SqlClientImpl,
) -> ChangeDeletesFromHostBehaviorReply {
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let table_name = message.table_name;
    let behavior = message.behavior;
    let mut is_successful = false;

    if is_authenticated {
        is_successful =
            client
                .dbi()
                .change_deletes_from_host_behavior(&db_name, &table_name, behavior);
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let reply = ChangeDeletesFromHostBehaviorReply {
        authentication_result: Some(auth_response),
        is_successful: is_successful,
        message: String::from(""),
    };

    return reply;
}

#[allow(dead_code, unused_variables)]
pub async fn change_updates_to_host_behavior(
    request: ChangeUpdatesToHostBehaviorRequest,
    client: &SqlClientImpl,
) -> ChangeUpdatesToHostBehaviorReply {
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let table_name = message.table_name;
    let behavior = message.behavior;
    let mut is_successful = false;

    if is_authenticated {
        is_successful =
            client
                .dbi()
                .change_updates_to_host_behavior(&db_name, &table_name, behavior);
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let reply = ChangeUpdatesToHostBehaviorReply {
        authentication_result: Some(auth_response),
        is_successful: is_successful,
        message: String::from(""),
    };

    return reply;
}

#[allow(dead_code, unused_variables)]
pub async fn change_deletes_to_host_behavior(
    request: ChangeDeletesToHostBehaviorRequest,
    client: &SqlClientImpl,
) -> ChangeDeletesToHostBehaviorReply {
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let table_name = message.table_name;
    let behavior = message.behavior;
    let mut is_successful = false;

    if is_authenticated {
        is_successful =
            client
                .dbi()
                .change_deletes_to_host_behavior(&db_name, &table_name, behavior);
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let reply = ChangeDeletesToHostBehaviorReply {
        authentication_result: Some(auth_response),
        is_successful: is_successful,
        message: String::from(""),
    };

    return reply;
}

pub async fn read_row_id_at_participant(
    request: GetReadRowIdsRequest,
    client: &SqlClientImpl,
) -> GetReadRowIdsReply {
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let table_name = message.table_name;
    let where_clause = message.where_clause;
    let mut row_id = 0;

    let mut row_ids: Vec<u32> = Vec::new();

    if is_authenticated {
        row_id = client
            .dbi()
            .read_row_id_from_part_db(&db_name, &table_name, &where_clause);
    }

    if row_id > 0 {
        row_ids.push(row_id);
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let reply = GetReadRowIdsReply {
        authentication_result: Some(auth_response),
        row_ids: row_ids,
    };

    return reply;
}

pub async fn get_data_hash_at_host(
    request: GetDataHashRequest,
    client: &SqlClientImpl,
) -> GetDataHashReply {
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let table_name = message.table_name;
    let requested_row_id = message.row_id;
    let mut row_hash: u64 = 0;

    if is_authenticated {
        row_hash = client
            .dbi()
            .get_data_hash_at_host(&db_name, &table_name, requested_row_id);
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let reply = GetDataHashReply {
        authentication_result: Some(auth_response),
        data_hash: row_hash,
    };

    return reply;
}

pub async fn get_data_hash_at_participant(
    request: GetDataHashRequest,
    client: &SqlClientImpl,
) -> GetDataHashReply {
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let table_name = message.table_name;
    let requested_row_id = message.row_id;
    let mut row_hash: u64 = 0;

    if is_authenticated {
        row_hash =
            client
                .dbi()
                .get_data_hash_at_participant(&db_name, &table_name, requested_row_id);
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let reply = GetDataHashReply {
        authentication_result: Some(auth_response),
        data_hash: row_hash,
    };

    return reply;
}
