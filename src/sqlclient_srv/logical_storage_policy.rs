use crate::{
    cdata::{AuthResult, SetLogicalStoragePolicyReply, SetLogicalStoragePolicyRequest},
    rcd_enum::LogicalStoragePolicy,
};

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
