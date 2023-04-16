use super::Rcd;
use ::rcd_enum::rcd_database_type::RcdDatabaseType;
use conv::UnwrapOk;
use conv::ValueFrom;
use tracing::{error, info, trace, warn};
use rcd_common::data_info::DataInfo;
use rcd_enum::deletes_to_host_behavior::DeletesToHostBehavior;
use rcd_enum::dml_type::DmlType;
use rcd_enum::partial_data_status::PartialDataStatus;
use rcd_enum::updates_to_host_behavior::UpdatesToHostBehavior;
use rcd_query::query_parser::determine_dml_type;
use rcd_query::query_parser::get_table_name;
use rcdproto::rcdp::ExecuteCooperativeWriteReply;
use rcdproto::rcdp::ExecuteCooperativeWriteRequest;
use rcdproto::rcdp::ExecuteWriteReply;
use rcdproto::rcdp::ExecuteWriteRequest;
use rcdproto::rcdp::RcdError;
use rcdproto::rcdp::{ExecuteReadReply, ExecuteReadRequest, StatementResultset};

pub async fn execute_read_at_host(core: &Rcd, request: ExecuteReadRequest) -> ExecuteReadReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let sql = request.sql_statement;
    let mut is_error = false;
    let mut error: Option<RcdError> = None;

    let mut result_table = Vec::new();

    let mut statement_result_set = StatementResultset {
        is_error: true,
        result_message: String::from(""),
        number_of_rows_affected: 0,
        rows: Vec::new(),
        execution_error_message: String::from(""),
    };

    if auth_result.0 {
        let result = core.dbi().has_cooperative_tables(&db_name, &sql);
        match result {
            Ok(has_cooperative_tables) => {
                if has_cooperative_tables {
                    trace!(
                        "execute_read_at_host: found cooperative tables for: {} w/ sql {}",
                        &db_name,
                        &sql
                    );

                    let cooperative_tables = core.dbi().get_cooperative_tables(&db_name, &sql);

                    trace!("execute_read_at_host: cooperative_tables: {cooperative_tables:?}");

                    for ct in &cooperative_tables {
                        let participants_for_table =
                            core.dbi().get_participants_for_table(&db_name, ct.as_str());

                        trace!("execute_read_at_host: participants_for_table: {participants_for_table:?}");

                        if participants_for_table.is_empty() {
                            warn!("WARN: execute_read_at_host: no participants found for table: {ct:?}");
                        }

                        for participant in &participants_for_table {
                            trace!("execute_read_at_host: participant: {participant:?}");

                            // we would need to get rows for that table from the participant
                            let host_info =
                                core.dbi().rcd_get_host_info().expect("no host info is set");
                            let remote_data_result = core
                                .remote()
                                .get_row_from_participant(participant.clone(), host_info)
                                .await;

                            trace!(
                                "execute_read_at_host: remote_data_result: {remote_data_result:?}"
                            );

                            if !remote_data_result.is_successful {
                                warn!("remote data result failed: {remote_data_result:?}");
                            }

                            let data_hash_for_row =
                                remote_data_result.row.as_ref().unwrap().hash.clone();

                            let saved_hash_for_row =
                                participant.row_data.first().unwrap().1.clone();

                            if data_hash_for_row == saved_hash_for_row {
                                let row = remote_data_result.row.as_ref().unwrap().clone();
                                result_table.push(row);
                                statement_result_set.is_error = false;
                            } else {
                                let row = remote_data_result.row.as_ref().unwrap().clone();
                                result_table.push(row);
                                statement_result_set.result_message = String::from(
                                    "warning: data hashes for host and participant rows do not match!",
                                );
                            }
                        }
                    }

                    statement_result_set.rows = result_table;
                } else {
                    let query_result = core.dbi().execute_read_at_host(&db_name, &sql);

                    match query_result {
                        Ok(result) => {
                            let result_rows = result.to_cdata_rows();
                            statement_result_set.number_of_rows_affected =
                                u64::value_from(result_rows.len()).unwrap_ok();
                            statement_result_set.rows = result_rows;
                            statement_result_set.is_error = false;
                        }
                        Err(e) => {
                            error!("execute_read_at_host: {}", &e.to_string());
                            statement_result_set.execution_error_message = e.to_string();
                        }
                    }
                }
            }
            Err(e) => {
                error!("execute_read_at_host: {e:?}");
                is_error = true;
                error = Some(RcdError {
                    number: 0,
                    message: e.to_string(),
                    help: String::from(""),
                });
            }
        }
    }

    let statement_results = vec![statement_result_set];

    ExecuteReadReply {
        authentication_result: Some(auth_result.1),
        total_resultsets: 1,
        results: statement_results,
        is_error,
        error,
    }
}

