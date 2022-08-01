#[allow(unused_imports)]
use crate::rcd_enum::{RcdGenerateContractError, RemoteDeleteBehavior};
#[allow(unused_imports)]
use crate::table::{Column, Data, Row, Table, Value};
#[allow(unused_imports)]
use crate::{
    rcd_enum::{self, LogicalStoragePolicy, RcdDbError},
    sql_text, table,
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
use std::path::Path;

#[allow(dead_code, unused_assignments, unused_variables)]
pub fn create_partial_database(db_name: &str, cwd: &str) -> Result<Connection, Error> {
    let mut db_part_name = db_name.replace(".db", "");
    db_part_name = db_part_name.replace(".dbpart", "");
    db_part_name = format!("{}{}", db_name, String::from(".dbpart"));
    let db_path = Path::new(&cwd).join(&db_part_name);
    Connection::open(&db_path)
}