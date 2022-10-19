use rcdx::rcd_enum::{DatabaseType, RcdDatabaseType};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RcdConn {
    ip4addr: String,
    port: u32,
    un: String,
    pw: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RcdDbMetadata {
    name: String,
    db_type: DatabaseType,
    rcd_db_type: RcdDatabaseType,
}
