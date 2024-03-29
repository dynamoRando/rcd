use self::db_part::get_partial_db_connection;
use rcd_common::{db::DbiConfigSqlite, defaults, table::*};
use rcd_enum::column_type::ColumnType;
use rcd_error::rcd_db_error::RcdDbError;
use rcd_sqlite_log::{log_entry::LogEntry, SqliteLog};
use rcdproto::rcdp::{ColumnSchema, RowValue};
use rusqlite::{types::Type, Connection, Result};
use std::path::Path;
use tracing::{debug, error, info, trace, warn};
pub mod db;
pub mod db_part;
pub mod rcd_db;
mod sql_text;
use stdext::function_name;
use tracing::instrument;

pub fn get_last_log_entries(number_of_entries: u32, config: &DbiConfigSqlite) -> Vec<LogEntry> {
    SqliteLog::get_last_x_logs(number_of_entries, &config.root_folder)
}

/// Takes a SELECT COUNT(*) SQL statement and returns if the result is > 0. Usually used to see if a table that has been
/// created has also populated any data in it.
pub fn has_any_rows(cmd: String, conn: &Connection) -> bool {
    total_count(cmd, conn) > 0
}

/// Takes a SELECT COUNT(*) SQL statement and returns the value
pub fn total_count(cmd: String, conn: &Connection) -> u32 {
    get_scalar_as_u32(cmd, conn)
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

    if !returned_arrays.is_empty() {
        Some(returned_arrays.first().unwrap().clone())
    } else {
        None
    }
}

pub fn get_scalar_as_u64(cmd: String, conn: &Connection) -> Option<u64> {
    trace!("[{}]: {cmd:?} {conn:?}", function_name!());

    let mut statement = conn.prepare(&cmd).unwrap();
    let mut returned_arrays: Vec<Vec<u8>> = Vec::new();

    let row_to_token = |data: Vec<u8>| -> Result<Vec<u8>> { Ok(data) };

    let tokens = statement
        .query_and_then([], |row| row_to_token(row.get(0).unwrap()))
        .unwrap();

    for t in tokens {
        returned_arrays.push(t.unwrap());
    }

    if !returned_arrays.is_empty() {
        let array = returned_arrays.first().unwrap().clone();
        let value: u64 = u64::from_ne_bytes(vec_to_array(array));
        Some(value)
    } else {
        None
    }
}

/// Runs any SQL statement that returns a single value and attempts
/// to return the result as a u32
pub fn get_scalar_as_u32(cmd: String, conn: &Connection) -> u32 {
    trace!("[{}]: {cmd:?} {conn:?}", function_name!());

    let mut value: u32 = 0;
    let mut statement = conn.prepare(&cmd).unwrap();
    let rows = statement.query_map([], |row| row.get(0)).unwrap();

    for item in rows {
        value = item.unwrap_or_default();
    }

    drop(statement);

    value
}

pub fn get_scalar_as_bool(cmd: String, conn: &Connection) -> bool {
    trace!("get_scalar_as_bool: {cmd:?}");

    let mut value: bool = false;
    let mut statement = conn.prepare(&cmd).unwrap();
    let rows = statement.query_map([], |row| row.get(0)).unwrap();

    for item in rows {
        value = item.unwrap();
    }

    drop(statement);

    value
}

pub fn execute_write(conn: &Connection, cmd: &str) -> usize {
    trace!("[{}]: {cmd:?} {conn:?}", function_name!());
    let result = conn.execute(cmd, []);
    match result {
        Ok(rows) => rows,
        Err(e) => {
            error!("[{}]: {e:?}", function_name!());
            warn!(
                "[{}]: this function should return a Result instead.",
                function_name!()
            );
            0
        }
    }
}

pub fn execute_read_on_connection_for_row(
    db_name: &str,
    table_name: &str,
    row_id: u32,
    cmd: String,
    conn: &Connection,
) -> Result<rcdproto::rcdp::Row> {
    let mut statement = conn.prepare(&cmd).unwrap();
    let total_columns = statement.column_count();
    let cols = statement.columns();

    let mut values: Vec<rcdproto::rcdp::RowValue> = Vec::new();
    let mut columns: Vec<rcdproto::rcdp::ColumnSchema> = Vec::new();

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

    trace!("[{}]: {statement:?}", function_name!());

    let row_to_hash = |hash: Vec<u8>| -> Result<Vec<u8>> { Ok(hash) };

    let mut hashes = statement
        .query_and_then([], |row| row_to_hash(row.get(0).unwrap()))
        .unwrap();

    let mut hash: Vec<u8> = Vec::new();

    if let Some(h) = hashes.next() {
        hash = h.unwrap()
    }

    /*
    for h in hashes {
        hash = h.unwrap();
        break;
    }
    */

    let result = rcdproto::rcdp::Row {
        database_name: db_name.to_string(),
        table_name: table_name.to_string(),
        row_id,
        values,
        is_remoteable: true,
        remote_metadata: None,
        hash,
    };

    trace!("[{}]: {result:?}", function_name!());

    Ok(result)
}

