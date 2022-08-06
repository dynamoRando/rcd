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
use crate::sqlitedb::{execute_write_on_connection, has_any_rows};

/*
    "CREATE TABLE IF NOT EXISTS COOP_DATABASE_CONTRACT
    (
        CONTRACT_ID CHAR(36) NOT NULL,
        GENERATED_DATE_UTC DATETIME NOT NULL,
        DESCRIPTION VARCHAR(255),
        RETIRED_DATE_UTC DATETIME,
        VERSION_ID CHAR(36) NOT NULL,
        REMOTE_DELETE_BEHAVIOR INT
    );",
*/


#[allow(dead_code, unused_variables)]
pub struct DatabaseContract {
    pub contract_id: GUID,
    pub generated_date: NaiveDateTime,
    pub description: String,
    pub retired_date: Option<NaiveDateTime>,
    pub version_id: GUID,
    pub remote_delete_behavior: u32,
}

impl DatabaseContract {
    /// Marks this contract as retired in the database with today's UTC date
    #[allow(unused_variables, dead_code, unused_assignments)]
    pub fn retire(&self, conn: &Connection) {
        let mut cmd = String::from("UPDATE COOP_DATABASE_CONTRACT SET RETIRED_DATE_UTC = ':retire_date' WHERE VERSION_ID = ':vid'");
        cmd = cmd.replace(":retire_date", &Utc::now().to_string());
        cmd = cmd.replace(":vid", &self.version_id.to_string());
        execute_write_on_connection(cmd, conn);
    }

    /// Checks if the contract has a retired date or not
    pub fn is_retired(&self) -> bool {
        return !self.retired_date.is_none();
    }

    #[allow(unused_variables, dead_code, unused_assignments)]
    pub fn save(&self, conn: &Connection) {
        let mut cmd = String::from(
            "SELECT COUNT(*) TOTALCOUNT FROM COOP_DATABASE_CONTRACT WHERE VERSION_ID = ':vid'",
        );
        cmd = cmd.replace(":vid", &self.version_id.to_string());
        if has_any_rows(cmd, conn) {
            // this is an update
            if self.is_retired() {
                let mut cmd = String::from(
                    "
                UPDATE COOP_DATABASE_CONTRACT 
                SET 
                    CONTRACT_ID = ':cid',
                    GENERATED_DATE_UTC = ':gen_date',
                    DESCRIPTION = ':desc',
                    RETIRED_DATE_UTC = ':ret_date',
                    REMOTE_DELETE_BEHAVIOR = ':remote_behavior'
                WHERE
                    VERSION_ID = ':vid'",
                );
                cmd = cmd.replace("cid", &self.contract_id.to_string());
                cmd = cmd.replace(":gen_date", &self.generated_date.to_string());
                cmd = cmd.replace(":desc", &&self.description);
                let ret = &self.retired_date.unwrap().to_string();
                cmd = cmd.replace(":ret_date", ret);
                cmd = cmd.replace(":vid", &self.version_id.to_string());
                cmd = cmd.replace(":remote_behavior", &self.remote_delete_behavior.to_string());
                execute_write_on_connection(cmd, conn);
            } else {
                let mut cmd = String::from(
                    "
                UPDATE COOP_DATABASE_CONTRACT 
                SET 
                    CONTRACT_ID = ':cid',
                    GENERATED_DATE_UTC = ':gen_date',
                    DESCRIPTION = ':desc',
                    REMOTE_DELETE_BEHAVIOR = ':remote_behavior'
                WHERE
                    VERSION_ID = ':vid'",
                );
                cmd = cmd.replace("cid", &self.contract_id.to_string());
                cmd = cmd.replace(":gen_date", &self.generated_date.to_string());
                cmd = cmd.replace(":desc", &&self.description);
                cmd = cmd.replace(":vid", &self.version_id.to_string());
                cmd = cmd.replace(":remote_behavior", &self.remote_delete_behavior.to_string());
                execute_write_on_connection(cmd, conn);
            }
        } else {
            // this is an insert
            if self.is_retired() {
                let mut cmd = String::from(
                    "
                INSERT INTO COOP_DATABASE_CONTRACT
                (
                    CONTRACT_ID,
                    GENERATED_DATE_UTC,
                    DESCRIPTION,
                    RETIRED_DATE_UTC,
                    VERSION_ID,
                    REMOTE_DELETE_BEHAVIOR
                )
                VALUES
                (
                    ':cid',
                    ':gen_date',
                    ':desc',
                    ':ret_date',
                    ':vid',
                    ':remote_behavior'
                );
                ",
                );
                cmd = cmd.replace("cid", &self.contract_id.to_string());
                cmd = cmd.replace(":gen_date", &self.generated_date.to_string());
                cmd = cmd.replace(":desc", &&self.description);
                let ret = &self.retired_date.unwrap().to_string();
                cmd = cmd.replace(":ret_date", ret);
                cmd = cmd.replace(":vid", &self.version_id.to_string());
                cmd = cmd.replace(":remote_behavior", &self.remote_delete_behavior.to_string());
                execute_write_on_connection(cmd, conn);
            } else {
                let mut cmd = String::from(
                    "
                INSERT INTO COOP_DATABASE_CONTRACT
                (
                    CONTRACT_ID,
                    GENERATED_DATE_UTC,
                    DESCRIPTION,
                    VERSION_ID,
                    REMOTE_DELETE_BEHAVIOR
                )
                VALUES
                (
                    ':cid',
                    ':gen_date',
                    ':desc',
                    ':vid',
                    ':remote_behavior'
                );
                ",
                );

                cmd = cmd.replace("cid", &self.contract_id.to_string());
                cmd = cmd.replace(":gen_date", &self.generated_date.to_string());
                cmd = cmd.replace(":desc", &&self.description);
                cmd = cmd.replace(":vid", &self.version_id.to_string());
                cmd = cmd.replace(":remote_behavior", &self.remote_delete_behavior.to_string());
                execute_write_on_connection(cmd, conn);
            }
        }
    }
}