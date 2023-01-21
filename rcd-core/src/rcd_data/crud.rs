use super::RcdData;
use rcd_common::db::PartialDataResult;
use rcd_enum::deletes_from_host_behavior::DeletesFromHostBehavior;
use rcd_enum::updates_from_host_behavior::UpdatesFromHostBehavior;
use rcd_enum::{
    partial_data_result_action::PartialDataResultAction, partial_data_status::PartialDataStatus,
};
use rcdproto::rcdp::{
    DeleteDataRequest, DeleteDataResult, GetRowFromPartialDatabaseRequest,
    GetRowFromPartialDatabaseResult, InsertDataRequest, InsertDataResult,
    NotifyHostOfRemovedRowRequest, NotifyHostOfRemovedRowResponse, Row, RowInfo, UpdateDataRequest,
    UpdateDataResult, UpdateRowDataHashForHostRequest, UpdateRowDataHashForHostResponse,
};

pub async fn insert_command_into_table(
    core: &RcdData,
    request: InsertDataRequest,
) -> InsertDataResult {
    let auth_result = core.authenticate_host(request.authentication.unwrap());
    let db_name = request.database_name;
    let table_name = request.table_name;

    let mut result = PartialDataResult {
        is_successful: false,
        row_id: 0,
        data_hash: None,
        partial_data_status: None,
        action: Some(PartialDataResultAction::Insert),
    };

    if auth_result.0 {
        let cmd = &request.cmd;

        result = core
            .dbi()
            .insert_data_into_partial_db(&db_name, &table_name, cmd);
    }

    

    InsertDataResult {
        authentication_result: Some(auth_result.1),
        is_successful: result.is_successful,
        data_hash: result.data_hash.unwrap(),
        message: String::from(""),
        row_id: result.row_id,
    }
}

pub async fn delete_command_into_table(
    core: &RcdData,
    request: DeleteDataRequest,
) -> DeleteDataResult {
    let auth_result = core.authenticate_host(request.authentication.unwrap());
    let db_name = request.database_name;
    let table_name = request.table_name;
    let where_clause = request.where_clause.clone();
    let mut action_message = String::from("");

    let mut rows: Vec<RowInfo> = Vec::new();

    let mut result = PartialDataResult {
        is_successful: false,
        row_id: 0,
        data_hash: None,
        partial_data_status: None,
        action: Some(PartialDataResultAction::Delete),
    };

    if auth_result.0 {
        let known_host = core.dbi().get_cds_host_for_part_db(&db_name).unwrap();

        // need to check if this is allowed
        let behavior = core
            .dbi()
            .get_deletes_from_host_behavior(&db_name, &table_name);

        match behavior {
            DeletesFromHostBehavior::Ignore => {
                action_message = format!(
                    "The participant does not allow updates for db {} table: {}",
                    db_name, table_name
                );
            }
            DeletesFromHostBehavior::AllowRemoval => {
                let cmd = &request.cmd;

                result = core.dbi().delete_data_in_partial_db(
                    &db_name,
                    &table_name,
                    cmd,
                    &where_clause,
                    &known_host.host_id,
                );

                let hash = match result.data_hash {
                    Some(_) => result.data_hash.unwrap(),
                    None => 0,
                };

                if result.is_successful {
                    let row = RowInfo {
                        database_name: db_name,
                        table_name,
                        rowid: result.row_id,
                        data_hash: hash,
                    };
                    rows.push(row);
                }
            }
            DeletesFromHostBehavior::DeleteWithLog => {
                let cmd = &request.cmd;

                result = core.dbi().delete_data_in_partial_db(
                    &db_name,
                    &table_name,
                    cmd,
                    &where_clause,
                    &known_host.host_id,
                );

                let hash = match result.data_hash {
                    Some(_) => result.data_hash.unwrap(),
                    None => 0,
                };

                if result.is_successful {
                    let row = RowInfo {
                        database_name: db_name,
                        table_name,
                        rowid: result.row_id,
                        data_hash: hash,
                    };
                    rows.push(row);
                }
            }
            DeletesFromHostBehavior::QueueForReview => {
                let cmd = &request.cmd;

                result = core.dbi().delete_data_in_partial_db(
                    &db_name,
                    &table_name,
                    cmd,
                    &where_clause,
                    &known_host.host_id,
                );

                if result.is_successful {
                    action_message =
                        String::from("The delete statement has been logged for review");
                }
            }
            DeletesFromHostBehavior::Unknown => todo!(),
            DeletesFromHostBehavior::QueueForReviewAndLog => todo!(),
        }
    }

    

    DeleteDataResult {
        authentication_result: Some(auth_result.1),
        is_successful: result.is_successful,
        message: action_message,
        rows,
    }
}

