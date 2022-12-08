use crate::{defaults, rcd_enum::*};

// objects moved to here from rcdx.dbi

#[derive(Clone, Debug)]
pub struct CdsContracts {
    pub host_id: String,
    pub contract_id: String,
    pub contract_version_id: String,
    pub database_name: String,
    pub database_id: String,
    pub description: String,
    pub generated_date: String,
    pub contract_status: ContractStatus,
}

#[derive(Clone, Debug)]
pub struct CdsContractsTables {
    pub database_id: String,
    pub database_name: String,
    pub table_id: String,
    pub table_name: String,
    pub logical_storage_policy: u32,
}

#[derive(Clone, Debug)]
pub struct CdsContractsTablesColumns {
    pub table_id: String,
    pub column_id: String,
    pub column_name: String,
    pub column_type: u32,
    pub column_length: u32,
    pub column_ordinal: u32,
    pub is_nullable: bool,
}

#[derive(Clone, Debug)]
pub struct CdsCoop {
    pub table_id: String,
    pub column_id: String,
    pub column_name: String,
    pub column_type: u32,
    pub column_length: u32,
    pub column_ordinal: u32,
    pub is_nullable: bool,
}

#[derive(Clone, Debug)]
pub struct CdsHosts {
    pub host_id: String,
    pub host_name: String,
    pub token: Vec<u8>,
    pub ip4: String,
    pub ip6: String,
    pub port: u32,
    pub last_comm_utc: String,
    pub http_addr: String,
    pub http_port: u32
}

#[derive(Debug, Clone)]
pub struct PartialDataResult {
    pub is_successful: bool,
    pub row_id: u32,
    pub data_hash: Option<u64>,
    pub partial_data_status: Option<u32>,
    pub action: Option<PartialDataResultAction>,
}

#[derive(Debug, Clone)]
pub struct DbiConfigSqlite {
    pub root_folder: String,
    pub rcd_db_name: String,
}

#[derive(Debug, Clone)]
pub struct DbiConfigMySql {
    pub user_name: String,
    pub pw: String,
    pub connection_string: String,
    pub host: String,
    pub connect_options: String,
}

#[derive(Debug, Clone)]
pub struct DbiConfigPostgres {
    pub user_name: String,
    pub pw: String,
    pub connection_string: String,
    pub host: String,
    pub connect_options: String,
}

pub fn get_data_queue_table_name(table_name: &str) -> String {
    println!(
        "get_data_queue_table_name: {}",
        format!("{}{}", table_name, defaults::DATA_QUEUE_TABLE_SUFFIX)
    );
    return format!("{}{}", table_name, defaults::DATA_QUEUE_TABLE_SUFFIX);
}

pub fn get_metadata_table_name(table_name: &str) -> String {
    return format!("{}{}", table_name, defaults::METADATA_TABLE_SUFFIX);
}

pub fn get_data_log_table_name(table_name: &str) -> String {
    println!(
        "get_data_log_table_name: {}",
        format!("{}{}", table_name, defaults::DATA_LOG_TABLE_SUFFIX)
    );
    return format!("{}{}", table_name, defaults::DATA_LOG_TABLE_SUFFIX);
}
