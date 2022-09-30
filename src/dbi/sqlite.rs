use self::db_part::get_partial_db_connection;

use super::DbiConfigSqlite;
use crate::{
    cdata::{ColumnSchema, RowValue},
    defaults,
    rcd_enum::ColumnType,
    table::{Column, Data, Table, Value},
};
use log::info;
use rusqlite::{types::Type, Connection, Result};
use std::path::Path;
pub mod db;
pub mod db_part;
pub mod rcd_db;
mod sql_text;

/// Takes a SELECT COUNT(*) SQL statement and returns if the result is > 0. Usually used to see if a table that has been
/// created has also populated any data in it.
pub fn has_any_rows(cmd: String, conn: &Connection) -> bool {
    return total_count(cmd, conn) > 0;
}

/// Takes a SELECT COUNT(*) SQL statement and returns the value
pub fn total_count(cmd: String, conn: &Connection) -> u32 {
    return get_scalar_as_u32(cmd, conn);
}

#[allow(dead_code)]
pub fn get_scalar_as_vec_u8(cmd: String, conn: &Connection) -> Option<Vec<u8>> {
    let mut statement = conn.prepare(&cmd).unwrap();
    let mut returned_arrays: Vec<Vec<u8>> = Vec::new();

    let row_to_token = |data: Vec<u8>| -> Result<Vec<u8>> { Ok(data) };

    let tokens = statement
        .query_and_then([], |row| row_to_token(row.get(0).unwrap()))
        .unwrap();

    for t in tokens {
        returned_arrays.push(t.unwrap());
    }

    if returned_arrays.len() >= 1 {
        return Some(returned_arrays.first().unwrap().clone());
    } else {
        return None;
    }
}

pub fn get_scalar_as_u64(cmd: String, conn: &Connection) -> Option<u64> {
    println!("{:?}", cmd);
    println!("{:?}", conn);

    let mut statement = conn.prepare(&cmd).unwrap();
    let mut returned_arrays: Vec<Vec<u8>> = Vec::new();

    let row_to_token = |data: Vec<u8>| -> Result<Vec<u8>> { Ok(data) };

    let tokens = statement
        .query_and_then([], |row| row_to_token(row.get(0).unwrap()))
        .unwrap();

    for t in tokens {
        returned_arrays.push(t.unwrap());
    }

    if returned_arrays.len() >= 1 {
        let array = returned_arrays.first().unwrap().clone();
        let value: u64 = u64::from_ne_bytes(vec_to_array(array));
        return Some(value);
    } else {
        return None;
    }
}

/// Runs any SQL statement that returns a single value and attempts
/// to return the result as a u32
pub fn get_scalar_as_u32(cmd: String, conn: &Connection) -> u32 {
    println!("get_scalar_as_u32: {:?}", cmd);
    println!("get_scalar_as_u32: {:?}", conn);

    let mut value: u32 = 0;
    let mut statement = conn.prepare(&cmd).unwrap();
    let rows = statement.query_map([], |row| row.get(0)).unwrap();

    for item in rows {
        value = item.unwrap();
    }

    return value;
}