pub async fn execute_read_at_participant(
    core: &Rcd,
    request: ExecuteReadRequest,
) -> ExecuteReadReply {
    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let sql = request.sql_statement;
    let mut is_error = false;
    let mut error: Option<RcdError> = None;

    let mut statement_result_set = StatementResultset {
        is_error: true,
        result_message: String::from(""),
        number_of_rows_affected: 0,
        rows: Vec::new(),
        execution_error_message: String::from(""),
    };

    if auth_result.0 {
        let query_result = core.dbi().execute_read_at_participant(&db_name, &sql);

        match query_result {
            Ok(_) => {
                let result_rows = query_result.unwrap().to_cdata_rows();
                statement_result_set.number_of_rows_affected =
                    u64::value_from(result_rows.len()).unwrap_ok();
                statement_result_set.rows = result_rows;
                statement_result_set.is_error = false;
            }
            Err(e) => {
                statement_result_set.execution_error_message = e.to_string();
                is_error = true;
                error = Some(RcdError {
                    number: 0,
                    message: e.to_string(),
                    help: String::from(""),
                });
            }
        }
    }

    let statement_results = vec![statement_result_set];

    ExecuteReadReply {
        authentication_result: Some(auth_result.1),
        total_resultsets: 1,
        results: statement_results,
        is_error,
        error,
    }
}

