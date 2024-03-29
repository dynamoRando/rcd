use crate::sqlite::{execute_write, get_db_conn, get_scalar_as_u64, has_table, sql_text};
use stdext::function_name;
use tracing::trace;
use rcd_common::db::{get_metadata_table_name, DbiConfigSqlite};
use rusqlite::named_params;

pub fn get_data_hash_at_host(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    config: &DbiConfigSqlite,
) -> u64 {
    let conn = get_db_conn(config, db_name);
    let metadata_table_name = get_metadata_table_name(table_name);
    let mut cmd = String::from("SELECT HASH FROM :metadata WHERE ROW_ID = :row_id");
    cmd = cmd.replace(":metadata", &metadata_table_name);
    cmd = cmd.replace(":row_id", &row_id.to_string());

    get_scalar_as_u64(cmd, &conn).unwrap()
}

pub fn remove_remote_row_reference_from_host(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    config: &DbiConfigSqlite,
) -> bool {
    let conn = get_db_conn(config, db_name);
    let metadata_table_name = get_metadata_table_name(table_name);

    let mut cmd = String::from(
        "DELETE FROM :table_name
         WHERE ROW_ID = :rid
    ;",
    );

    trace!("{cmd}");

    cmd = cmd.replace(":table_name", &metadata_table_name);

    let mut statement = conn.prepare(&cmd).unwrap();

    let rows = statement.execute(named_params! {":rid": row_id}).unwrap();

    trace!("total row_references_deleted: {rows}");

    rows > 0
}

pub fn insert_metadata_into_host_db(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    hash: u64,
    internal_participant_id: &str,
    config: DbiConfigSqlite,
) -> bool {
    let conn = get_db_conn(&config, db_name);
    let metadata_table_name = get_metadata_table_name(table_name);

    if !has_table(&metadata_table_name, &conn) {
        //  need to create table
        let mut cmd = sql_text::Coop::text_create_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name);
        execute_write(&conn, &cmd);
    }

    let mut cmd = sql_text::Coop::text_insert_row_metadata_table();
    cmd = cmd.replace(":table_name", &metadata_table_name);
    let mut statement = conn.prepare(&cmd).unwrap();

    let rows = statement
        .execute(named_params! {":row": row_id, ":hash" : hash.to_ne_bytes(), ":pid" : internal_participant_id })
        .unwrap();

    rows > 0
}

pub fn delete_metadata_in_host_db(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    internal_participant_id: &str,
    config: DbiConfigSqlite,
) -> bool {
    let conn = get_db_conn(&config, db_name);
    let metadata_table_name = get_metadata_table_name(table_name);

    if !has_table(&metadata_table_name, &conn) {
        //  need to create table
        let mut cmd = sql_text::Coop::text_create_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name);
        execute_write(&conn, &cmd);
    }

    let mut cmd = sql_text::Coop::text_delete_row_metadata_table();
    cmd = cmd.replace(":table_name", &metadata_table_name);
    let mut statement = conn.prepare(&cmd).unwrap();

    trace!("[{}]: statement: {statement:?}", function_name!());

    let rows = statement
        .execute(named_params! {":row": row_id, ":pid" : internal_participant_id })
        .unwrap();

    trace!("[{}]: rows affected: {rows:?}", function_name!());
    
    rows > 0
}

pub fn update_metadata_in_host_db(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    hash: u64,
    internal_participant_id: &str,
    config: DbiConfigSqlite,
) -> bool {
    let conn = get_db_conn(&config, db_name);
    let metadata_table_name = get_metadata_table_name(table_name);

    if !has_table(&metadata_table_name, &conn) {
        //  need to create table
        let mut cmd = sql_text::Coop::text_create_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name);
        execute_write(&conn, &cmd);
    }

    let mut cmd = sql_text::Coop::text_update_row_metadata_table();
    cmd = cmd.replace(":table_name", &metadata_table_name);
    let mut statement = conn.prepare(&cmd).unwrap();

    let rows = statement
        .execute(named_params! {":row": row_id, ":hash" : hash.to_ne_bytes(), ":pid" : internal_participant_id })
        .unwrap();

    rows > 0
}
