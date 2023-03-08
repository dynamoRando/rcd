use crate::sql_text::sqlite::{ADD_LOGIN, SQLITE_CREATE_LOGIN_TABLE};
use crate::{proxy_db::DbConfigSqlite, RcdProxyErr};
use log::{debug, trace};
use rusqlite::named_params;
use rusqlite::{types::Type, Connection, Result};
use std::path::Path;
use thiserror::Error;

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
    }

    pub fn register_user(&self, un: &str, hash: &str) -> Result<(), RcdProxyErr> {
        let conn = self.conn();
        let mut cmd = "SELECT COUNT(*) num_user FROM LOGIN WHERE username = ':un'".to_string();
        cmd = cmd.replace(":un", &un);
        if !self.has_any_rows(&cmd) {
            let mut statement = conn.prepare(ADD_LOGIN).unwrap();
            let num_rows = statement
                .execute(named_params! { ":un": un, ":hash": hash })
                .unwrap();

            debug!("{statement:?}");

            if num_rows > 0 {
                Ok(())
            } else {
                Err(RcdProxyErr::DbError("Unable to add user".to_string()))
            }
        } else {
            let msg = format!("User '{}' already exists", un);
            return Err(RcdProxyErr::UserAlreadyExists(msg));
        }
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

    fn get_scalar_as_u32(&self, cmd: &str) -> u32 {
        let mut value: u32 = 0;
        let conn = self.conn();
        let mut statement = conn.prepare(&cmd).unwrap();
        let rows = statement.query_map([], |row| row.get(0)).unwrap();

        for item in rows {
            value = item.unwrap_or_default();
        }

        drop(statement);

        value
    }

    fn has_any_rows(&self, cmd: &str) -> bool {
        self.get_scalar_as_u32(cmd) > 0
    }
}
