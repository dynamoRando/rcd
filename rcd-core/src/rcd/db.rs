use log::{warn, trace};
use rcd_common::{data_info::DataInfo, host_info::HostInfo};

use super::Rcd;
use rcd_enum::{
    deletes_from_host_behavior::DeletesFromHostBehavior,
    deletes_to_host_behavior::DeletesToHostBehavior, host_status::HostStatus,
    partial_data_result_action::PartialDataResultAction,
    rcd_generate_contract_error::RcdGenerateContractError,
    remote_delete_behavior::RemoteDeleteBehavior,
    updates_from_host_behavior::UpdatesFromHostBehavior,
    updates_to_host_behavior::UpdatesToHostBehavior,
};
use rcdproto::rcdp::{
    AcceptPendingActionReply, AcceptPendingActionRequest, AuthRequest,
    ChangeDeletesFromHostBehaviorReply, ChangeDeletesFromHostBehaviorRequest,
    ChangeDeletesToHostBehaviorReply, ChangeDeletesToHostBehaviorRequest, ChangeHostStatusReply,
    ChangeHostStatusRequest, ChangeUpdatesFromHostBehaviorRequest,
    ChangeUpdatesToHostBehaviorReply, ChangeUpdatesToHostBehaviorRequest,
    ChangesUpdatesFromHostBehaviorReply, CreateUserDatabaseReply, CreateUserDatabaseRequest,
    DatabaseSchema, EnableCoooperativeFeaturesReply, EnableCoooperativeFeaturesRequest,
    GenerateContractReply, GenerateContractRequest, GenerateHostInfoReply, GenerateHostInfoRequest,
    GetActiveContractReply, GetActiveContractRequest, GetCooperativeHostsReply,
    GetCooperativeHostsRequest, GetDataHashReply, GetDataHashRequest, GetDatabasesReply,
    GetDatabasesRequest, GetDeletesFromHostBehaviorReply, GetDeletesFromHostBehaviorRequest,
    GetDeletesToHostBehaviorReply, GetDeletesToHostBehaviorRequest, GetParticipantsReply,
    GetParticipantsRequest, GetPendingActionsReply, GetPendingActionsRequest, GetReadRowIdsReply,
    GetReadRowIdsRequest, GetUpdatesFromHostBehaviorReply, GetUpdatesFromHostBehaviorRequest,
    GetUpdatesToHostBehaviorReply, GetUpdatesToHostBehaviorRequest, HasTableReply, HasTableRequest,
    Host, HostInfoReply, HostInfoStatus, ParticipantStatus, PendingStatement, RcdError,
};

pub async fn create_user_database(
    core: &Rcd,
    request: CreateUserDatabaseRequest,
) -> CreateUserDatabaseReply {
    let mut is_database_created = false;

    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;

    if auth_result.0 {
        let result = core.dbi().create_database(&db_name);
        if result.is_ok() {
            is_database_created = true;
        }
    }

    CreateUserDatabaseReply {
        authentication_result: Some(auth_result.1),
        is_created: is_database_created,
        message: String::from(""),
    }
}

pub async fn get_cooperative_hosts(
    core: &Rcd,
    request: GetCooperativeHostsRequest,
) -> GetCooperativeHostsReply {
    let auth_result = core.verify_login(request.authentication.unwrap());
    let mut hosts: Vec<HostInfoStatus> = Vec::new();

    if auth_result.0 {
        let result = core.dbi().get_cooperative_hosts();

        if !result.is_empty() {
            for host in &result {
                let h = Host {
                    host_guid: host.host_id.clone(),
                    host_name: host.host_name.clone(),
                    ip4_address: host.ip4.clone(),
                    ip6_address: host.ip6.clone(),
                    database_port_number: host.port,
                    token: Vec::new(),
                    http_addr: host.http_addr.clone(),
                    http_port: host.port,
                };

                let i = HostInfoStatus {
                    host: Some(h),
                    last_communcation_utc: host.last_comm_utc.clone(),
                    status: HostStatus::to_u32(host.status),
                };

                hosts.push(i);
            }
        }
    }

    GetCooperativeHostsReply {
        authentication_result: Some(auth_result.1),
        hosts,
    }
}

