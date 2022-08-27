use crate::rcd_enum::DatabaseType;

mod dbi_sqlite;

#[derive(Debug, Clone)]
#[allow(dead_code)]
/// Database Interface: an abstraction over the underlying database layer. Supports:
/// - Sqlite
/// - MySQL
/// - Postgres
/// - SQL Server
pub struct Dbi {
    pub db_type: DatabaseType,
    pub mysql_config: Option<DbiConfigMySql>,
    pub postgres_config: Option<DbiConfigPostgres>,
    pub sqlite_config: Option<DbiConfigSqlite>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DbiConfigSqlite {
    pub root_folder: String,
    pub rcd_db_name: String,
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

impl Dbi {
    pub fn configure_admin(self: &Self, login: &str, pw: &str) {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return dbi_sqlite::configure_admin(login, pw, settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    #[allow(dead_code, unused_variables)]
    pub fn verify_login(self: &Self, login: &str, pw: &str) -> bool {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                return dbi_sqlite::verify_login(login, pw, settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    pub fn configure_rcd_db(self: &Self) {
        match self.db_type {
            DatabaseType::Sqlite => {
                let settings = self.get_sqlite_settings();
                dbi_sqlite::configure_rcd_db(settings);
            }
            DatabaseType::Unknown => unimplemented!(),
            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
        }
    }

    fn get_sqlite_settings(self: &Self) -> DbiConfigSqlite {
        return self.sqlite_config.as_ref().unwrap().clone();
    }
}
