use std::path::Path;

use super::rcd_db::{get_deletes_from_host_behavior, get_updates_from_host_behavior};
use super::{
    execute_read_on_connection_for_row, get_db_conn_with_result, get_scalar_as_u32,
    get_scalar_as_u64, get_table_col_names,
};
use crate::dbi::sqlite::{
    execute_write, get_table_col_names_with_data_type_as_string, has_table, sql_text,
};
use crate::dbi::{
    get_data_log_table_name, get_data_queue_table_name, get_metadata_table_name, CdsHosts,
    DbiConfigSqlite, DeletePartialDataResult, InsertPartialDataResult, UpdatePartialDataResult,
};
use crate::rcd_enum::{ColumnType, DatabaseType, UpdatesFromHostBehavior};
use crate::table::Table;
use crate::{crypt, defaults, query_parser};
use chrono::Utc;
use rcdproto::rcdp::{ColumnSchema, Contract, TableSchema};
use rusqlite::types::Type;
use rusqlite::{named_params, Connection, Result};

pub fn get_data_hash_at_participant(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    config: &DbiConfigSqlite,
) -> u64 {
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let metadata_table_name = get_metadata_table_name(table_name);
    let mut cmd = String::from("SELECT HASH FROM :metadata WHERE ROW_ID = :row_id");
    cmd = cmd.replace(":metadata", &metadata_table_name);
    cmd = cmd.replace(":row_id", &row_id.to_string());

    return get_scalar_as_u64(cmd, &conn).unwrap();
}

pub fn get_row_from_partial_database(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    config: &DbiConfigSqlite,
) -> rcdproto::rcdp::Row {
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let mut cmd = String::from("SELECT * from :table_name WHERE ROWID = :rid");

    cmd = cmd.replace(":table_name", table_name);
    cmd = cmd.replace(":rid", &row_id.to_string());

    return execute_read_on_connection_for_row(db_name, table_name, row_id, cmd, &conn).unwrap();
}

pub fn delete_data_in_partial_db(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    where_clause: &str,
    config: &DbiConfigSqlite,
) -> DeletePartialDataResult {
    let behavior = get_deletes_from_host_behavior(db_name, table_name, config);

    match behavior {
        crate::rcd_enum::DeletesFromHostBehavior::Unknown => todo!(),
        crate::rcd_enum::DeletesFromHostBehavior::AllowRemoval => {
            return execute_delete(db_name, table_name, cmd, where_clause, config)
        }
        crate::rcd_enum::DeletesFromHostBehavior::QueueForReview => todo!(),
        crate::rcd_enum::DeletesFromHostBehavior::DeleteWithLog => todo!(),
        crate::rcd_enum::DeletesFromHostBehavior::Ignore => todo!(),
    }
}

#[allow(dead_code, unused_variables, unused_mut, unused_assignments)]
pub fn update_data_into_partial_db_queue(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    where_clause: &str,
    host: &CdsHosts,
    config: &DbiConfigSqlite,
) -> UpdatePartialDataResult {
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

    unimplemented!()
}

pub fn update_data_into_partial_db(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    where_clause: &str,
    host: &CdsHosts,
    config: &DbiConfigSqlite,
) -> UpdatePartialDataResult {
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
                host,
                config,
            );
        }
        UpdatesFromHostBehavior::OverwriteWithLog => {
            execute_update_with_log(db_name, table_name, cmd, where_clause, config)
        }
        UpdatesFromHostBehavior::Ignore => todo!(),
    }
}

pub fn insert_data_into_partial_db(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    config: &DbiConfigSqlite,
) -> InsertPartialDataResult {
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let mut row_id = 0;

    let total_rows = execute_write(&conn, cmd);
    if total_rows > 0 {
        let cmd = String::from("select last_insert_rowid()");
        row_id = get_scalar_as_u32(cmd, &conn);
    }

    // we need to parse the values of this row
    // and create a data hash for it
    let insert_values = query_parser::get_values_from_insert_statement(cmd, DatabaseType::Sqlite);
    let hash_value = crypt::calculate_hash_for_struct(&insert_values);

    // we need to determine if there is a metadata table for this table or not
    // and if there is not one, create it
    // then we need to save the data hash along with the row id
    let metadata_table_name = get_metadata_table_name(table_name);

    if !has_table(metadata_table_name.clone(), &conn) {
        //  need to create table
        let mut cmd = sql_text::COOP::text_create_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name.clone());
        execute_write(&conn, &cmd);
    }

    let mut cmd = sql_text::COOP::text_insert_row_metadata_table();
    cmd = cmd.replace(":table_name", &metadata_table_name.clone());
    let mut statement = conn.prepare(&cmd).unwrap();

    println!("{:?}", row_id);
    println!("{:?}", hash_value);

    statement
        .execute(named_params! {":row": row_id, ":hash" : hash_value.to_ne_bytes() })
        .unwrap();

    let result = InsertPartialDataResult {
        is_successful: total_rows > 0,
        row_id,
        data_hash: hash_value,
    };

    return result;
}

