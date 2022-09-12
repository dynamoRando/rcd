use super::DbiConfigSqlite;
use crate::table::{Column, Data, Row, Table, Value};
use log::info;
use rusqlite::{types::Type, Connection, Result};
use std::path::Path;

mod cds_types;
pub mod db;
pub mod db_part;
pub mod rcd_db;
mod sql_text;

#[allow(dead_code, unused_variables)]
/// Takes a SELECT COUNT(*) SQL statement and returns if the result is > 0. Usually used to see if a table that has been
/// created has also populated any data in it.
pub fn has_any_rows(cmd: String, conn: &Connection) -> bool {
    return total_count(cmd, conn) > 0;
}

#[allow(dead_code, unused_variables)]
/// Takes a SELECT COUNT(*) SQL statement and returns the value
pub fn total_count(cmd: String, conn: &Connection) -> u32 {
    return get_scalar_as_u32(cmd, conn);
}

#[allow(dead_code, unused_variables)]
/// Runs any SQL statement that returns a single value and attempts
/// to return the result as a u32
pub fn get_scalar_as_u32(cmd: String, conn: &Connection) -> u32 {
    // println!("get_scalar_as_u32: {:?}", cmd);

    let mut value: u32 = 0;
    let mut statement = conn.prepare(&cmd).unwrap();
    let rows = statement.query_map([], |row| row.get(0)).unwrap();

    for item in rows {
        value = item.unwrap();
    }

    return value;
}

pub fn execute_write(conn: &Connection, cmd: &str) -> usize {
    println!("{}", cmd);
    println!("{:?}", conn);
    return conn.execute(&cmd, []).unwrap();
}

pub fn execute_read_on_connection(cmd: String, conn: &Connection) -> rusqlite::Result<Table> {
    let mut statement = conn.prepare(&cmd).unwrap();
    let total_columns = statement.column_count();
    let cols = statement.columns();
    let mut table = Table::new();

    for col in cols {
        let col_idx = statement.column_index(col.name()).unwrap();
        let empty_string = String::from("");
        let col_type = match col.decl_type() {
            Some(c) => c,
            None => &empty_string,
        };

        let c = Column {
            name: col.name().to_string(),
            is_nullable: false,
            idx: col_idx,
            data_type: col_type.to_string(),
            is_primary_key: false,
        };

        info!("adding col {}", c.name);

        table.add_column(c);
    }

    // println!("execute_read_on_connection: statement: {:?}", statement);

    let mut rows = statement.query([])?;

    while let Some(row) = rows.next()? {
        let mut data_row = Row::new();

        for i in 0..total_columns {
            let dt = row.get_ref_unwrap(i).data_type();

            let string_value: String = match dt {
                Type::Blob => String::from(""),
                Type::Integer => row.get_ref_unwrap(i).as_i64().unwrap().to_string(),
                Type::Real => row.get_ref_unwrap(i).as_f64().unwrap().to_string(),
                Type::Text => row.get_ref_unwrap(i).as_str().unwrap().to_string(),
                _ => String::from(""),
            };

            let string_value = string_value;
            let col = table.get_column_by_index(i).unwrap();

            let data_item = Data {
                data_string: string_value,
                data_byte: Vec::new(),
            };

            let data_value = Value {
                data: Some(data_item),
                col: col,
            };

            data_row.add_value(data_value);
        }

        table.add_row(data_row);
    }

    return Ok(table);
}

#[allow(dead_code, unused_variables)]
pub fn has_cooperative_tables_mock(db_name: &str, cwd: &str, cmd: &str) -> bool {
    return false;
}

pub fn execute_read(db_name: &str, cmd: &str, config: DbiConfigSqlite) -> rusqlite::Result<Table> {
    let conn = get_db_conn(&config, db_name);
    let mut statement = conn.prepare(cmd).unwrap();
    let total_columns = statement.column_count();
    let cols = statement.columns();
    let mut table = Table::new();

    for col in cols {
        let col_idx = statement.column_index(col.name()).unwrap();

        let c = Column {
            name: col.name().to_string(),
            is_nullable: false,
            idx: col_idx,
            data_type: col.decl_type().unwrap().to_string(),
            is_primary_key: false,
        };

        info!("adding col {}", c.name);

        table.add_column(c);
    }

    let mut rows = statement.query([])?;

    while let Some(row) = rows.next()? {
        let mut data_row = Row::new();

        for i in 0..total_columns {
            let dt = row.get_ref_unwrap(i).data_type();

            let string_value: String = match dt {
                Type::Blob => String::from(""),
                Type::Integer => row.get_ref_unwrap(i).as_i64().unwrap().to_string(),
                Type::Real => row.get_ref_unwrap(i).as_f64().unwrap().to_string(),
                Type::Text => row.get_ref_unwrap(i).as_str().unwrap().to_string(),
                _ => String::from(""),
            };

            let string_value = string_value;
            let col = table.get_column_by_index(i).unwrap();

            let data_item = Data {
                data_string: string_value,
                data_byte: Vec::new(),
            };

            let data_value = Value {
                data: Some(data_item),
                col: col,
            };

            data_row.add_value(data_value);
        }

        table.add_row(data_row);
    }

    return Ok(table);
}

/// Runs any SQL statement that returns a single vlaue and attempts
/// to return the result as a u32
fn get_scalar_as_string(cmd: String, conn: &Connection) -> String {
    let mut value = String::from("");
    let mut statement = conn.prepare(&cmd).unwrap();
    let rows = statement.query_map([], |row| row.get(0)).unwrap();

    for item in rows {
        value = item.unwrap();
    }

    return value;
}

fn has_table(table_name: String, conn: &Connection) -> bool {
    let mut cmd = String::from(
        "SELECT count(*) AS TABLECOUNT FROM sqlite_master WHERE type='table' AND name=':table_name'",
    );
    cmd = cmd.replace(":table_name", &table_name);
    return has_any_rows(cmd, conn);
}

pub fn get_db_conn(config: &DbiConfigSqlite, db_name: &str) -> Connection {
    let db_path = Path::new(&config.root_folder).join(&db_name);
    return Connection::open(&db_path).unwrap();
}

pub fn get_db_conn_with_result(config: &DbiConfigSqlite, db_name: &str) -> Result<Connection> {
    let db_path = Path::new(&config.root_folder).join(&db_name);
    return Connection::open(&db_path);
}

pub fn execute_write_on_connection(db_name: &str, cmd: &str, config: &DbiConfigSqlite) -> usize {
    let conn = get_db_conn(&config, db_name);
    return conn.execute(&cmd, []).unwrap();
}
