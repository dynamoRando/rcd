use crate::sqlite::{
    db_part::get_partial_db_connection, execute_write, get_scalar_as_u32, has_table, sql_text,
};
use tracing::debug;
use rcd_common::db::*;
use rcd_common::{
    crypt,
    db::{DbiConfigSqlite, PartialDataResult},
};
use rcd_enum::{database_type::DatabaseType, partial_data_result_action::PartialDataResultAction};
use rusqlite::named_params;

pub fn insert_data_into_partial_db(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    config: &DbiConfigSqlite,
) -> PartialDataResult {
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let mut row_id = 0;

    let total_rows = execute_write(&conn, cmd);
    if total_rows > 0 {
        let cmd = String::from("select last_insert_rowid()");
        row_id = get_scalar_as_u32(cmd, &conn);
    }

    // we need to parse the values of this row
    // and create a data hash for it
    let insert_values =
        rcd_query::query_parser::get_values_from_insert_statement(cmd, DatabaseType::Sqlite);
    let hash_value = crypt::calculate_hash_for_struct(&insert_values);

    // we need to determine if there is a metadata table for this table or not
    // and if there is not one, create it
    // then we need to save the data hash along with the row id
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

    debug!("{row_id:?}");
    debug!("{hash_value:?}");

    statement
        .execute(named_params! {":row": row_id, ":hash" : hash_value.to_ne_bytes() })
        .unwrap();

    PartialDataResult {
        is_successful: total_rows > 0,
        row_id,
        data_hash: Some(hash_value),
        partial_data_status: None,
        action: Some(PartialDataResultAction::Insert),
    }
}