pub fn execute_read(cmd: &str, conn: &Connection) -> Result<Table, RcdDbError> {
    let mut statement = conn.prepare(cmd)?;
    let total_columns = statement.column_count();
    let cols = statement.columns();
    let mut table = Table::new();

    trace!("[{}]: {:?}", function_name!(), cmd);

    for col in cols {
        let col_idx = statement.column_index(col.name())?;

        trace!("[{}]: {col:?}", function_name!());
        let mut data_type = String::from("");

        let col_type = col.decl_type();
        if let Some(dt) = col_type {
            data_type = dt.to_string();
        };

        let c = Column {
            name: col.name().to_string(),
            is_nullable: false,
            idx: col_idx,
            data_type,
            is_primary_key: false,
        };

        trace!("[{}]: adding col {}", function_name!(), c.name);

        table.add_column(c);
    }

    let mut rows = statement.query([])?;

    while let Some(row) = rows.next()? {
        let mut data_row = rcd_common::table::Row::new();

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
                col,
            };

            data_row.add_value(data_value);
        }

        table.add_row(data_row);
    }

    Ok(table)
}

pub fn execute_read_at_participant(
    db_name: &str,
    cmd: &str,
    config: &DbiConfigSqlite,
) -> Result<Table, RcdDbError> {
    if !has_database(config, db_name) {
        return Err(RcdDbError::DbNotFound(db_name.to_string()));
    }

    let conn = get_partial_db_connection(db_name, &config.root_folder);
    execute_read(cmd, &conn)
}

pub fn execute_read_at_host(
    db_name: &str,
    cmd: &str,
    config: DbiConfigSqlite,
) -> core::result::Result<Table, RcdDbError> {
    if !has_database(&config, db_name) {
        return Err(RcdDbError::DbNotFound(db_name.to_string()));
    }

    let conn = get_db_conn(&config, db_name);
    execute_read(cmd, &conn)
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

    drop(statement);

    value
}

fn has_table(table_name: &str, conn: &Connection) -> bool {
    let mut cmd = String::from(
        "SELECT count(*) AS TABLECOUNT FROM sqlite_master WHERE type='table' AND name=':table_name'",
    );
    cmd = cmd.replace(":table_name", table_name);
    has_any_rows(cmd, conn)
}

#[instrument]
pub fn has_database(config: &DbiConfigSqlite, db_name: &str) -> bool {
    let mut db_exists_as_regular_db = false;
    let mut db_exists_as_partial_db = false;

    if !db_name.ends_with(".db") {
        let db = db_name.to_owned() + ".db";
        let path = Path::new(&config.root_folder).join(db);
        db_exists_as_regular_db = Path::exists(&path);
    }

    if !db_name.ends_with(".dbpart") {
        let db = db_name.to_owned() + ".dbpart";
        let path = Path::new(&config.root_folder).join(db);
        db_exists_as_partial_db = Path::exists(&path);

        if !db_exists_as_partial_db && db_name.ends_with(".db") {
            let mut db_part_name = db_name.replace(".db", "");
            db_part_name = db_part_name.replace(".dbpart", "");
            db_part_name = format!("{}{}", db_part_name, String::from(".dbpart"));
            let path = Path::new(&config.root_folder).join(db_part_name);
            db_exists_as_partial_db = Path::exists(&path);
        }
    }

    let path = Path::new(&config.root_folder).join(db_name);
    let db_exists: bool = Path::exists(&path);

    db_exists || db_exists_as_regular_db || db_exists_as_partial_db
}

pub fn get_db_conn(config: &DbiConfigSqlite, db_name: &str) -> Connection {
    let db_path = Path::new(&config.root_folder).join(db_name);
    trace!("[{}]: {db_path:?}", function_name!());
    Connection::open(db_path).unwrap()
}

pub fn get_db_conn_with_result(config: &DbiConfigSqlite, db_name: &str) -> Result<Connection> {
    let db_path = Path::new(&config.root_folder).join(db_name);
    Connection::open(db_path)
}

pub fn execute_write_on_connection_at_host(
    db_name: &str,
    cmd: &str,
    config: &DbiConfigSqlite,
) -> Result<usize, RcdDbError> {
    if !has_database(config, db_name) {
        return Err(RcdDbError::DbNotFound(db_name.to_string()));
    }

    let conn = get_db_conn(config, db_name);

    trace!("[{}]: {conn:?} {cmd:?}", function_name!());

    let result = conn.execute(cmd, []);

    let _ = conn.close();

    match result {
        Ok(x) => Ok(x),
        Err(e) => Err(RcdDbError::General(e.to_string())),
    }
}

pub fn execute_write_on_connection_at_participant(
    db_name: &str,
    cmd: &str,
    config: &DbiConfigSqlite,
) -> usize {
    let conn = get_partial_db_connection(db_name, &config.root_folder);
    let result = conn.execute(cmd, []).unwrap();
    let _ = conn.close();

    result
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

    trace!("[{}]: {col_names:?}", function_name!());

    let result: &str = &col_names[1..col_names.len() - 1];

    trace!("[{}]: {result:?}", function_name!());

    result.to_string()
}

/// Returns a table describing the schema of the table
/// # Columns:
/// 1. columnId
/// 2. name
/// 3. type
/// 4. NotNull
/// 5. defaultValue
/// 6. IsPK
pub fn get_schema_of_table(
    table_name: String,
    conn: &Connection,
) -> core::result::Result<Table, RcdDbError> {
    let mut cmd = String::from("PRAGMA table_info(\":table_name\")");
    cmd = cmd.replace(":table_name", &table_name);
    execute_read(&cmd, conn)
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

    drop(statement);

    result
}

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
