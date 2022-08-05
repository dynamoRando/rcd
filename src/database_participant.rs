use crate::rcd_enum::ContractStatus;
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
#[allow(unused_imports)]
use std::path::Path;
#[allow(unused_imports)]
use crate::sqlitedb::{execute_write_on_connection, has_any_rows};

#[allow(dead_code)]
pub struct DatabaseParticipant {
    pub internal_id: GUID,
    pub alias: String,
    pub ip4addr: String,
    pub ip6addr: String,
    pub db_port: u32,
    pub contract_status: ContractStatus,
    pub accepted_contract_version: GUID,
    pub token: Vec<u8>,
    pub id: GUID,
}
