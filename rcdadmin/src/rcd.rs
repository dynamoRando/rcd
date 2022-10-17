use rcdx::rcd_enum::{DatabaseType, RcdDatabaseType};

#[allow(dead_code)]
#[derive(Debug)]
pub struct RcdConn {
    ip4addr: String,
    port: u32,
    un: String,
    pw: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RcdDbMetadata {
    name: String,
    /// Unknown = 0,
    /// Sqlite = 1,
    /// Mysql = 2,
    /// Postgres = 3,
    // Sqlserver = 4,
    db_type: DatabaseType,
    /// Unknown = 0,
    /// Rcd = 1,
    /// Host = 2,
    /// Partial = 3,
    rcd_db_type: RcdDatabaseType,
}