pub async fn execute_write_at_participant(
    core: &Rcd,
    request: ExecuteWriteRequest,
) -> ExecuteWriteReply {
    let mut rows_affected: u32 = 0;
    let mut is_overall_successful = false;

    let auth_result = core.verify_login(request.authentication.unwrap());

    let db_name = request.database_name;
    let statement = request.sql_statement;
    let where_clause = request.where_clause;

    if auth_result.0 {
        let db_type = core.dbi().db_type();
        let rcd_db_type = core.dbi().get_rcd_db_type(&db_name);
        let known_host = core.dbi().get_cds_host_for_part_db(&db_name).unwrap();

        if rcd_db_type == RcdDatabaseType::Partial {
            let statement_type = determine_dml_type(&statement, db_type);
            let table_name = get_table_name(&statement, db_type);

            match statement_type {
                DmlType::Unknown => todo!(),
                DmlType::Insert => todo!(),
                DmlType::Update => {
                    let update_behavior = core
                        .dbi()
                        .get_updates_to_host_behavior(&db_name, &table_name);

                    let data_result = core.dbi().update_data_into_partial_db(
                        &db_name,
                        &table_name,
                        &statement,
                        &known_host.host_id,
                        &where_clause,
                    );

                    let data_info = DataInfo {
                        db_name: db_name.clone(),
                        table_name: table_name.clone(),
                        row_id: data_result.row_id,
                        hash: data_result.data_hash,
                        is_deleted: false,
                    };

                    match update_behavior {
                        UpdatesToHostBehavior::Unknown => todo!(),
                        UpdatesToHostBehavior::SendDataHashChange => {
                            let remote_host =
                                core.dbi().get_cds_host_for_part_db(&db_name).unwrap();
                            let own_host_info =
                                core.dbi().rcd_get_host_info().expect("no host info is set");

                            let notify_result = core
                                .remote()
                                .notify_host_of_updated_hash(
                                    &remote_host,
                                    &own_host_info,
                                    &data_info,
                                )
                                .await;

                            if data_result.is_successful && notify_result {
                                is_overall_successful = true;
                                rows_affected = 1;
                            }
                        }
                        UpdatesToHostBehavior::DoNothing => {
                            is_overall_successful = true;
                            rows_affected = 1;
                        }
                    }
                }
                DmlType::Delete => {
                    let known_host = core.dbi().get_cds_host_for_part_db(&db_name).unwrap();

                    let delete_behavior = core
                        .dbi()
                        .get_deletes_to_host_behavior(&db_name, &table_name);

                    let delete_result = core.dbi().delete_data_in_partial_db(
                        &db_name,
                        &table_name,
                        &statement,
                        &where_clause,
                        &known_host.host_id,
                    );

                    match delete_behavior {
                        DeletesToHostBehavior::Unknown => todo!(),
                        DeletesToHostBehavior::SendNotification => {
                            let remote_host =
                                core.dbi().get_cds_host_for_part_db(&db_name).unwrap();
                            let own_host_info =
                                core.dbi().rcd_get_host_info().expect("no host info is set");

                            let notify_result = core
                                .remote()
                                .notify_host_of_removed_row(
                                    &remote_host,
                                    &own_host_info,
                                    &db_name,
                                    &table_name,
                                    delete_result.row_id,
                                )
                                .await;

                            if !notify_result {
                                warn!("notify host {remote_host:?} of delete was not successful");
                            }

                            if delete_result.is_successful && notify_result {
                                is_overall_successful = true;
                                rows_affected = 1;
                            }
                        }
                        DeletesToHostBehavior::DoNothing => {
                            info!("configured to not notify host on local delete");
                            if delete_result.is_successful {
                                is_overall_successful = true;
                                rows_affected = 1;
                            }
                        }
                    }
                }
                DmlType::Select => todo!(),
            }

            // we need to determine the statement type (INSERT/UPDATE/DELETE)
            // and check to see if we need to communicate changes upstream to the host
            // we do this by looking at the CDS_CONTRACTS_TABLES and checking
            // the UPDATES_TO_HOST_BEHAVIOR and/or the DELETES_TO_HOST_BEHAVIOR
            // and responding accordingly
        }
    }

    ExecuteWriteReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_overall_successful,
        total_rows_affected: rows_affected,
        is_error: false,
        error: None,
    }
}

pub async fn execute_write_at_host(core: &Rcd, request: ExecuteWriteRequest) -> ExecuteWriteReply {
    let mut rows_affected: u32 = 0;
    let auth_result = core.verify_login(request.authentication.unwrap());
    let db_name = request.database_name;
    let statement = request.sql_statement;
    let mut is_sql_successful: bool = false;
    let mut is_error = false;
    let mut rcd_error: Option<RcdError> = None;

    if auth_result.0 {
        let sql_result = core.dbi().execute_write_at_host(&db_name, &statement);

        match sql_result {
            Ok(_) => {
                rows_affected = sql_result.unwrap() as u32;
                is_sql_successful = true;
            }
            Err(e) => {
                is_sql_successful = false;
                is_error = true;
                rcd_error = Some(RcdError {
                    number: 0,
                    message: e.to_string(),
                    help: String::from(""),
                });
            }
        }
    } else {
        warn!("WARNING: execute_write_at_host not authenticated!");
    }

    ExecuteWriteReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_sql_successful,
        total_rows_affected: rows_affected,
        is_error,
        error: rcd_error,
    }
}

