use rcd_common::rcd_enum::{RcdGenerateContractError, RemoteDeleteBehavior};
use rcdproto::rcdp::{
    AuthResult, ChangeDeletesFromHostBehaviorReply, ChangeDeletesFromHostBehaviorRequest,
    ChangeDeletesToHostBehaviorReply, ChangeDeletesToHostBehaviorRequest,
    ChangeUpdatesFromHostBehaviorRequest, ChangeUpdatesToHostBehaviorReply,
    ChangeUpdatesToHostBehaviorRequest, ChangesUpdatesFromHostBehaviorReply,
    EnableCoooperativeFeaturesReply, EnableCoooperativeFeaturesRequest, GenerateContractReply,
    GenerateContractRequest, GetDataHashReply, GetDataHashRequest, GetReadRowIdsReply,
    GetReadRowIdsRequest, HasTableReply, HasTableRequest,
};

use super::SqlClientImpl;

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
