use crate::proxy_db::DbConfigSqlite;
use log::trace;
use rusqlite::{types::Type, Connection, Result};
use std::path::Path;
use thiserror::Error;
use crate::sql_text::sqlite::SQLITE_CREATE_LOGIN_TABLE;

#[derive(Debug, Error)]
pub enum ProxySqliteErr {
    #[error("Error `{0}`")]
    General(String),
}

#[derive(Clone, Debug)]
pub struct ProxySqlite {
    config: DbConfigSqlite,
}

impl ProxySqlite {
    pub fn new(config: DbConfigSqlite) -> Self {
        Self { config }
    }

    pub fn config(&self) {
        self.write(SQLITE_CREATE_LOGIN_TABLE).unwrap();

        todo!();
    }

    fn write(&self, sql: &str) -> Result<usize, ProxySqliteErr> {
        let result = self.conn().execute(sql, []);

        return match result {
            Ok(n) => Ok(n),
            Err(e) => Err(ProxySqliteErr::General(e.to_string())),
        };
    }

    fn conn(&self) -> Connection {
        let path = Path::new(&self.config.dir).join(&self.config.db_name);
        trace!("{path:?}");
        Connection::open(path).unwrap()
    }
}
