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
