use chrono::{DateTime, Utc};

use crate::proxy_db_sqlite::ProxySqlite;
use crate::user_info::UserInfo;
use crate::RcdProxyErr;

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

    pub fn save_token(&self, login: &str, token: &str, expiration: DateTime<Utc>) -> Result<(), RcdProxyErr> {
        match self.config {
            ProxyDbConfig::Unknown => todo!(),
            ProxyDbConfig::Sqlite(_) => self.sqlite().save_token(login, token, expiration),
            ProxyDbConfig::MySql(_) => todo!(),
            ProxyDbConfig::Postgres(_) => todo!(),
        }
    }

    pub fn revoke_tokens_for_login(&self, un: &str) -> bool {
        self.delete_expired_tokens();
        match self.config {
            ProxyDbConfig::Unknown => todo!(),
            ProxyDbConfig::Sqlite(_) => self.sqlite().revoke_tokens_for_login(un),
            ProxyDbConfig::MySql(_) => todo!(),
            ProxyDbConfig::Postgres(_) => todo!(),
        }
    }

    pub fn login_has_token(&self, un: &str) -> bool {
        self.delete_expired_tokens();
        match self.config {
            ProxyDbConfig::Unknown => todo!(),
            ProxyDbConfig::Sqlite(_) => self.sqlite().login_has_token(un),
            ProxyDbConfig::MySql(_) => todo!(),
            ProxyDbConfig::Postgres(_) => todo!(),
        }
    }

    pub fn verify_token(&self, token: &str) -> bool {
        self.delete_expired_tokens();
        match self.config {
            ProxyDbConfig::Sqlite(_) => {
                self.sqlite().verify_token(&token)
            }
            ProxyDbConfig::Unknown => unimplemented!(),
            ProxyDbConfig::MySql(_) => unimplemented!(),
            ProxyDbConfig::Postgres(_) => unimplemented!(),
        }
    }

    pub fn delete_expired_tokens(&self) {
        match self.config {
            ProxyDbConfig::Sqlite(_) => {
                self.sqlite().delete_expired_tokens();
            }
            ProxyDbConfig::Unknown => unimplemented!(),
            ProxyDbConfig::MySql(_) => unimplemented!(),
            ProxyDbConfig::Postgres(_) => unimplemented!(),
        }
    }

    #[allow(dead_code)]
    pub fn has_user(&self, un: &str) -> bool {
        match self.config {
            ProxyDbConfig::Unknown => todo!(),
            ProxyDbConfig::Sqlite(_) => self.sqlite().has_user(un),
            ProxyDbConfig::MySql(_) => todo!(),
            ProxyDbConfig::Postgres(_) => todo!(),
        }
    }

    #[allow(dead_code)]
    pub fn has_host(&self, id: &str) -> bool {
        match self.config {
            ProxyDbConfig::Unknown => todo!(),
            ProxyDbConfig::Sqlite(_) => self.sqlite().has_host(id),
            ProxyDbConfig::MySql(_) => todo!(),
            ProxyDbConfig::Postgres(_) => todo!(),
        }
    }

    pub fn get_host(&self, id: &str) -> Result<UserInfo, RcdProxyErr> {
        match self.config {
            ProxyDbConfig::Unknown => todo!(),
            ProxyDbConfig::Sqlite(_) => self.sqlite().get_host(id),
            ProxyDbConfig::MySql(_) => todo!(),
            ProxyDbConfig::Postgres(_) => todo!(),
        }
    }

    pub fn get_user(&self, un: &str) -> Result<UserInfo, RcdProxyErr> {
        match self.config {
            ProxyDbConfig::Unknown => todo!(),
            ProxyDbConfig::Sqlite(_) => self.sqlite().get_user(un),
            ProxyDbConfig::MySql(_) => todo!(),
            ProxyDbConfig::Postgres(_) => todo!(),
        }
    }

    pub fn update_user(&self, u: &UserInfo) -> Result<(), RcdProxyErr> {
        match self.config {
            ProxyDbConfig::Unknown => todo!(),
            ProxyDbConfig::Sqlite(_) => self.sqlite().update_user(u),
            ProxyDbConfig::MySql(_) => todo!(),
            ProxyDbConfig::Postgres(_) => todo!(),
        }
    }

    fn sqlite(&self) -> &ProxySqlite {
        self.sqlite.as_ref().unwrap()
    }
}
