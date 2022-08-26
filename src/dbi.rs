use crate::rcd_enum::DatabaseType;

#[allow(dead_code)]
pub struct Dbi {
    db_type: DatabaseType,
}



#[allow(dead_code)]
struct DbiSqlite {
    pub root_folder: String,
    rcd_db: String,
}