pub fn create_partial_database_from_contract(
    contract: &Contract,
    config: &DbiConfigSqlite,
) -> bool {
    println!("{:?}", config);

    let db_name = contract.schema.as_ref().unwrap().database_name.clone();
    let _ = create_partial_database(&db_name, config);

    let conn = get_partial_db_connection(&db_name, &config.root_folder);

    for table in &contract.schema.as_ref().unwrap().tables {
        create_table_from_schema(table, &conn);
    }

    return true;
}

pub fn create_partial_database(
    db_name: &str,
    config: &DbiConfigSqlite,
) -> Result<Connection, rusqlite::Error> {
    let mut db_part_name = db_name.replace(".db", "");
    db_part_name = db_part_name.replace(".dbpart", "");
    db_part_name = format!("{}{}", db_part_name, String::from(".dbpart"));
    return get_db_conn_with_result(config, &db_part_name);
}

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn get_db_id(db_name: &str, config: &DbiConfigSqlite) -> String {
    unimplemented!();
}

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn get_table_id(db_name: &str, table_name: &str, config: &DbiConfigSqlite) -> String {
    unimplemented!();
}

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn create_table_in_partial_database(
    db_name: &str,
    table_name: &str,
    schema: Vec<ColumnSchema>,
    config: &DbiConfigSqlite,
) -> Result<bool> {
    unimplemented!();
}

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn add_row_to_partial_database(db_name: &str, table_name: &str, row_data: Table) -> String {
    unimplemented!();
}

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn update_row_in_partial_database(db_name: &str, table_name: &str, row_data: Table) -> String {
    unimplemented!();
}

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn save_contract(db_name: &str, table_name: &str, row_data: Table) -> String {
    unimplemented!();
}

pub fn read_row_id_from_part_db(
    db_name: &str,
    table_name: &str,
    where_clause: &str,
    config: &DbiConfigSqlite,
) -> u32 {
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let mut cmd = String::from("SELECT ROWID FROM :table_name WHERE :where_clause");
    cmd = cmd.replace(":table_name", table_name);
    cmd = cmd.replace(":where_clause", where_clause);

    let row_id = get_scalar_as_u32(cmd, &conn);

    return row_id;
}

pub fn get_partial_db_connection(db_name: &str, cwd: &str) -> Connection {
    let mut db_part_name = db_name.replace(".db", "");
    db_part_name = db_part_name.replace(".dbpart", "");
    db_part_name = format!("{}{}", db_part_name, String::from(".dbpart"));
    let db_path = Path::new(&cwd).join(&db_part_name);
    let conn = Connection::open(&db_path).unwrap();
    return conn;
}

fn create_table_from_schema(table_schema: &TableSchema, conn: &Connection) {
    let table_name = table_schema.table_name.clone();
    let mut cmd = String::from("CREATE TABLE IF NOT EXISTS :tablename ");
    cmd = cmd.replace(":tablename", &table_name);
    cmd = cmd + " ( ";

    for column in &table_schema.columns {
        let col_name = column.column_name.clone();
        let col_type = ColumnType::from_u32(column.column_type).data_type_as_string_sqlite();
        let mut col_length = String::from("");

        if column.column_length > 0 {
            col_length = col_length + " ( ";
            col_length = col_length + &column.column_length.to_string();
            col_length = col_length + " ) ";
        }

        let mut col_nullable = String::from("");

        if !column.is_nullable {
            col_nullable = String::from("NOT NULL");
        }

        let col_statement: String;

        let last_column = &table_schema.columns.last().unwrap().column_name;

        if last_column.to_string() == column.column_name {
            col_statement = format!(
                " {} {} {} {} ",
                col_name, col_type, col_length, col_nullable
            );
        } else {
            col_statement = format!(
                " {} {} {} {} , ",
                col_name, col_type, col_length, col_nullable
            );
        }

        cmd = cmd + &col_statement;
    }
    cmd = cmd + " ) ";
    execute_write(conn, &cmd);
}

