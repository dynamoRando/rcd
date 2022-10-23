use rcdproto::rcdp::{
    AuthResult, GetLogicalStoragePolicyReply, GetLogicalStoragePolicyRequest,
    SetLogicalStoragePolicyReply, SetLogicalStoragePolicyRequest,
};

use rcd_common::rcd_enum::LogicalStoragePolicy;

use super::SqlClientImpl;

pub async fn set_logical_storage_policy(
    request: SetLogicalStoragePolicyRequest,
    client: &SqlClientImpl,
) -> SetLogicalStoragePolicyReply {
    let mut policy_is_set = false;

    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let policy_num = message.policy_mode;
    let policy = LogicalStoragePolicy::from_i64(policy_num as i64);
    let table_name = message.table_name;

    if is_authenticated {
        policy_is_set = client
            .dbi()
            .set_logical_storage_policy(&db_name, table_name.as_str(), policy)
            .unwrap();
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let set_policy_reply = SetLogicalStoragePolicyReply {
        authentication_result: Some(auth_response),
        is_successful: policy_is_set,
        message: String::from(""),
    };

    return set_policy_reply;
}

pub async fn get_logical_storage_policy(
    request: GetLogicalStoragePolicyRequest,
    client: &SqlClientImpl,
) -> GetLogicalStoragePolicyReply {
    let mut policy = LogicalStoragePolicy::None;

    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let table_name = message.table_name;

    if is_authenticated {
        let i_policy = client
            .dbi()
            .get_logical_storage_policy(&db_name, &table_name)
            .unwrap();

        policy = LogicalStoragePolicy::from_i64(i_policy as i64);
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let get_policy_reply = GetLogicalStoragePolicyReply {
        authentication_result: Some(auth_response),
        policy_mode: LogicalStoragePolicy::to_u32(policy),
    };

    return get_policy_reply;
}