pub async fn get_host_info(core: &Rcd, request: AuthRequest) -> HostInfoReply {
    let auth_result = core.verify_login(request);
    let mut host_info: Option<HostInfo> = None;

    if auth_result.0 {
        host_info = Some(core.dbi().rcd_get_host_info());
    }

    let host = if host_info.is_some() {
        Host {
            host_guid: host_info.as_ref().unwrap().id.clone(),
            host_name: host_info.as_ref().unwrap().name.clone(),
            ip4_address: "".to_string(),
            ip6_address: "".to_string(),
            database_port_number: 0,
            token: Vec::new(),
            http_addr: "".to_string(),
            http_port: 0,
        }
    } else {
        Host {
            host_guid: "".to_string(),
            host_name: "".to_string(),
            ip4_address: "".to_string(),
            ip6_address: "".to_string(),
            database_port_number: 0,
            token: Vec::new(),
            http_addr: "".to_string(),
            http_port: 0,
        }
    };

    HostInfoReply {
        authentication_result: Some(auth_result.1),
        host_info: Some(host),
    }
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

    GenerateHostInfoReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_generate_successful,
    }
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

    ChangeHostStatusReply {
        authentication_result: Some(auth_result.1),
        is_successful: name_result || id_result,
        status,
    }
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
        pending_statements = core.dbi().get_pending_actions(db_name, table_name, action);
    }

    GetPendingActionsReply {
        authentication_result: Some(auth_result.1),
        pending_statements,
    }
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

        trace!("{data_result:?}");
        trace!("is_local_update_successful: {is_local_update_successful}");

        if data_result.is_successful {
            is_local_update_successful = true;

            let remote_host = core.dbi().get_cds_host_for_part_db(db_name).unwrap();
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

            let data_info = DataInfo {
                db_name: db_name.to_string(),
                table_name: table_name.to_string(),
                row_id,
                hash,
                is_deleted,
            };

            let notify_is_successful = core
                .remote()
                .notify_host_of_updated_hash(&remote_host, &own_host_info, &data_info)
                .await;

            trace!("notify_is_successful: {notify_is_successful}");

            if notify_is_successful {
                is_remote_update_successful = true;
            }
        }
    } else {
        trace!("not authenticated");
    }

    AcceptPendingActionReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_local_update_successful && is_remote_update_successful,
    }
}

pub async fn has_table(core: &Rcd, request: HasTableRequest) -> HasTableReply {
    let mut has_table = false;

    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let table_name = request.table_name;

    if auth_result.0 {
        has_table = core.dbi().has_table(&db_name, table_name.as_str())
    }

    HasTableReply {
        authentication_result: Some(auth_result.1),
        has_table,
    }
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

    GenerateContractReply {
        authentication_result: Some(auth_result.1),
        is_successful,
        message: reply_message,
    }
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

    GetDataHashReply {
        authentication_result: Some(auth_result.1),
        data_hash: row_hash,
    }
}

pub async fn change_updates_from_host_behavior(
    core: &Rcd,
    request: ChangeUpdatesFromHostBehaviorRequest,
) -> ChangesUpdatesFromHostBehaviorReply {
    let auth_result = core.verify_login(request.authentication.unwrap());
    let db_name = request.database_name;
    let table_name = request.table_name;
    let behavior = request.behavior;
    let mut is_successful = false;

    if auth_result.0 {
        is_successful =
            core.dbi()
                .change_updates_from_host_behavior(&db_name, &table_name, behavior);
    }

    ChangesUpdatesFromHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        is_successful,
        message: String::from(""),
    }
}

