use crate::sqlite::{
    execute_write, get_scalar_as_u32, get_table_col_names, has_table,
    rcd_db::get_updates_from_host_behavior, sql_text,
};

use super::{add_record_to_log_table, get_partial_db_connection};
use chrono::Utc;
use rcd_common::{
    crypt,
    db::{get_data_queue_table_name, DbiConfigSqlite, PartialDataResult},
    defaults,
    rcd_enum::{UpdatesFromHostBehavior},
};
use rcd_enum::{partial_data_result_action::PartialDataResultAction, partial_data_status::PartialDataStatus};
use rusqlite::{named_params, types::Type};

pub fn update_data_into_partial_db_queue(
    db_name: &str,
    table_name: &str,
    update_statement: &str,
    where_clause: &str,
    host_id: &str,
    config: &DbiConfigSqlite,
) -> PartialDataResult {
    let queue_log_table = get_data_queue_table_name(table_name);
    let conn = &get_partial_db_connection(db_name, &config.root_folder);

    if !has_table(queue_log_table.clone(), conn) {
        let mut cmd = sql_text::COOP::text_create_data_queue_table();
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
            'UPDATE'
        )
    ;",
    );

    cmd = cmd.replace(":table_name", &queue_log_table);

    let mut statement = conn.prepare(&cmd).unwrap();
    let rows_inserted = statement
        .execute(named_params! {
            ":id": next_id,
            ":statement": update_statement,
            ":where_clause": where_clause,
            ":ts": Utc::now().to_string(),
            ":hid": host_id,
        })
        .unwrap();

    let update_result = PartialDataResult {
        is_successful: rows_inserted > 0,
        row_id: next_id,
        data_hash: None,
        partial_data_status: Some(PartialDataStatus::to_u32(PartialDataStatus::Pending)),
        action: Some(PartialDataResultAction::Update),
    };

    return update_result;
}

pub fn update_data_into_partial_db(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    where_clause: &str,
    host_id: &str,
    config: &DbiConfigSqlite,
) -> PartialDataResult {
    let behavior = get_updates_from_host_behavior(db_name, table_name, config);
    match behavior {
        UpdatesFromHostBehavior::AllowOverwrite => {
            return execute_update_overwrite(db_name, table_name, cmd, where_clause, config);
        }
        UpdatesFromHostBehavior::Unknown => todo!(),
        UpdatesFromHostBehavior::QueueForReview => {
            return update_data_into_partial_db_queue(
                db_name,
                table_name,
                cmd,
                where_clause,
                host_id,
                config,
            );
        }
        UpdatesFromHostBehavior::OverwriteWithLog => {
            execute_update_with_log(db_name, table_name, cmd, where_clause, config)
        }
        UpdatesFromHostBehavior::Ignore => todo!(),
        UpdatesFromHostBehavior::QueueForReviewAndLog => todo!(),
    }
}