pub async fn execute_cooperative_write_at_host(
    core: &Rcd,
    request: ExecuteCooperativeWriteRequest,
) -> ExecuteCooperativeWriteReply {
    let mut is_remote_action_successful = false;

    let auth_result = core.verify_login(request.authentication.unwrap());
    let db_name = request.database_name;
    let statement = request.sql_statement;

    if auth_result.0 && core.dbi().has_participant(&db_name, &request.alias) {
        let dml_type = determine_dml_type(&statement, core.dbi().db_type());
        let db_participant = core
            .dbi()
            .get_participant_by_alias(&db_name, &request.alias)
            .unwrap();
        let host_info = core.dbi().rcd_get_host_info().expect("no host info is set");
        let cmd_table_name = get_table_name(&statement, core.dbi().db_type());
        let where_clause = request.where_clause.clone();

        let db_participant_reference = db_participant.clone();

        match dml_type {
            DmlType::Unknown => {
                panic!();
            }
            DmlType::Insert => {
                let remote_insert_result = core
                    .remote()
                    .insert_row_at_participant(
                        db_participant,
                        &host_info,
                        &db_name,
                        &cmd_table_name,
                        &statement,
                    )
                    .await;

                if remote_insert_result.is_successful {
                    // we need to add the data hash and row id here
                    let data_hash = remote_insert_result.data_hash;
                    let row_id = remote_insert_result.row_id;

                    let internal_participant_id = db_participant_reference.internal_id.to_string();

                    let local_insert_is_successful = core.dbi().insert_metadata_into_host_db(
                        &db_name,
                        &cmd_table_name,
                        row_id,
                        data_hash,
                        &internal_participant_id,
                    );

                    if local_insert_is_successful {
                        is_remote_action_successful = true;
                    }
                } else {
                    warn!("remote insert was not successful: {remote_insert_result:?}");
                }
            }
            DmlType::Update => {
                let remote_update_result = core
                    .remote()
                    .update_row_at_participant(
                        db_participant,
                        &host_info,
                        &db_name,
                        &cmd_table_name,
                        &statement,
                        &where_clause,
                    )
                    .await;

                if remote_update_result.is_successful {
                    let data_hash: u64;
                    let row_id: u32;

                    let update_result =
                        PartialDataStatus::from_u32(remote_update_result.update_status);

                    match update_result {
                        PartialDataStatus::Unknown => todo!(),
                        PartialDataStatus::SucessOverwriteOrLog => {
                            data_hash = remote_update_result.rows.first().unwrap().data_hash;
                            row_id = remote_update_result.rows.first().unwrap().rowid;
                            let internal_participant_id =
                                db_participant_reference.internal_id.to_string();

                            let local_update_is_successful = core.dbi().update_metadata_in_host_db(
                                &db_name,
                                &cmd_table_name,
                                row_id,
                                data_hash,
                                &internal_participant_id,
                            );

                            trace!("local update is successful: {local_update_is_successful}");

                            if local_update_is_successful {
                                is_remote_action_successful = true;
                            }
                        }
                        PartialDataStatus::Pending => {
                            is_remote_action_successful = true;
                        }
                        PartialDataStatus::Ignored => todo!(),
                    }
                }
            }
            DmlType::Delete => {
                let remote_delete_result = core
                    .remote()
                    .remove_row_at_participant(
                        db_participant,
                        &host_info,
                        &db_name,
                        &cmd_table_name,
                        &statement,
                        &where_clause,
                    )
                    .await;

                if remote_delete_result.is_successful {
                    let row_id: u32 = if remote_delete_result.rows.is_empty() {
                        0
                    } else {
                        remote_delete_result.rows.first().unwrap().rowid
                    };

                    let internal_participant_id = db_participant_reference.internal_id.to_string();

                    let local_delete_is_successful = core.dbi().delete_metadata_in_host_db(
                        &db_name,
                        &cmd_table_name,
                        row_id,
                        &internal_participant_id,
                    );

                    if local_delete_is_successful {
                        is_remote_action_successful = true;
                    }
                } else {
                    warn!("remote delete was not successful");
                }
            }
            DmlType::Select => panic!(),
        }
    }

    let execute_write_reply = ExecuteCooperativeWriteReply {
        authentication_result: Some(auth_result.1),
        is_successful: is_remote_action_successful,
        total_rows_affected: 0,
    };

    trace!("{execute_write_reply:?}");

    execute_write_reply
}
