use crate::rcd_enum::ContractStatus;
#[allow(unused_imports)]
use crate::rcd_enum::{RcdGenerateContractError, RemoteDeleteBehavior};
#[allow(unused_imports)]
use crate::sqlitedb::{execute_write_on_connection, has_any_rows};
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

/*
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
*/

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

impl DatabaseParticipant {
    pub fn exists(alias: &str, conn: &Connection) -> bool {
        let mut cmd =
            String::from("SELECT COUNT(*) TOTALCOUNT FROM COOP_PARTICIPANT WHERE ALIAS = ':alias'");
        cmd = cmd.replace(":alias", alias);
        return has_any_rows(cmd, conn);
    }

    #[allow(dead_code, unused_variables)]
    pub fn save(&self, conn: &Connection) {
        if Self::exists(&self.alias, conn) {
            // this is an update
            let cmd = String::from(
                "
            UPDATE COOP_PARTICIPANT
            SET
                IP4ADDRESS = ':ip4addr',
                IP6ADDRESS = ':ip6addr',
                PORT = ':db_port',
                CONTRACT_STATUS = ':contract_status',
                ACCEPTED_CONTRACT_VERSION_ID = ':accepted_contract_version',
                TOKEN = ':token',
                PARTICIPANT_ID = ':id'
            WHERE
                ALIAS = ':alias'
            ;
            ",
            );

            let mut statement = conn.prepare(&cmd).unwrap();
            statement
                .execute(named_params! {
                        ":ip4addr": &self.ip4addr,
                        ":ip6addr": &self.ip6addr,
                        ":db_port": &self.db_port.to_string(),
                        ":contract_status": &ContractStatus::to_u32(self.contract_status),
                        ":accepted_contract_version": &self.accepted_contract_version.to_string(),
                        ":token": &self.token,
                        ":id": &self.id.to_string(),
                        ":alias": &self.alias,
                })
                .unwrap();
        } else {
            // this is an insert
            let cmd = String::from(
                "
            INSERT INTO COOP_PARTICIPANT
            (
                INTERNAL_PARTICIPANT_ID,
                ALIAS,
                IP4ADDRESS,
                IP6ADDRESS,
                PORT,
                CONTRACT_STATUS,
                ACCEPTED_CONTRACT_VERSION_ID,
                TOKEN,
                PARTICIPANT_ID
            )
            VALUES
            (
                ':internal_id',
                ':alias',
                ':ip4addr',
                ':ip6addr',
                ':db_port',
                ':contract_status',
                ':accepted_contract_version',
                ':token',
                ':id'
            );
            ",
            );

            let mut statement = conn.prepare(&cmd).unwrap();
            statement
                .execute(named_params! {
                        ":internal_id": &self.internal_id.to_string(),
                        ":alias": &self.alias,
                        ":ip4addr": &self.ip4addr,
                        ":ip6addr": &self.ip6addr,
                        ":db_port": &self.db_port.to_string(),
                        ":contract_status": &ContractStatus::to_u32(self.contract_status),
                        ":accepted_contract_version": &self.accepted_contract_version.to_string(),
                        ":token": &self.token,
                        ":id": &self.id.to_string()
                })
                .unwrap();
        }
    }
}
