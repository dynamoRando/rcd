#[allow(unused_imports)]
use crate::table::{Column, Data, Row, Table, Value};
use crate::{rcd_enum::LogicalStoragePolicy, sql_text};
#[allow(unused_imports)]
use guid_create::GUID;
use log::info;
use rusqlite::types::Type;
#[allow(unused_imports)]
use rusqlite::{named_params, Connection, Error, Result};
use std::path::Path;

pub fn create_database(db_name: &str, cwd: &str) -> Result<Connection, Error> {
    let db_path = Path::new(&cwd).join(&db_name);
    Connection::open(&db_path)
}

pub fn execute_write(db_name: &str, cwd: &str, cmd: &str) -> usize {
    let db_path = Path::new(&cwd).join(&db_name);
    let conn = Connection::open(&db_path).unwrap();
    let mut statement = conn.prepare(cmd).unwrap();
    let total_rows = statement.execute([]).unwrap();

    return total_rows;
}

#[allow(dead_code)]
pub fn execute_read(db_name: &str, cwd: &str, cmd: &str) -> Result<Table> {
    let db_path = Path::new(&cwd).join(&db_name);
    let conn = Connection::open(&db_path)?;
    let mut statement = conn.prepare(cmd).unwrap();
    let total_columns = statement.column_count();
    let col_names = statement.column_names();
    let mut table = Table::new();

    let mut curr_idx = 0;

    for name in col_names {
        let c = Column {
            name: name.to_string(),
            is_nullable: false,
            idx: curr_idx,
        };

        curr_idx = curr_idx + 1;

        info!("adding col {}", c.name);

        table.add_column(c);
    }

    let mut rows = statement.query([])?;

    while let Some(row) = rows.next()? {
        println!("reading row..");
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

#[allow(unused_variables)]
pub fn enable_coooperative_features(db_name: &str, cwd: &str) {
    let db_path = Path::new(&cwd).join(&db_name);
    let conn = Connection::open(&db_path).unwrap();

    create_remotes_table(&conn);
    create_participant_table(&conn);
    create_contracts_table(&conn);
    create_data_host_tables(&conn);
    populate_data_host_tables(db_name, &conn);
}

#[allow(dead_code, unused_variables)]
/// Creates the COOP_REMOTES table if it does not exist. This holds
/// the logical storage policy for every table in the database.
fn create_remotes_table(conn: &Connection) {
    let cmd = String::from(
        "CREATE TABLE IF NOT EXISTS COOP_REMOTES
    (
        TABLENAME VARCHAR(255) NOT NULL,
        LOGICAL_STORAGE_POLICY INT NOT NULL
    );",
    );

    conn.execute(&cmd, []).unwrap();
}

#[allow(dead_code, unused_variables)]
/// Creates the COOP_PARTICIPANT table if it does not exist. This holds
/// the participant information that are cooperating with this database.
fn create_participant_table(conn: &Connection) {
    let cmd = String::from(
        "CREATE TABLE IF NOT EXISTS COOP_PARTICIPANT
    (
        INTERNAL_PARTICIPANT_ID CHAR(36) NOT NULL,
        ALIAS VARCHAR(50) NOT NULL,
        IP4ADDRESS VARCHAR(25),
        IP6ADDRESS VARCHAR(25),
        PORT INT,
        CONTRACT_STATUS INT,
        ACCEPTED_CONTRACT_VERSION_ID CHAR(36),
        TOKEN BLOB NOT NULL,
        PARTICIPANT_ID CHAR(36)
    );",
    );

    conn.execute(&cmd, []).unwrap();
}

#[allow(dead_code, unused_variables)]
/// Creates the COOP_DATABASE_CONTRACT table if it does not exist. This holds
/// all the contracts we have generated for this database.
fn create_contracts_table(conn: &Connection) {
    let cmd = String::from(
        "CREATE TABLE IF NOT EXISTS COOP_DATABASE_CONTRACT
        (
            CONTRACT_ID CHAR(36) NOT NULL,
            GENERATED_DATE_UTC DATETIME NOT NULL,
            DESCRIPTION VARCHAR(255),
            RETIRED_DATE_UTC DATETIME,
            VERSION_ID CHAR(36) NOT NULL,
            REMOTE_DELETE_BEHAVIOR INT
        );",
    );

    conn.execute(&cmd, []).unwrap();
}

#[allow(dead_code, unused_variables)]
/// Creates the COOP_DATA_HOST_* tables if they do not exist in the current database. These tables are used
/// to store schema information and the database_id that we send to participants of this database. This
/// data is usually contained at the participant in the database contract.
fn create_data_host_tables(conn: &Connection) {
    unimplemented!();
}

#[allow(dead_code, unused_variables)]
/// Populates the COOP_DATA_HOST_* tables with the needed information such as database_id and
/// the current database schema, if applicable.
fn populate_data_host_tables(db_name: &str, conn: &Connection) {
    populate_database_id(db_name, conn);

    unimplemented!();
}

#[allow(dead_code, unused_variables)]
/// Checks the COOP_DATA_HOST table to see if a database id has been generated and if not, creates a saves one.
/// This is the id we will use to identify this database as having cooperative tables to participants
fn populate_database_id(db_name: &str, conn: &Connection) {
    let cmd = sql_text::COOP::text_get_count_from_data_host();
    let has_database_id = has_any_rows(cmd, conn);

    if !has_database_id {
        let cmd = sql_text::COOP::text_add_database_id_to_host();
        let db_id = GUID::rand();
        let mut statement = conn.prepare(&cmd).unwrap();
        statement
            .execute(named_params! {":database_id": db_id.to_string(), ":database_name" : db_name})
            .unwrap();
    }
}

#[allow(dead_code, unused_variables)]
/// Takes a SELECT COUNT(*) SQL statement and returns if the result is > 0. Usually used to see if a table that has been
/// created has also populated any data in it.
fn has_any_rows(cmd: String, conn: &Connection) -> bool {
    let mut has_rows = false;

    let mut statement = conn.prepare(&cmd).unwrap();

    let rows = statement.query_map([], |row| row.get(0)).unwrap();

    for item in rows {
        let count: u64 = item.unwrap();
        if count > 0 {
            has_rows = true;
        }
    }

    return has_rows;
}

#[allow(dead_code, unused_variables)]
fn get_remote_status_for_tables() -> Vec<(String, LogicalStoragePolicy)> {
    let cmd = sql_text::COOP::text_get_logical_storage_policy_tables();
    unimplemented!();
}
