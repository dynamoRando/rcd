use rusqlite::named_params;

use crate::{
    crypt,
    dbi::{
        get_metadata_table_name,
        sqlite::{
            db_part::get_partial_db_connection, execute_write, get_scalar_as_u32, has_table,
            sql_text,
        },
        DbiConfigSqlite, PartialDataResult,
    },
    query_parser,
    
};

use rcd_common::rcd_enum::{DatabaseType, PartialDataResultAction};

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

    let result = PartialDataResult {
        is_successful: total_rows > 0,
        row_id,
        data_hash: Some(hash_value),
        partial_data_status: None,
        action: Some(PartialDataResultAction::Insert),
    };

    return result;
}
