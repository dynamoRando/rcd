use rusqlite::{named_params};

use crate::{dbi::{DbiConfigSqlite, sqlite::{has_table, get_db_conn, sql_text, execute_write}}, defaults};

pub fn insert_metadata_into_host_db(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    hash: u64,
    internal_participant_id: &str,
    config: DbiConfigSqlite,
) -> bool {
    let conn = get_db_conn(&config, db_name);
    let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);

    if !has_table(metadata_table_name.clone(), &conn) {
        //  need to create table
        let mut cmd = sql_text::COOP::text_create_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name.clone());
        execute_write(&conn, &cmd);
    }

    let mut cmd = sql_text::COOP::text_insert_row_metadata_table();
    cmd = cmd.replace(":table_name", &metadata_table_name.clone());
    let mut statement = conn.prepare(&cmd).unwrap();

    let rows = statement
        .execute(named_params! {":row": row_id, ":hash" : hash.to_ne_bytes(), ":pid" : internal_participant_id })
        .unwrap();

    return rows > 0;
}

pub fn delete_metadata_in_host_db(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    internal_participant_id: &str,
    config: DbiConfigSqlite,
) -> bool {
    let conn = get_db_conn(&config, db_name);
    let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);

    if !has_table(metadata_table_name.clone(), &conn) {
        //  need to create table
        let mut cmd = sql_text::COOP::text_create_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name.clone());
        execute_write(&conn, &cmd);
    }

    let mut cmd = sql_text::COOP::text_delete_row_metadata_table();
    cmd = cmd.replace(":table_name", &metadata_table_name.clone());
    let mut statement = conn.prepare(&cmd).unwrap();

    let rows = statement
        .execute(named_params! {":row": row_id, ":pid" : internal_participant_id })
        .unwrap();

    return rows > 0;
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
    let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);

    if !has_table(metadata_table_name.clone(), &conn) {
        //  need to create table
        let mut cmd = sql_text::COOP::text_create_metadata_table();
        cmd = cmd.replace(":table_name", &metadata_table_name.clone());
        execute_write(&conn, &cmd);
    }

    let mut cmd = sql_text::COOP::text_update_row_metadata_table();
    cmd = cmd.replace(":table_name", &metadata_table_name.clone());
    let mut statement = conn.prepare(&cmd).unwrap();

    let rows = statement
        .execute(named_params! {":row": row_id, ":hash" : hash.to_ne_bytes(), ":pid" : internal_participant_id })
        .unwrap();

    return rows > 0;
}
