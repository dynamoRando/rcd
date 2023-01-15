use rcd_common::{
    host_info::HostInfo,
    rcd_enum::{
        DeletesFromHostBehavior, HostStatus, RcdGenerateContractError, RemoteDeleteBehavior,
        UpdatesFromHostBehavior, 
    },
};
use rcd_enum::updates_to_host_behavior::UpdatesToHostBehavior;
use rcd_enum::deletes_to_host_behavior::DeletesToHostBehavior;
use rcd_enum::partial_data_result_action::PartialDataResultAction;
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
    Host, HostInfoReply, HostInfoStatus, ParticipantStatus, PendingStatement,
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

pub async fn get_cooperative_hosts(
    core: &Rcd,
    request: GetCooperativeHostsRequest,
) -> GetCooperativeHostsReply {
    let auth_result = core.verify_login(request.authentication.unwrap());
    let mut hosts: Vec<HostInfoStatus> = Vec::new();

    if auth_result.0 {
        let result = core.dbi().get_cooperative_hosts();

        if result.len() > 0 {
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

    let result = GetCooperativeHostsReply {
        authentication_result: Some(auth_result.1),
        hosts,
    };

    return result;
}

pub async fn get_host_info(core: &Rcd, request: AuthRequest) -> HostInfoReply {
    let auth_result = core.verify_login(request);
    let mut host_info: Option<HostInfo> = None;

    if auth_result.0 {
        host_info = Some(core.dbi().rcd_get_host_info().clone());
    }

    let host: Host;

    if host_info.is_some() {
        host = Host {
            host_guid: host_info.as_ref().unwrap().id.clone(),
            host_name: host_info.as_ref().unwrap().name.clone(),
            ip4_address: "".to_string(),
            ip6_address: "".to_string(),
            database_port_number: 0,
            token: Vec::new(),
            http_addr: "".to_string(),
            http_port: 0,
        };
    } else {
        host = Host {
            host_guid: "".to_string(),
            host_name: "".to_string(),
            ip4_address: "".to_string(),
            ip6_address: "".to_string(),
            database_port_number: 0,
            token: Vec::new(),
            http_addr: "".to_string(),
            http_port: 0,
        }
    }

    let result = HostInfoReply {
        authentication_result: Some(auth_result.1),
        host_info: Some(host),
    };

    return result;
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

    let reply = ChangesUpdatesFromHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_successful,
        message: String::from(""),
    };

    return reply;
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

    let reply = GetUpdatesToHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        behavior: behavior,
    };

    return reply;
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

    let reply = GetUpdatesFromHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        behavior: behavior,
    };

    return reply;
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

    return GetActiveContractReply {
        authentication_result: Some(auth_result.1),
        contract: None,
    };
}

pub async fn get_participants(core: &Rcd, request: GetParticipantsRequest) -> GetParticipantsReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let mut participants_result: Vec<ParticipantStatus> = Vec::new();

    if auth_result.0 {
        let participants = core
            .dbi()
            .get_participants_for_database(&request.database_name);
        participants_result = participants;
    }

    let result = GetParticipantsReply {
        authentication_result: Some(auth_result.1),
        participants: participants_result,
    };

    return result;
}

pub async fn get_databases(core: &Rcd, request: GetDatabasesRequest) -> GetDatabasesReply {
    let mut db_result: Vec<DatabaseSchema> = Vec::new();

    let auth_result = core.verify_login(request.authentication.unwrap());

    if auth_result.0 {
        let db_names = core.dbi().get_database_names();
        for name in &db_names {
            let db_schema = core.dbi().get_database_schema(&name);
            println!("{:?}", db_schema);
            db_result.push(db_schema);
        }
    }

    let result = GetDatabasesReply {
        authentication_result: Some(auth_result.1),
        databases: db_result,
    };

    return result;
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

    let reply = GetDataHashReply {
        authentication_result: Some(auth_result.1),
        data_hash: row_hash,
    };

    return reply;
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

    let reply = ChangeDeletesFromHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_successful,
        message: String::from(""),
    };

    return reply;
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

    let reply = GetDeletesFromHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        behavior: behavior,
    };

    return reply;
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

    let reply = GetDeletesToHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        behavior: behavior,
    };

    return reply;
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

    let reply = ChangeDeletesToHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_successful,
        message: String::from(""),
    };

    return reply;
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

    let reply = ChangeUpdatesToHostBehaviorReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_successful,
        message: String::from(""),
    };

    return reply;
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

    let reply = GetReadRowIdsReply {
        authentication_result: Some(auth_result.1),
        row_ids: row_ids,
    };

    return reply;
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

    let enable_cooperative_features_reply = EnableCoooperativeFeaturesReply {
        authentication_result: Some(auth_result.1),
        is_successful: true,
        message: String::from(""),
    };

    return enable_cooperative_features_reply;
}
