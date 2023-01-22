use rcd_enum::logical_storage_policy::LogicalStoragePolicy;
use rcdproto::rcdp::{
    GetLogicalStoragePolicyReply, GetLogicalStoragePolicyRequest, SetLogicalStoragePolicyReply,
    SetLogicalStoragePolicyRequest,
};

use super::Rcd;

pub async fn set_logical_storage_policy(
    core: &Rcd,
    request: SetLogicalStoragePolicyRequest,
) -> SetLogicalStoragePolicyReply {
    let mut policy_is_set = false;

    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let policy_num = request.policy_mode;
    let policy = LogicalStoragePolicy::from_i64(policy_num as i64);
    let table_name = request.table_name;

    if auth_result.0 {
        policy_is_set = core
            .dbi()
            .set_logical_storage_policy(&db_name, table_name.as_str(), policy)
            .unwrap();
    }

    SetLogicalStoragePolicyReply {
        authentication_result: Some(auth_result.1),
        is_successful: policy_is_set,
        message: String::from(""),
    }
}

pub async fn get_logical_storage_policy(
    core: &Rcd,
    request: GetLogicalStoragePolicyRequest,
) -> GetLogicalStoragePolicyReply {
    let mut policy = LogicalStoragePolicy::None;

    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let table_name = request.table_name;

    if auth_result.0 {
        let i_policy = core
            .dbi()
            .get_logical_storage_policy(&db_name, &table_name)
            .unwrap();

        policy = LogicalStoragePolicy::from_i64(i_policy as i64);
    }

    GetLogicalStoragePolicyReply {
        authentication_result: Some(auth_result.1),
        policy_mode: LogicalStoragePolicy::to_u32(policy),
    }
}