fn execute_update_overwrite(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    where_clause: &str,
    config: &DbiConfigSqlite,
) -> PartialDataResult {
    let original_cmd = cmd.clone();
    let mut cmd;
    cmd = String::from("SELECT ROWID FROM :table_name WHERE :where_clause")
        .replace(":table_name", table_name);

    if where_clause.len() > 0 {
        cmd = cmd.replace(":where_clause", where_clause);
    } else {
        cmd = cmd.replace("WHERE", "");
        cmd = cmd.replace(":where_clause", "");
    }

    // we need to determine the row_ids that we're going to update because we're going to need to update
    // the data hashes for them
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let mut statement = conn.prepare(&cmd).unwrap();

    // once we have the row ids, then we will need to get the hash of the rows after they've been updated.

    let mut row_ids: Vec<u32> = Vec::new();
    let row_to_id = |rowid: u32| -> rusqlite::Result<u32> { Ok(rowid) };

    let ids = statement
        .query_and_then([], |row| row_to_id(row.get(0).unwrap()))
        .unwrap();

    for id in ids {
        row_ids.push(id.unwrap());
    }

    // println!("{:?}", row_ids);

    let total_rows = execute_write(&conn, &original_cmd);

    if total_rows != row_ids.len() {
        panic!("the update statement did not match the expected count of affected rows");
    }

    // now we need to update the data hashes for every row that was changed
    // ... how do we do that?
    let col_names = get_table_col_names(table_name.to_string(), &conn);
    let mut cmd;
    cmd = String::from("SELECT :col_names FROM :table_name WHERE ROWID = :rid");
    cmd = cmd.replace(":table_name", table_name);

    let mut col_name_list = String::from("");

    for name in &col_names {
        col_name_list = col_name_list + name + ",";
    }

    let completed_col_name_list = &col_name_list[0..&col_name_list.len() - 1];
    // println!("{}", completed_col_name_list);

    cmd = cmd.replace(":col_names", &completed_col_name_list);

    // println!("{:?}", cmd);

    let mut row_hashes: Vec<(u32, u64)> = Vec::new();

    for id in &row_ids {
        let sql = cmd.replace(":rid", &id.to_string());

        // println!("{:?}", sql);

        let mut stmt = conn.prepare(&sql).unwrap();
        let mut rows = stmt.query([]).unwrap();

        // for a single row
        while let Some(row) = rows.next().unwrap() {
            let mut row_values: Vec<String> = Vec::new();
            for i in 0..col_names.len() {
                let dt = row.get_ref_unwrap(i).data_type();

                let string_value: String = match dt {
                    Type::Blob => String::from(""),
                    Type::Integer => row.get_ref_unwrap(i).as_i64().unwrap().to_string(),
                    Type::Real => row.get_ref_unwrap(i).as_f64().unwrap().to_string(),
                    Type::Text => row.get_ref_unwrap(i).as_str().unwrap().to_string(),
                    _ => String::from(""),
                };

                row_values.push(string_value);
            }

            let hash_value = crypt::calculate_hash_for_struct(&row_values);
            let row_hash: (u32, u64) = (*id, hash_value);
            row_hashes.push(row_hash);
        }
    }

    // now that we have the row hashes, we should save them back to the table
    let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);
    let mut cmd = String::from("UPDATE :table_name SET HASH = :hash WHERE ROW_ID = :rid");
    cmd = cmd.replace(":table_name", &metadata_table_name);

    for row in &row_hashes {
        let mut statement = conn.prepare(&cmd).unwrap();
        statement
            .execute(named_params! {":hash": row.1.to_ne_bytes(), ":rid" : row.0})
            .unwrap();
    }

    let row_data = row_hashes.first().unwrap();

    let result = PartialDataResult {
        is_successful: true,
        row_id: row_data.0,
        data_hash: Some(row_data.1),
        partial_data_status: Some(1),
        action: Some(PartialDataResultAction::Update),
    };

    return result;
}

fn execute_update_with_log(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    where_clause: &str,
    config: &DbiConfigSqlite,
) -> PartialDataResult {
    add_record_to_log_table(db_name, table_name, where_clause, "UPDATE", config);
    execute_update_overwrite(db_name, table_name, cmd, where_clause, config)
}

pub fn handle_update_pending_action(
    db_name: &str,
    table_name: &str,
    sql_update_statement: &str,
    where_clause: &str,
    row_id: u32,
    config: &DbiConfigSqlite,
) -> PartialDataResult {
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let queue_table_name = get_data_queue_table_name(table_name);

    let behavior = get_updates_from_host_behavior(db_name, table_name, config);

    let mut update_result = PartialDataResult {
        is_successful: false,
        row_id: 0,
        data_hash: None,
        partial_data_status: None,
        action: Some(PartialDataResultAction::Update),
    };

    if behavior == UpdatesFromHostBehavior::QueueForReview {
        update_result = execute_update_overwrite(
            db_name,
            table_name,
            &sql_update_statement,
            &where_clause,
            config,
        );

        if update_result.is_successful {
            let mut cmd = String::from("DELETE FROM :table_name WHERE ID = :rid");
            cmd = cmd.replace(":table_name", &queue_table_name);
            cmd = cmd.replace(":rid", &row_id.to_string());
            execute_write(&conn, &cmd);
        }
    } else if behavior == UpdatesFromHostBehavior::QueueForReviewAndLog {
        update_result = execute_update_with_log(
            db_name,
            table_name,
            &sql_update_statement,
            &where_clause,
            config,
        );

        if update_result.is_successful {
            let mut cmd = String::from("DELETE FROM :table_name WHERE ID = :rid");
            cmd = cmd.replace(":table_name", &queue_table_name);
            cmd = cmd.replace(":rid", &row_id.to_string());
            execute_write(&conn, &cmd);
        }
    }

    return update_result;
}