pub async fn update_command_into_table(
    core: &RcdData,
    request: UpdateDataRequest,
) -> UpdateDataResult {
    let auth_result = core.authenticate_host(request.authentication.unwrap());

    let db_name = request.database_name;
    let table_name = request.table_name;
    let where_clause = request.where_clause.clone();
    let mut action_message = String::from("");
    let mut update_status: u32 = 0;
    let mut rows: Vec<RowInfo> = Vec::new();

    let mut result = PartialDataResult {
        is_successful: false,
        row_id: 0,
        data_hash: None,
        partial_data_status: None,
        action: Some(PartialDataResultAction::Update),
    };

    if auth_result.0 {
        let known_host = core.dbi().get_cds_host_for_part_db(&db_name).unwrap();
        let cmd = &request.cmd;

        // need to check if this is allowed
        let behavior = core
            .dbi()
            .get_updates_from_host_behavior(&db_name, &table_name);

        match behavior {
            UpdatesFromHostBehavior::Ignore => {
                action_message = format!(
                    "The participant does not allow updates for db {} table: {}",
                    db_name, table_name
                );
                update_status = PartialDataStatus::to_u32(PartialDataStatus::Ignored);
            }
            UpdatesFromHostBehavior::AllowOverwrite => {
                result = core.dbi().update_data_into_partial_db(
                    &db_name,
                    &table_name,
                    cmd,
                    &known_host.host_id,
                    &where_clause,
                );

                if result.is_successful {
                    let row = RowInfo {
                        database_name: db_name,
                        table_name,
                        rowid: result.row_id,
                        data_hash: result.data_hash.unwrap(),
                    };
                    rows.push(row);
                    update_status =
                        PartialDataStatus::to_u32(PartialDataStatus::SucessOverwriteOrLog);
                }
            }
            UpdatesFromHostBehavior::OverwriteWithLog => {
                result = core.dbi().update_data_into_partial_db(
                    &db_name,
                    &table_name,
                    cmd,
                    &known_host.host_id,
                    &where_clause,
                );

                if result.is_successful {
                    let row = RowInfo {
                        database_name: db_name,
                        table_name,
                        rowid: result.row_id,
                        data_hash: result.data_hash.unwrap(),
                    };
                    rows.push(row);
                    update_status =
                        PartialDataStatus::to_u32(PartialDataStatus::SucessOverwriteOrLog);
                }
            }
            UpdatesFromHostBehavior::QueueForReview => {
                result = core.dbi().update_data_into_partial_db_queue(
                    &db_name,
                    &table_name,
                    cmd,
                    &where_clause,
                    &known_host,
                );

                if result.is_successful {
                    update_status = PartialDataStatus::to_u32(PartialDataStatus::Pending);
                    action_message =
                        String::from("The update statement has been logged for review");
                }
            }
            UpdatesFromHostBehavior::Unknown => unimplemented!(),
            UpdatesFromHostBehavior::QueueForReviewAndLog => {
                result = core.dbi().update_data_into_partial_db_queue(
                    &db_name,
                    &table_name,
                    cmd,
                    &where_clause,
                    &known_host,
                );

                if result.is_successful {
                    update_status = PartialDataStatus::to_u32(PartialDataStatus::Pending);
                    action_message =
                        String::from("The update statement has been logged for review");
                }
            }
        }
    }

    

    UpdateDataResult {
        authentication_result: Some(auth_result.1),
        is_successful: result.is_successful,
        message: action_message,
        rows,
        update_status,
    }
}

pub async fn get_row_from_partial_database(
    core: &RcdData,
    request: GetRowFromPartialDatabaseRequest,
) -> GetRowFromPartialDatabaseResult {
    let auth_result = core.authenticate_host(request.authentication.unwrap());

    let mut result_row = Row {
        row_id: 0,
        database_name: request.row_address.as_ref().unwrap().database_name.clone(),
        table_name: request.row_address.as_ref().unwrap().table_name.clone(),
        values: Vec::new(),
        is_remoteable: true,
        remote_metadata: None,
        hash: Vec::new(),
    };

    if auth_result.0 {
        let db_name = request.row_address.as_ref().unwrap().database_name.clone();
        let table_name = request.row_address.as_ref().unwrap().table_name.clone();
        let row_id = request.row_address.as_ref().unwrap().row_id;

        result_row = core
            .dbi()
            .get_row_from_partial_database(&db_name, &table_name, row_id);
    }

    

    GetRowFromPartialDatabaseResult {
        authentication_result: Some(auth_result.1),
        is_successful: false,
        result_message: String::from(""),
        row: Some(result_row),
    }
}

pub async fn update_row_data_hash_for_host(
    core: &RcdData,
    request: UpdateRowDataHashForHostRequest,
) -> UpdateRowDataHashForHostResponse {
    let mut is_successful = false;

    let authentication = request.authentication.unwrap();
    let user_name = authentication.user_name.clone();

    let auth_result = core.authenticate_participant(authentication, &request.database_name);

    if auth_result.0 {
        println!("is authenticated");
        let db_name = request.database_name.clone();
        let table_name = request.table_name.clone();
        let row_id = request.row_id;
        let hash = request.updated_hash_value;

        let internal_participant_id = core
            .dbi()
            .get_participant_by_alias(&db_name, &user_name)
            .unwrap()
            .internal_id;

        is_successful = core.dbi().update_metadata_in_host_db(
            &db_name,
            &table_name,
            row_id,
            hash,
            &internal_participant_id.to_string(),
        );
    } else {
        println!("not authenticated!");
    }

    

    UpdateRowDataHashForHostResponse {
        authentication_result: Some(auth_result.1),
        is_successful,
    }
}

pub async fn notify_host_of_removed_row(
    core: &RcdData,
    request: NotifyHostOfRemovedRowRequest,
) -> NotifyHostOfRemovedRowResponse {
    let auth_result =
        core.authenticate_participant(request.authentication.unwrap(), &request.database_name);
    let mut is_successful = false;

    if auth_result.0 {
        println!("is authenticated");
        let db_name = request.database_name.clone();
        let table_name = request.table_name.clone();
        let row_id = request.row_id;

        is_successful =
            core.dbi()
                .remove_remote_row_reference_from_host(&db_name, &table_name, row_id);
    } else {
        println!("not authenticated!");
    }

    

    NotifyHostOfRemovedRowResponse {
        authentication_result: Some(auth_result.1),
        is_successful,
    }
}
