use chrono::Utc;
use rusqlite::named_params;

use crate::sqlite::{
    execute_write, get_scalar_as_u32, has_table, rcd_db::get_deletes_from_host_behavior, sql_text,
};
use rcd_common::{
    db::{get_data_queue_table_name, DbiConfigSqlite, PartialDataResult},
    defaults,
};

use super::{add_record_to_log_table, get_partial_db_connection};
use rcd_enum::{
    deletes_from_host_behavior::DeletesFromHostBehavior,
    partial_data_result_action::PartialDataResultAction,
};

pub fn delete_data_into_partial_db_queue(
    db_name: &str,
    table_name: &str,
    delete_statement: &str,
    where_clause: &str,
    host_id: &str,
    config: &DbiConfigSqlite,
) -> PartialDataResult {
    let queue_log_table = get_data_queue_table_name(table_name);
    let conn = &get_partial_db_connection(db_name, &config.root_folder);

    if !has_table(queue_log_table.clone(), conn) {
        let mut cmd = sql_text::Coop::text_create_data_queue_table();
        cmd = cmd.replace(":table_name", &queue_log_table);
        execute_write(conn, &cmd);
    }

    let mut cmd = String::from("SELECT MAX(ID) FROM :table_name");
    cmd = cmd.replace(":table_name", &queue_log_table);

    let max_id = get_scalar_as_u32(cmd, conn);
    let next_id = max_id + 1;

    cmd = String::from(
        "
        INSERT INTO :table_name 
        (
            ID,
            STATEMENT,
            WHERE_CLAUSE,
            REQUESTED_TS_UTC,
            HOST_ID,
            ACTION
        )
        VALUES
        (
            :id,
            :statement,
            :where_clause,
            :ts,
            :hid,
            'DELETE'
        )
    ;",
    );

    cmd = cmd.replace(":table_name", &queue_log_table);

    let mut statement = conn.prepare(&cmd).unwrap();
    let rows_inserted = statement
        .execute(named_params! {
            ":id": next_id,
            ":statement": delete_statement,
            ":where_clause": where_clause,
            ":ts": Utc::now().to_string(),
            ":hid": host_id,
        })
        .unwrap();

    PartialDataResult {
        is_successful: rows_inserted > 0,
        row_id: next_id,
        data_hash: None,
        partial_data_status: None,
        action: Some(PartialDataResultAction::Delete),
    }
}

pub fn delete_data_in_partial_db(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    where_clause: &str,
    host_id: &str,
    config: &DbiConfigSqlite,
) -> PartialDataResult {
    let behavior = get_deletes_from_host_behavior(db_name, table_name, config);

    match behavior {
        DeletesFromHostBehavior::Unknown => todo!(),
        DeletesFromHostBehavior::AllowRemoval => {
            execute_delete(db_name, table_name, cmd, where_clause, config)
        }
        DeletesFromHostBehavior::QueueForReview => {
            delete_data_into_partial_db_queue(
                db_name,
                table_name,
                cmd,
                where_clause,
                host_id,
                config,
            )
        }
        DeletesFromHostBehavior::DeleteWithLog => {
            execute_delete_with_log(db_name, table_name, cmd, where_clause, config)
        }
        DeletesFromHostBehavior::Ignore => todo!(),
        DeletesFromHostBehavior::QueueForReviewAndLog => todo!(),
    }
}

fn execute_delete(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    where_clause: &str,
    config: &DbiConfigSqlite,
) -> PartialDataResult {
    let original_cmd = cmd;

    let mut cmd;
    cmd = String::from("SELECT ROWID FROM :table_name WHERE :where_clause")
        .replace(":table_name", table_name);

    if !where_clause.is_empty() {
        cmd = cmd.replace(":where_clause", where_clause);
    } else {
        cmd = cmd.replace("WHERE", "");
        cmd = cmd.replace(":where_clause", "");
    }

    // we need to determine the row_ids that we're going to update because we're going to need to delete them
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let mut statement = conn.prepare(&cmd).unwrap();

    // once we have the row ids, then we will delete the rows in the actual and metadata table

    let mut row_ids: Vec<u32> = Vec::new();
    let row_to_id = |rowid: u32| -> rusqlite::Result<u32> { Ok(rowid) };

    let ids = statement
        .query_and_then([], |row| row_to_id(row.get(0).unwrap()))
        .unwrap();

    for id in ids {
        row_ids.push(id.unwrap());
    }

    println!("{:?}", row_ids);

    let total_rows = execute_write(&conn, original_cmd);

    println!("total rows deleted: {}", total_rows);

    if total_rows != row_ids.len() {
        panic!("the delete statement did not match the expected count of affected rows");
    }

    // now we need to delete data from the metadata table
    let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);
    let mut cmd = String::from("DELETE FROM :table_name WHERE ROW_ID = :rid");
    cmd = cmd.replace(":table_name", &metadata_table_name);

    for row in &row_ids {
        let mut statement = conn.prepare(&cmd).unwrap();
        statement.execute(named_params! {":rid" : row}).unwrap();
        println!("{:?}", statement);
    }

    let deleted_row_id = row_ids.first().unwrap();

    let result = PartialDataResult {
        is_successful: true,
        row_id: *deleted_row_id,
        data_hash: None,
        partial_data_status: None,
        action: Some(PartialDataResultAction::Delete),
    };

    println!("{:?}", result);

    result
}

pub fn handle_delete_pending_action(
    db_name: &str,
    table_name: &str,
    sql_update_statement: &str,
    where_clause: &str,
    row_id: u32,
    config: &DbiConfigSqlite,
) -> PartialDataResult {
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let queue_table_name = get_data_queue_table_name(table_name);

    let behavior = get_deletes_from_host_behavior(db_name, table_name, config);

    let mut update_result = PartialDataResult {
        is_successful: false,
        row_id: 0,
        data_hash: None,
        partial_data_status: None,
        action: Some(PartialDataResultAction::Delete),
    };

    if behavior == DeletesFromHostBehavior::QueueForReview {
        update_result = execute_delete(
            db_name,
            table_name,
            sql_update_statement,
            where_clause,
            config,
        );

        if update_result.is_successful {
            let mut cmd = String::from("DELETE FROM :table_name WHERE ID = :rid");
            cmd = cmd.replace(":table_name", &queue_table_name);
            cmd = cmd.replace(":rid", &row_id.to_string());
            execute_write(&conn, &cmd);
        }
    } else if behavior == DeletesFromHostBehavior::QueueForReviewAndLog {
        update_result = execute_delete_with_log(
            db_name,
            table_name,
            sql_update_statement,
            where_clause,
            config,
        );

        if update_result.is_successful {
            let mut cmd = String::from("DELETE FROM :table_name WHERE ID = :rid");
            cmd = cmd.replace(":table_name", &queue_table_name);
            cmd = cmd.replace(":rid", &row_id.to_string());
            execute_write(&conn, &cmd);
        }
    }

    update_result
}

fn execute_delete_with_log(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    where_clause: &str,
    config: &DbiConfigSqlite,
) -> PartialDataResult {
    add_record_to_log_table(db_name, table_name, where_clause, "DELETE", config);
    execute_delete(db_name, table_name, cmd, where_clause, config)
}
