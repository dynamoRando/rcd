use crate::cdata::Contract;
use crate::cdata::{DatabaseSchema, Host};
use crate::host_info::HostInfo;
use crate::rcd_enum::ContractStatus;
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
pub struct CoopDatabaseContract {
    pub contract_id: GUID,
    pub generated_date: DateTime<Utc>,
    pub description: String,
    pub retired_date: Option<DateTime<Utc>>,
    pub version_id: GUID,
    pub remote_delete_behavior: u32,
}

impl CoopDatabaseContract {
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
            contract_guid: self.contract_id.to_string(),
            description: self.description.clone(),
            contract_version: self.version_id.to_string(),
            host_info: Some(c_host_info),
            schema: Some(db_schema),
            status: ContractStatus::to_u32(contract_status),
        };

        return contract;
    }

    /// Checks if the contract has a retired date or not
    pub fn is_retired(&self) -> bool {
        return !self.retired_date.is_none();
    }
}
