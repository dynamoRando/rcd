use crate::proxy_db_sqlite::ProxySqlite;
use crate::{RcdProxyErr, PROXY_DB};

#[derive(Clone, Debug)]
pub struct DbConfigSqlite {
    pub db_name: String,
    pub dir: String,
}

#[derive(Clone, Debug)]
pub struct DbConfigMySql {}

#[derive(Clone, Debug)]
pub struct DbConfigPostgres {}

#[derive(Clone, Debug)]
#[allow(dead_code, unused_variables)]
pub enum ProxyDbConfig {
    Unknown,
    Sqlite(DbConfigSqlite),
    MySql(DbConfigMySql),
    Postgres(DbConfigPostgres),
}

#[derive(Clone, Debug)]
pub struct ProxyDb {
    config: ProxyDbConfig,
    sqlite: Option<ProxySqlite>,
}

impl ProxyDb {
    pub fn new_with_sqlite(db_name: String, dir: String) -> Self {
        let config = DbConfigSqlite { db_name, dir };
        let sqlite = ProxySqlite::new(config.clone());

        Self {
            config: ProxyDbConfig::Sqlite(config),
            sqlite: Some(sqlite),
        }
    }

    /// Configures the backing database. This will setup the needed tables, etc for work.
    pub fn config(&self) {
        match self.config {
            ProxyDbConfig::Unknown => todo!(),
            ProxyDbConfig::Sqlite(_) => {
                self.sqlite().config();
            }
            ProxyDbConfig::MySql(_) => todo!(),
            ProxyDbConfig::Postgres(_) => todo!(),
        }
    }

    pub fn register_user(&self, un: &str, hash: &str) -> Result<(), RcdProxyErr> {
        match self.config {
            ProxyDbConfig::Unknown => todo!(),
            ProxyDbConfig::Sqlite(_) => self.sqlite().register_user(un, hash),
            ProxyDbConfig::MySql(_) => todo!(),
            ProxyDbConfig::Postgres(_) => todo!(),
        }
    }

    #[allow(dead_code, unused_variables)]
    pub fn has_user(&self, un: &str) -> Result<bool, RcdProxyErr> {
        todo!()
    }

    fn sqlite(&self) -> &ProxySqlite {
        self.sqlite.as_ref().unwrap()
    }
}
