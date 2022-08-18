use crate::cdata::Contract;
use crate::cdata::{DatabaseSchema, Host};
use crate::defaults;
use crate::host_info::HostInfo;
use crate::rcd_enum::ContractStatus;
#[allow(unused_imports)]
use crate::rcd_enum::{RcdGenerateContractError, RemoteDeleteBehavior};
use crate::sqlitedb::execute_read_on_connection;
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
#[derive(Clone)]
pub struct DatabaseContract {
    pub contract_id: GUID,
    pub generated_date: DateTime<Utc>,
    pub description: String,
    pub retired_date: Option<DateTime<Utc>>,
    pub version_id: GUID,
    pub remote_delete_behavior: u32,
}

impl DatabaseContract {
    pub fn to_cdata_contract(
        &self,
        host_info: &HostInfo,
        host_ip4_addr: &str,
        host_ip6_addr: &str,
        host_db_port: u32,
        contract_status: ContractStatus,
        db_schema: DatabaseSchema,
    ) -> Contract {
        let c_host_info = Host {
            host_guid: host_info.id.clone(),
            host_name: host_info.name.clone(),
            ip4_address: host_ip4_addr.to_string(),
            ip6_address: host_ip6_addr.to_string(),
            database_port_number: host_db_port,
            token: host_info.token.clone(),
        };

        let contract = Contract {
            contract_guid: String::from(""),
            description: String::from(""),
            contract_version: String::from(""),
            host_info: Some(c_host_info),
            schema: Some(db_schema),
            status: ContractStatus::to_u32(contract_status),
        };

        return contract;
    }

    pub fn get_active_contract(conn: &Connection) -> DatabaseContract {
        let cmd = String::from(
            "
            SELECT 
                CONTRACT_ID,
                GENERATED_DATE_UTC,
                DESCRIPTION,
                VERSION_ID,
                REMOTE_DELETE_BEHAVIOR 
            FROM 
                COOP_DATABASE_CONTRACT 
            WHERE 
                RETIRED IS NULL
            ;",
        );

        let row_to_active_contract = |contract_id: String,
                                      generated_date_utc: String,
                                      description: String,
                                      version_id: String,
                                      remote_delete_behavior: u32|
         -> Result<DatabaseContract> {
            let contract = DatabaseContract {
                contract_id: GUID::parse(&contract_id).unwrap(),
                generated_date: Utc::datetime_from_str(
                    &Utc,
                    &generated_date_utc,
                    defaults::DATETIME_STRING_FORMAT,
                )
                .unwrap(),
                description: description,
                version_id: GUID::parse(&version_id).unwrap(),
                remote_delete_behavior: remote_delete_behavior,
                retired_date: None,
            };

            Ok(contract)
        };

        let mut results: Vec<DatabaseContract> = Vec::new();

        let mut statement = conn.prepare(&cmd).unwrap();
        let contracts = statement
            .query_and_then([], |row| {
                row_to_active_contract(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                )
            })
            .unwrap();

        for contract in contracts {
            results.push(contract.unwrap());
        }

        return results.first().unwrap().clone();
    }

    pub fn get_all(conn: &Connection) -> Vec<DatabaseContract> {
        let mut result: Vec<DatabaseContract> = Vec::new();

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

        let cmd = String::from(
            "SELECT 
            CONTRACT_ID,
            GENERATED_DATE_UTC,
            DESCRIPTION,
            RETIRED_DATE_UTC,
            VERSION_ID,
            REMOTE_DELETE_BEHAVIOR
        FROM
            COOP_DATABASE_CONTRACT
            ;
            ",
        );

        let table = execute_read_on_connection(cmd, conn).unwrap();

        for row in table.rows {
            for val in &row.vals {
                let mut cid = GUID::rand();
                let mut gen_date = Utc::now();
                let mut desc = String::from("");
                let mut is_retired = false;
                let mut ret_date = Utc::now();
                let mut vid = GUID::rand();
                let mut delete_behavior: u32 = 0;

                if val.col.name == "CONTRACT_ID" {
                    let vcid = val.data.as_ref().unwrap().data_string.clone();
                    let tcid = GUID::parse(&vcid).unwrap();
                    cid = tcid;
                }

                if val.col.name == "GENERATED_DATE_UTC" {
                    let vgen_date = val.data.as_ref().unwrap().data_string.clone();
                    // println!("{}", vgen_date);
                    let tgen_date =
                        Utc::datetime_from_str(&Utc, &vgen_date, defaults::DATETIME_STRING_FORMAT);
                    gen_date = tgen_date.unwrap();
                }

                if val.col.name == "DESCRIPTION" {
                    let vdesc = val.data.as_ref().unwrap().data_string.clone();
                    desc = vdesc.clone();
                }

                if val.col.name == "RETIRED_DATE_UTC" {
                    if val.is_null() {
                        is_retired = false;
                    } else {
                        let vret_date = val.data.as_ref().unwrap().data_string.clone();
                        if vret_date.len() > 0 {
                            // println!("{}", vret_date);
                            let tret_date = Utc::datetime_from_str(
                                &Utc,
                                &vret_date,
                                defaults::DATETIME_STRING_FORMAT,
                            );
                            ret_date = tret_date.unwrap();
                            is_retired = true;
                        } else {
                            is_retired = false;
                        }
                    }
                }

                if val.col.name == "VERSION_ID" {
                    let vvid = val.data.as_ref().unwrap().data_string.clone();
                    let tvid = GUID::parse(&vvid).unwrap();
                    vid = tvid;
                }

                if val.col.name == "REMOTE_DELETE_BEHAVIOR" {
                    let vbehavior = val.data.as_ref().unwrap().data_string.clone();
                    delete_behavior = vbehavior.parse().unwrap();
                }

                let item = DatabaseContract {
                    contract_id: cid,
                    generated_date: gen_date,
                    description: desc,
                    retired_date: if is_retired { Some(ret_date) } else { None },
                    version_id: vid,
                    remote_delete_behavior: delete_behavior,
                };

                result.push(item);
            }
        }

        return result;
    }

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
                cmd = cmd.replace(":cid", &self.contract_id.to_string());
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
                cmd = cmd.replace(":cid", &self.contract_id.to_string());
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
                cmd = cmd.replace(":cid", &self.contract_id.to_string());
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

                cmd = cmd.replace(":cid", &self.contract_id.to_string());
                println!("{}", &self.generated_date);
                cmd = cmd.replace(":gen_date", &self.generated_date.to_string());
                cmd = cmd.replace(":desc", &&self.description);
                cmd = cmd.replace(":vid", &self.version_id.to_string());
                cmd = cmd.replace(":remote_behavior", &self.remote_delete_behavior.to_string());
                execute_write_on_connection(cmd, conn);
            }
        }
    }
}