fn execute_update_with_log(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    where_clause: &str,
    config: &DbiConfigSqlite,
) -> UpdatePartialDataResult {
    let data_log_table = get_data_log_table_name(table_name);
    let conn = &get_partial_db_connection(db_name, &config.root_folder);

    if !has_table(data_log_table.clone(), conn) {
        let mut cmd = sql_text::COOP::text_create_data_log_table();
        let table_col_names =
            get_table_col_names_with_data_type_as_string(db_name, table_name, config);
        cmd = cmd.replace(":column_list", &table_col_names);
        cmd = cmd.replace(":table_name", &data_log_table);

        execute_write(conn, &cmd);
    }

    // we first need to determine the rows that we're about to overwrite and get them so we can insert them
    let col_names_vec = get_table_col_names(table_name.to_string(), conn);
    let mut col_names = String::from("");
    let mut original_col_names = String::from("");

    for name in &col_names_vec {
        let item = format!("{}{}", name, ",");
        col_names = format!("{}{}", col_names, item);
        original_col_names = format!("{}{}", original_col_names, item);
    }

    // remove the final comma from the list of original column names
    original_col_names = original_col_names[0..original_col_names.len() - 1].to_string();

    // for the list of column names, add rowid as a column to get from the db
    col_names = format!("{}{}", col_names, "ROWID");

    let mut select_cmd = String::from("SELECT :col_names FROM :table_name WHERE :where_clause");
    select_cmd = select_cmd.replace(":col_names", &col_names);
    select_cmd = select_cmd.replace(":table_name", table_name);
    select_cmd = select_cmd.replace(":where_clause", where_clause);

    let mut stmt = conn.prepare(&select_cmd).unwrap();
    let mut rows = stmt.query([]).unwrap();

    // for every row that we find that we're going to change, we want to insert a copy of it into the data_log_table
    while let Some(row) = rows.next().unwrap() {
        let mut insert_cmd = String::from("INSERT INTO :data_log_table ( :cols, ROW_ID, ACTION, TS_UTC ) VALUES ( :col_vals, :rid, ':action', ':ts_utc') ");
        insert_cmd = insert_cmd.replace(":data_log_table", &data_log_table);
        insert_cmd = insert_cmd.replace(":cols", &original_col_names);

        // need to build the rest of the insert statement - the column values, rowid, etc.
        let mut col_vals = String::from("");
        let total_cols = col_names_vec.len();

        // iterate over the column names and get the value for each as a string
        // remember, the last item is the ROWID, which is not in this list and we will need to get
        for i in 0..col_names_vec.len() {
            let dt = row.get_ref_unwrap(i).data_type();

            let string_value: String = match dt {
                Type::Blob => String::from(""),
                Type::Integer => row.get_ref_unwrap(i).as_i64().unwrap().to_string(),
                Type::Real => row.get_ref_unwrap(i).as_f64().unwrap().to_string(),
                Type::Text => {
                    format!("'{}'", row.get_ref_unwrap(i).as_str().unwrap().to_string())
                }
                _ => String::from(""),
            };

            // add the value to the list of values to insert
            col_vals = format!("{}{}{}", col_vals, string_value, ",");
        }

        col_vals = col_vals[0..col_vals.len() - 1].to_string();
        insert_cmd = insert_cmd.replace(":col_vals", &col_vals);

        println!("{:?}", insert_cmd);

        let row_id_val = row.get_ref_unwrap(total_cols).as_i64().unwrap().to_string();

        insert_cmd = insert_cmd.replace(":rid", &row_id_val);
        insert_cmd = insert_cmd.replace(":action", "UPDATE");
        insert_cmd = insert_cmd.replace(":ts_utc", &Utc::now().to_string());

        println!("{:?}", insert_cmd);

        execute_write(conn, &insert_cmd);
    }

    execute_update_overwrite(db_name, table_name, cmd, where_clause, config)
}

fn execute_update_overwrite(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    where_clause: &str,
    config: &DbiConfigSqlite,
) -> UpdatePartialDataResult {
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
    let row_to_id = |rowid: u32| -> Result<u32> { Ok(rowid) };

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

    let result = UpdatePartialDataResult {
        is_successful: true,
        row_id: row_data.0,
        data_hash: row_data.1,
        update_status: 1,
    };

    return result;
}

fn execute_delete(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    where_clause: &str,
    config: &DbiConfigSqlite,
) -> DeletePartialDataResult {
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

    // we need to determine the row_ids that we're going to update because we're going to need to delete them
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let mut statement = conn.prepare(&cmd).unwrap();

    // once we have the row ids, then we will delete the rows in the actual and metadata table

    let mut row_ids: Vec<u32> = Vec::new();
    let row_to_id = |rowid: u32| -> Result<u32> { Ok(rowid) };

    let ids = statement
        .query_and_then([], |row| row_to_id(row.get(0).unwrap()))
        .unwrap();

    for id in ids {
        row_ids.push(id.unwrap());
    }

    println!("{:?}", row_ids);

    let total_rows = execute_write(&conn, &original_cmd);

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

    let result = DeletePartialDataResult {
        is_successful: true,
        row_id: *deleted_row_id,
        data_hash: 0,
    };

    println!("{:?}", result);

    return result;
}
