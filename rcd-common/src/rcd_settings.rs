use crate::rcd_enum::DatabaseType;

/// Represents settings for rcd that can be passed in on a test case
#[derive(Debug, Clone)]
pub struct RcdSettings {
    pub admin_un: String,
    pub admin_pw: String,
    pub database_type: DatabaseType,
    pub backing_database_name: String,
    pub client_service_addr_port: String,
    pub database_service_addr_port: String,
    pub client_grpc_timeout_in_seconds: u32,
    pub data_grpc_timeout_in_seconds: u32,
}
