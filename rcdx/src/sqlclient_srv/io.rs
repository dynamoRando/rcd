use rcdproto::rcdp::{
    AuthResult, ExecuteCooperativeWriteReply, ExecuteCooperativeWriteRequest, ExecuteReadReply,
    ExecuteReadRequest, ExecuteWriteReply, ExecuteWriteRequest, StatementResultset,
};

use super::SqlClientImpl;
use crate::{
    host_info::HostInfo,
    query_parser,
    rcd_enum::{DmlType, RcdDatabaseType},
    remote_db_srv,
};
use conv::UnwrapOk;
use conv::ValueFrom;

pub async fn execute_read_at_host(
    request: ExecuteReadRequest,
    client: &SqlClientImpl,
) -> ExecuteReadReply {
    // check if the user is authenticated
    let message = request.clone();

    // println!("execute_read_at_host: {:?}", message);

    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let sql = message.sql_statement;

    let mut result_table = Vec::new();

    let mut statement_result_set = StatementResultset {
        is_error: true,
        result_message: String::from(""),
        number_of_rows_affected: 0,
        rows: Vec::new(),
        execution_error_message: String::from(""),
    };

    if is_authenticated {
        if client.dbi().has_cooperative_tables(&db_name, &sql) {
            // println!("cooperative tables found for {}", sql);

            // we would need to get a list of participants for each of the cooperative tables
            let cooperative_tables = client.dbi().get_cooperative_tables(&db_name, &sql);

            for ct in &cooperative_tables {
                let participants_for_table = client
                    .dbi()
                    .get_participants_for_table(&db_name, ct.as_str());
                for participant in &participants_for_table {
                    // we would need to get rows for that table from the participant
                    let host_info = HostInfo::get(&client.dbi());
                    let remote_data_result = remote_db_srv::get_row_from_participant(
                        participant.clone(),
                        host_info,
                        client.own_db_addr_port.clone(),
                    )
                    .await;

                    let data_hash_for_row = remote_data_result.row.as_ref().unwrap().hash.clone();

                    let saved_hash_for_row = participant.row_data.first().unwrap().1.clone();

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
            let query_result = client.dbi().execute_read_at_host(&db_name, &sql);

            if query_result.is_ok() {
                let result_rows = query_result.unwrap().to_cdata_rows();
                statement_result_set.number_of_rows_affected =
                    u64::value_from(result_rows.len()).unwrap_ok();
                statement_result_set.rows = result_rows;
                statement_result_set.is_error = false;
            } else {
                statement_result_set.execution_error_message =
                    query_result.unwrap_err().to_string();
            }
        }
    }

    let mut statement_results = Vec::new();
    statement_results.push(statement_result_set);

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let execute_read_reply = ExecuteReadReply {
        authentication_result: Some(auth_response),
        total_resultsets: 1,
        results: statement_results,
    };

    return execute_read_reply;
}

pub async fn execute_read_at_participant(
    request: ExecuteReadRequest,
    client: &SqlClientImpl,
) -> ExecuteReadReply {
    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let sql = message.sql_statement;

    let mut statement_result_set = StatementResultset {
        is_error: true,
        result_message: String::from(""),
        number_of_rows_affected: 0,
        rows: Vec::new(),
        execution_error_message: String::from(""),
    };

    if is_authenticated {
        let query_result = client.dbi().execute_read_at_participant(&db_name, &sql);

        if query_result.is_ok() {
            let result_rows = query_result.unwrap().to_cdata_rows();
            statement_result_set.number_of_rows_affected =
                u64::value_from(result_rows.len()).unwrap_ok();
            statement_result_set.rows = result_rows;
            statement_result_set.is_error = false;
        } else {
            statement_result_set.execution_error_message = query_result.unwrap_err().to_string();
        }
    }

    let mut statement_results = Vec::new();
    statement_results.push(statement_result_set);

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let execute_read_reply = ExecuteReadReply {
        authentication_result: Some(auth_response),
        total_resultsets: 1,
        results: statement_results,
    };

    return execute_read_reply;
}

pub async fn execute_write_at_partipant(
    request: ExecuteWriteRequest,
    client: &SqlClientImpl,
) -> ExecuteWriteReply {
    let mut rows_affected: u32 = 0;
    let mut is_overall_successful = false;

    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let statement = message.sql_statement;
    let where_clause = message.where_clause;

    if is_authenticated {
        let db_type = client.dbi().db_type();
        let rcd_db_type = client.dbi().get_rcd_db_type(&db_name);

        if rcd_db_type == RcdDatabaseType::Partial {
            let statement_type = query_parser::determine_dml_type(&statement, db_type);
            let table_name = query_parser::get_table_name(&statement, db_type);

            match statement_type {
                DmlType::Unknown => todo!(),
                DmlType::Insert => todo!(),
                DmlType::Update => {
                    let update_behavior = client
                        .dbi()
                        .get_updates_to_host_behavior(&db_name, &table_name);

                    let update_result = client.dbi().update_data_into_partial_db(
                        &db_name,
                        &table_name,
                        &statement,
                        &where_clause,
                    );

                    match update_behavior {
                        crate::rcd_enum::UpdatesToHostBehavior::Unknown => todo!(),
                        crate::rcd_enum::UpdatesToHostBehavior::SendDataHashChange => {
                            let remote_host = client.dbi().get_cds_host_for_part_db(&db_name).unwrap();
                            let own_host_info = client.dbi().rcd_get_host_info().clone();
                            let own_db_addr_port = client.own_db_addr_port.clone();

                            let notify_result = remote_db_srv::notify_host_of_updated_hash(
                                &remote_host,
                                &own_host_info,
                                own_db_addr_port,
                                &db_name,
                                &table_name,
                                update_result.row_id,
                                update_result.data_hash,
                            )
                            .await;

                            if update_result.is_successful && notify_result {
                                is_overall_successful = true;
                                rows_affected = 1;
                            }
                        }
                        crate::rcd_enum::UpdatesToHostBehavior::DoNothing => {
                            is_overall_successful = true;
                            rows_affected = 1;
                        }
                    }
                }
                DmlType::Delete => {
                    let delete_behavior = client
                        .dbi()
                        .get_deletes_to_host_behavior(&db_name, &table_name);

                    let delete_result = client.dbi().delete_data_in_partial_db(
                        &db_name,
                        &table_name,
                        &statement,
                        &where_clause,
                    );

                    match delete_behavior {
                        crate::rcd_enum::DeletesToHostBehavior::Unknown => todo!(),
                        crate::rcd_enum::DeletesToHostBehavior::SendNotification => {
                            let remote_host = client.dbi().get_cds_host_for_part_db(&db_name).unwrap();
                            let own_host_info = client.dbi().rcd_get_host_info().clone();
                            let own_db_addr_port = client.own_db_addr_port.clone();

                            let notify_result = remote_db_srv::notify_host_of_removed_row(
                                &remote_host,
                                &own_host_info,
                                own_db_addr_port,
                                &db_name,
                                &table_name,
                                delete_result.row_id,
                            )
                            .await;

                            if delete_result.is_successful && notify_result {
                                is_overall_successful = true;
                                rows_affected = 1;
                            }
                        }
                        crate::rcd_enum::DeletesToHostBehavior::DoNothing => todo!(),
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

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let execute_write_reply = ExecuteWriteReply {
        authentication_result: Some(auth_response),
        is_successful: is_overall_successful,
        total_rows_affected: rows_affected,
    };

    return execute_write_reply;
}

pub async fn execute_write_at_host(
    request: ExecuteWriteRequest,
    client: &SqlClientImpl,
) -> ExecuteWriteReply {
    let mut rows_affected: u32 = 0;

    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let statement = message.sql_statement;

    if is_authenticated {
        // println!("{:?}", &statement);
        rows_affected = client.dbi().execute_write_at_host(&db_name, &statement) as u32;
    } else {
        println!("WARNING: execute_write_at_host not authenticated!");
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let execute_write_reply = ExecuteWriteReply {
        authentication_result: Some(auth_response),
        is_successful: true,
        total_rows_affected: rows_affected,
    };

    return execute_write_reply;
}

pub async fn execute_cooperative_write_at_host(
    request: ExecuteCooperativeWriteRequest,
    client: &SqlClientImpl,
) -> ExecuteCooperativeWriteReply {
    let mut is_remote_action_successful = false;

    // check if the user is authenticated
    let message = request.clone();
    let a = message.authentication.unwrap();

    let is_authenticated = client.verify_login(&a.user_name, &a.pw);
    let db_name = message.database_name;
    let statement = message.sql_statement;

    if is_authenticated {
        if client.dbi().has_participant(&db_name, &message.alias) {
            let dml_type = query_parser::determine_dml_type(&statement, client.dbi().db_type());
            let db_participant = client
                .dbi()
                .get_participant_by_alias(&db_name, &message.alias)
                .unwrap();
            let host_info = client.dbi().rcd_get_host_info();
            let cmd_table_name = query_parser::get_table_name(&statement, client.dbi().db_type());
            let where_clause = message.where_clause.clone();

            let db_participant_reference = db_participant.clone();

            match dml_type {
                DmlType::Unknown => {
                    panic!();
                }
                DmlType::Insert => {
                    let remote_insert_result = remote_db_srv::insert_row_at_participant(
                        db_participant,
                        &host_info,
                        &db_name,
                        &cmd_table_name,
                        &statement,
                    )
                    .await;

                    if remote_insert_result.is_successful {
                        // we need to add the data hash and row id here
                        let data_hash = remote_insert_result.data_hash.clone();
                        let row_id = remote_insert_result.row_id;

                        let internal_participant_id =
                            db_participant_reference.internal_id.to_string().clone();

                        let local_insert_is_successful = client.dbi().insert_metadata_into_host_db(
                            &db_name,
                            &cmd_table_name,
                            row_id,
                            data_hash,
                            &internal_participant_id,
                        );

                        if local_insert_is_successful {
                            is_remote_action_successful = true;
                        }
                    }
                }
                DmlType::Update => {
                    let remote_update_result = remote_db_srv::update_row_at_participant(
                        db_participant,
                        &host_info,
                        &db_name,
                        &cmd_table_name,
                        &statement,
                        &where_clause,
                    )
                    .await;

                    if remote_update_result.is_successful {
                        let data_hash = remote_update_result.rows.first().unwrap().data_hash;
                        let row_id = remote_update_result.rows.first().unwrap().rowid;

                        let internal_participant_id =
                            db_participant_reference.internal_id.to_string().clone();

                        let local_update_is_successful = client.dbi().update_metadata_in_host_db(
                            &db_name,
                            &cmd_table_name,
                            row_id,
                            data_hash,
                            &internal_participant_id,
                        );

                        if local_update_is_successful {
                            is_remote_action_successful = true;
                        }
                    }
                }
                DmlType::Delete => {
                    let remote_delete_result = remote_db_srv::remove_row_at_participant(
                        db_participant,
                        &host_info,
                        &db_name,
                        &cmd_table_name,
                        &statement,
                        &where_clause,
                    )
                    .await;

                    if remote_delete_result.is_successful {
                        let row_id = remote_delete_result.rows.first().unwrap().rowid;

                        let internal_participant_id =
                            db_participant_reference.internal_id.to_string().clone();

                        let local_delete_is_successful = client.dbi().delete_metadata_in_host_db(
                            &db_name,
                            &cmd_table_name,
                            row_id,
                            &internal_participant_id,
                        );

                        if local_delete_is_successful {
                            is_remote_action_successful = true;
                        }
                    }
                }
                DmlType::Select => panic!(),
            }
        }
    }

    let auth_response = AuthResult {
        is_authenticated: is_authenticated,
        user_name: String::from(""),
        token: String::from(""),
        authentication_message: String::from(""),
    };

    let execute_write_reply = ExecuteCooperativeWriteReply {
        authentication_result: Some(auth_response),
        is_successful: is_remote_action_successful,
        total_rows_affected: 0,
    };

    return execute_write_reply;
}
