use crate::rcd_enum::ContractStatus;

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
}