#[allow(dead_code, unused_variables)]
pub fn get_scalar_as_bool(cmd: String, conn: &Connection) -> bool {
    println!("get_scalar_as_bool: {:?}", cmd);

    let mut value: bool = false;
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

pub fn execute_read_on_connection_for_row(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    cmd: String,
    conn: &Connection,
) -> Result<crate::cdata::Row> {
    let mut statement = conn.prepare(&cmd).unwrap();
    let total_columns = statement.column_count();
    let cols = statement.columns();

    let mut values: Vec<crate::cdata::RowValue> = Vec::new();
    let mut columns: Vec<crate::cdata::ColumnSchema> = Vec::new();

    for col in cols {
        let col_idx = statement.column_index(col.name()).unwrap();
        let empty_string = String::from("");
        let col_type = match col.decl_type() {
            Some(c) => c,
            None => &empty_string,
        };

        let c = ColumnSchema {
            column_name: col.name().to_string(),
            column_type: ColumnType::to_u32(ColumnType::try_parse(col_type).unwrap()),
            column_length: 0,
            is_nullable: false,
            ordinal: col_idx as u32,
            table_id: String::from(""),
            column_id: col_idx.to_string(),
            is_primary_key: false,
        };
        columns.push(c);
    }

    let mut rows = statement.query([])?;

    while let Some(row) = rows.next()? {
        for i in 0..total_columns {
            let dt = row.get_ref_unwrap(i).data_type();
            let string_value: String = match dt {
                Type::Blob => String::from(""),
                Type::Integer => row.get_ref_unwrap(i).as_i64().unwrap().to_string(),
                Type::Real => row.get_ref_unwrap(i).as_f64().unwrap().to_string(),
                Type::Text => row.get_ref_unwrap(i).as_str().unwrap().to_string(),
                _ => String::from(""),
            };

            let mut row_value = RowValue {
                column: None,
                is_null_value: false,
                value: Vec::new(),
                string_value: String::from(""),
            };

            let column = columns
                .iter()
                .enumerate()
                .filter(|&(_, c)| c.ordinal == i as u32)
                .map(|(_, c)| c);

            let col = column.last().unwrap().clone();
            row_value.column = Some(col);
            row_value.value = string_value.as_bytes().to_vec();
            row_value.string_value = string_value.clone();
            values.push(row_value);
        }
    }

    let mut cmd = String::from("SELECT HASH FROM :table_name WHERE ROW_ID = :rid");
    let metadata_table_name = format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);
    cmd = cmd.replace(":table_name", &metadata_table_name);
    cmd = cmd.replace(":rid", &row_id.to_string());

    let mut statement = conn.prepare(&cmd).unwrap();

    let row_to_hash = |hash: Vec<u8>| -> Result<Vec<u8>> { Ok(hash) };

    let hashes = statement
        .query_and_then([], |row| row_to_hash(row.get(0).unwrap()))
        .unwrap();

    let mut hash: Vec<u8> = Vec::new();

    for h in hashes {
        hash = h.unwrap();
        break;
    }

    let result = crate::cdata::Row {
        database_name: db_name.to_string(),
        table_name: table_name.to_string(),
        row_id: row_id,
        values: values,
        is_remoteable: true,
        remote_metadata: None,
        hash: hash,
    };

    return Ok(result);
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
        let mut data_row = crate::table::Row::new();

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

pub fn execute_read_at_participant(
    db_name: &str,
    cmd: &str,
    config: DbiConfigSqlite,
) -> rusqlite::Result<Table> {
    let conn = get_partial_db_connection(&db_name, &config.root_folder);
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
        let mut data_row = crate::table::Row::new();

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

pub fn execute_read_at_host(
    db_name: &str,
    cmd: &str,
    config: DbiConfigSqlite,
) -> rusqlite::Result<Table> {
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
        let mut data_row = crate::table::Row::new();

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

pub fn execute_write_on_connection_at_host(
    db_name: &str,
    cmd: &str,
    config: &DbiConfigSqlite,
) -> usize {
    let conn = get_db_conn(&config, db_name);

    // println!("{:?}", conn);
    // println!("{:?}", cmd);

    return conn.execute(&cmd, []).unwrap();
}

pub fn execute_write_on_connection_at_participant(
    db_name: &str,
    cmd: &str,
    config: &DbiConfigSqlite,
) -> usize {
    let conn = get_partial_db_connection(&db_name, &config.root_folder);
    return conn.execute(&cmd, []).unwrap();
}

pub fn get_table_col_names_with_data_type_as_string(
    db_name: &str,
    table_name: &str,
    config: &DbiConfigSqlite,
) -> String {
    let pdbc = get_partial_db_connection(db_name, &config.root_folder);
    let table = get_schema_of_table(table_name.to_string(), &pdbc).unwrap();

    let mut col_names = String::from("");

    for row in &table.rows {
        let col_name = row.vals[1].data.as_ref().unwrap().data_string.clone();
        let data_type = row.vals[2].data.as_ref().unwrap().data_string.clone();
        col_names = format!("{} {} {}{}", col_names, col_name, data_type, ",");
    }

    // println!("{:?}", col_names);

    let result: &str = &col_names[1..col_names.len() - 1];

    // println!("{:?}", result);

    return result.to_string();
}

/// Returns a table describing the schema of the table
/// # Columns:
/// 1. columnId
/// 2. name
/// 3. type
/// 4. NotNull
/// 5. defaultValue
/// 6. IsPK
pub fn get_schema_of_table(table_name: String, conn: &Connection) -> Result<Table> {
    let mut cmd = String::from("PRAGMA table_info(\":table_name\")");
    cmd = cmd.replace(":table_name", &table_name);

    return Ok(execute_read_on_connection(cmd, conn).unwrap());
}

pub fn get_table_col_names(table_name: String, conn: &Connection) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut cmd = String::from("select NAME from pragma_table_info(\":table_name\") as tblInfo;");
    cmd = cmd.replace(":table_name", &table_name);

    let row_to_string = |column_name: String| -> Result<String> { Ok(column_name) };

    let mut statement = conn.prepare(&cmd).unwrap();

    let names = statement
        .query_and_then([], |row| row_to_string(row.get(0).unwrap()))
        .unwrap();

    for name in names {
        result.push(name.unwrap());
    }

    return result;
}

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