pub async fn get_updates_to_host_behavior(
    core: &Rcd,
    request: GetUpdatesToHostBehaviorRequest,
) -> GetUpdatesToHostBehaviorReply {
    let auth_result = core.verify_login(request.authentication.unwrap());
    let db_name = request.database_name;
    let table_name = request.table_name;
    let mut behavior = 0;

    if auth_result.0 {
        let x = core
            .dbi()
            .get_updates_to_host_behavior(&db_name, &table_name);
        behavior = UpdatesToHostBehavior::to_u32(x);
    }

    GetUpdatesToHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        behavior,
    }
}

pub async fn get_updates_from_host_behavior(
    core: &Rcd,
    request: GetUpdatesFromHostBehaviorRequest,
) -> GetUpdatesFromHostBehaviorReply {
    let auth_result = core.verify_login(request.authentication.unwrap());
    let db_name = request.database_name;
    let table_name = request.table_name;
    let mut behavior = 0;

    if auth_result.0 {
        let x = core
            .dbi()
            .get_updates_from_host_behavior(&db_name, &table_name);
        behavior = UpdatesFromHostBehavior::to_u32(x);
    }

    GetUpdatesFromHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        behavior,
    }
}

pub async fn get_active_contract(
    core: &Rcd,
    request: GetActiveContractRequest,
) -> GetActiveContractReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    if auth_result.0 {
        let contract = core.dbi().get_active_contract_proto(&request.database_name);

        return GetActiveContractReply {
            authentication_result: Some(auth_result.1),
            contract: Some(contract),
        };
    }

    GetActiveContractReply {
        authentication_result: Some(auth_result.1),
        contract: None,
    }
}

pub async fn get_participants(core: &Rcd, request: GetParticipantsRequest) -> GetParticipantsReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let mut participants_result: Vec<ParticipantStatus> = Vec::new();
    let mut is_error: bool = false;
    let mut error: Option<RcdError> = None;

    if auth_result.0 {
        let result = core
            .dbi()
            .get_participants_for_database(&request.database_name);
        match result {
            Ok(participants) => {
                participants_result = participants;
            }
            Err(e) => {
                let message = format!(
                    "{} - {}",
                    e, "Are cooperative functions enabled on this database?"
                );
                warn!("{}", message);
                is_error = true;
                error = Some(RcdError {
                    number: 0,
                    message: e.to_string(),
                    help: "Are cooperative functions enabled on this database?".to_string(),
                })
            }
        };
    }

    GetParticipantsReply {
        authentication_result: Some(auth_result.1),
        participants: participants_result,
        is_error,
        error,
    }
}

pub async fn get_databases(core: &Rcd, request: GetDatabasesRequest) -> GetDatabasesReply {
    let mut db_result: Vec<DatabaseSchema> = Vec::new();

    let auth_result = core.verify_login(request.authentication.unwrap());

    if auth_result.0 {
        let db_names = core.dbi().get_database_names();
        for name in &db_names {
            let db_schema = core.dbi().get_database_schema(name);
            trace!("{db_schema:?}");
            db_result.push(db_schema);
        }
    }

    GetDatabasesReply {
        authentication_result: Some(auth_result.1),
        databases: db_result,
    }
}

pub async fn get_data_hash_at_host(core: &Rcd, request: GetDataHashRequest) -> GetDataHashReply {
    let auth_result = core.verify_login(request.authentication.unwrap());
    let db_name = request.database_name;
    let table_name = request.table_name;
    let requested_row_id = request.row_id;
    let mut row_hash: u64 = 0;

    if auth_result.0 {
        row_hash = core
            .dbi()
            .get_data_hash_at_host(&db_name, &table_name, requested_row_id);
    }

    GetDataHashReply {
        authentication_result: Some(auth_result.1),
        data_hash: row_hash,
    }
}

