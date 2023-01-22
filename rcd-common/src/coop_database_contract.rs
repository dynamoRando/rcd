use crate::host_info::HostInfo;
use chrono::{DateTime, Utc};
use guid_create::GUID;
use rcd_enum::contract_status::ContractStatus;
use rcdproto::rcdp::{Contract, DatabaseSchema, Host};

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
        host_http_addr: &str,
        host_http_port: u32,
    ) -> Contract {
        let c_host_info = Host {
            host_guid: host_info.id.clone(),
            host_name: host_info.name.clone(),
            ip4_address: host_ip4_addr.to_string(),
            ip6_address: host_ip6_addr.to_string(),
            database_port_number: host_db_port,
            token: host_info.token.clone(),
            http_addr: host_http_addr.to_string(),
            http_port: host_http_port,
        };

        Contract {
            contract_guid: self.contract_id.to_string(),
            description: self.description.clone(),
            contract_version: self.version_id.to_string(),
            host_info: Some(c_host_info),
            schema: Some(db_schema),
            status: ContractStatus::to_u32(contract_status),
        }
    }

    /// Checks if the contract has a retired date or not
    pub fn is_retired(&self) -> bool {
        self.retired_date.is_some()
    }
}
