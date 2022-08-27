use crate::rcd_enum::DatabaseType;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Dbi {
    pub db_type: DatabaseType,
    pub mysql_config: Option<DbiConfigMySql>,
    pub postgres_config: Option<DbiConfigPostgres>,
    pub sqlite_config: Option<DbiConfigSqlite>
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DbiConfigSqlite {
    pub root_folder: String,
    pub rcd_db_name: String
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DbiConfigMySql {
    pub user_name: String,
    pub pw: String,
    pub connection_string: String,
    pub host: String,
    pub connect_options: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DbiConfigPostgres {
    pub user_name: String,
    pub pw: String,
    pub connection_string: String,
    pub host: String,
    pub connect_options: String,
}