pub async fn change_deletes_from_host_behavior(
    core: &Rcd,
    request: ChangeDeletesFromHostBehaviorRequest,
) -> ChangeDeletesFromHostBehaviorReply {
    let auth_result = core.verify_login(request.authentication.unwrap());
    let db_name = request.database_name;
    let table_name = request.table_name;
    let behavior = request.behavior;
    let mut is_successful = false;

    if auth_result.0 {
        is_successful =
            core.dbi()
                .change_deletes_from_host_behavior(&db_name, &table_name, behavior);
    }

    ChangeDeletesFromHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        is_successful,
        message: String::from(""),
    }
}

pub async fn get_deletes_from_host_behavior(
    core: &Rcd,
    request: GetDeletesFromHostBehaviorRequest,
) -> GetDeletesFromHostBehaviorReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let table_name = request.table_name;

    let mut behavior = 0;

    if auth_result.0 {
        let x = core
            .dbi()
            .get_deletes_from_host_behavior(&db_name, &table_name);
        behavior = DeletesFromHostBehavior::to_u32(x);
    }

    GetDeletesFromHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        behavior,
    }
}

pub async fn get_deletes_to_host_behavior(
    core: &Rcd,
    request: GetDeletesToHostBehaviorRequest,
) -> GetDeletesToHostBehaviorReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let table_name = request.table_name;

    let mut behavior = 0;

    if auth_result.0 {
        let x = core
            .dbi()
            .get_deletes_to_host_behavior(&db_name, &table_name);
        behavior = DeletesToHostBehavior::to_u32(x);
    }

    GetDeletesToHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        behavior,
    }
}

pub async fn change_deletes_to_host_behavior(
    core: &Rcd,
    request: ChangeDeletesToHostBehaviorRequest,
) -> ChangeDeletesToHostBehaviorReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let table_name = request.table_name;
    let behavior = request.behavior;
    let mut is_successful = false;

    if auth_result.0 {
        is_successful = core
            .dbi()
            .change_deletes_to_host_behavior(&db_name, &table_name, behavior);
    }

    ChangeDeletesToHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        is_successful,
        message: String::from(""),
    }
}

pub async fn change_updates_to_host_behavior(
    core: &Rcd,
    request: ChangeUpdatesToHostBehaviorRequest,
) -> ChangeUpdatesToHostBehaviorReply {
    let auth_result = core.verify_login(request.authentication.unwrap());
    let db_name = request.database_name;
    let table_name = request.table_name;
    let behavior = request.behavior;
    let mut is_successful = false;

    if auth_result.0 {
        is_successful = core
            .dbi()
            .change_updates_to_host_behavior(&db_name, &table_name, behavior);
    }

    ChangeUpdatesToHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        is_successful,
        message: String::from(""),
    }
}

pub async fn read_row_id_at_participant(
    core: &Rcd,
    request: GetReadRowIdsRequest,
) -> GetReadRowIdsReply {
    let auth_result = core.verify_login(request.authentication.unwrap());
    let db_name = request.database_name;
    let table_name = request.table_name;
    let where_clause = request.where_clause;
    let mut row_id = 0;

    let mut row_ids: Vec<u32> = Vec::new();

    if auth_result.0 {
        row_id = core
            .dbi()
            .read_row_id_from_part_db(&db_name, &table_name, &where_clause);
    }

    if row_id > 0 {
        row_ids.push(row_id);
    }

    GetReadRowIdsReply {
        authentication_result: Some(auth_result.1),
        row_ids,
    }
}

pub async fn enable_coooperative_features(
    core: &Rcd,
    request: EnableCoooperativeFeaturesRequest,
) -> EnableCoooperativeFeaturesReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;

    if auth_result.0 {
        core.dbi().enable_coooperative_features(&db_name);
    }

    EnableCoooperativeFeaturesReply {
        authentication_result: Some(auth_result.1),
        is_successful: true,
        message: String::from(""),
    }
}
