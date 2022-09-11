use std::path::Path;

use super::{get_db_conn, get_db_conn_with_result, get_scalar_as_u32};
use crate::cdata::{ColumnSchema, Contract, TableSchema};
use crate::dbi::sqlite::execute_write;
use crate::dbi::{DbiConfigSqlite, InsertPartialDataResult};
use crate::rcd_enum::ColumnType;
#[allow(unused_imports)]
use crate::rcd_enum::{RcdGenerateContractError, RemoteDeleteBehavior};
#[allow(unused_imports)]
use crate::table::{Column, Data, Row, Table, Value};
#[allow(unused_imports)]
use crate::{
    rcd_enum::{self, LogicalStoragePolicy, RcdDbError},
    table,
};
#[allow(unused_imports)]
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
#[allow(unused_imports)]
use guid_create::GUID;
#[allow(unused_imports)]
use log::info;
#[allow(unused_imports)]
use rusqlite::types::Type;
#[allow(unused_imports)]
use rusqlite::{named_params, Connection, Error, Result};

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn insert_data_into_partial_db(
    db_name: &str,
    table_name: &str,
    cmd: &str,
    config: &DbiConfigSqlite,
) -> InsertPartialDataResult {
    let conn = get_db_conn(config, db_name);

    let mut row_id = 0;

    // need to insert the data
    // need to generate a data hash
    // need to get the row id of the data that was saved
    // http://www.sqlite.org/c3ref/last_insert_rowid.html
    // https://stackoverflow.com/questions/5867404/best-way-to-get-the-id-of-the-last-inserted-row-on-sqlite

    // hashing function in sqlite: https://www.i-programmer.info/news/84-database/10527-sqlite-317-adds-sha1-extension.html
    // would we parse the insert statement for the values to hash? maybe?
    // we also need to save the hash here locally with the data along with the row_id

    let total_rows = execute_write(&conn, cmd);
    if total_rows > 0 {
        let cmd = String::from("select last_insert_rowid()");
        row_id = get_scalar_as_u32(cmd, &conn);
    }

    let result = InsertPartialDataResult {
        is_successful: total_rows > 0,
        row_id,
        data_hash: Vec::new(),
    };

    unimplemented!();
}

#[allow(dead_code, unused_assignments, unused_variables)]
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

#[allow(dead_code, unused_assignments, unused_variables)]
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

#[allow(dead_code, unused_variables, unused_assignments)]
pub fn get_partial_db_connection(db_name: &str, cwd: &str) -> Connection {
    let mut db_part_name = db_name.replace(".db", "");
    db_part_name = db_part_name.replace(".dbpart", "");
    db_part_name = format!("{}{}", db_part_name, String::from(".dbpart"));
    let db_path = Path::new(&cwd).join(&db_part_name);
    let conn = Connection::open(&db_path).unwrap();
    return conn;
}

#[allow(dead_code, unused_variables, unused_assignments, unused_mut)]
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

        let mut col_statement = String::from("");

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

        cmd = cmd + &col_statement
    }

    cmd = cmd + " ) ";

    // println!("{}", cmd);
    // println!("{:?}", conn);

    execute_write(conn, &cmd);
}
