use crate::sql_text::sqlite::{ADD_LOGIN, GET_USER, SQLITE_CREATE_LOGIN_TABLE, UPDATE_USER};
use crate::user_info::UserInfo;
use crate::PROXY_DB;
use crate::{proxy_db::DbConfigSqlite, RcdProxyErr};
use log::{debug, trace, warn};
use rusqlite::named_params;
use rusqlite::{Connection, Result};
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
        if !self.has_user(un) {
            let mut statement = conn.prepare(ADD_LOGIN).unwrap();
            let num_rows = statement
                .execute(named_params! { ":un": un, ":hash": hash.as_bytes().to_vec() })
                .unwrap();

            if num_rows > 0 {
                Ok(())
            } else {
                Err(RcdProxyErr::DbError("Unable to add user".to_string()))
            }
        } else {
            let msg = format!("User '{}' already exists", un);
            Err(RcdProxyErr::UserAlreadyExists(msg))
        }
    }

    pub fn has_user(&self, un: &str) -> bool {
        let where_clause = format!("username = '{}'", un);
        self.has_any_rows_with_table("LOGIN", &where_clause)
    }

    pub fn get_user(&self, un: &str) -> Result<UserInfo, RcdProxyErr> {
        if !self.has_user(un) {
            return Err(RcdProxyErr::UserNotFound(un.to_string()));
        }

        let cmd = GET_USER;
        let conn = self.conn();
        let mut statement = conn.prepare(&cmd).unwrap();

        let row_to_user = |un: String,
                           hash: Vec<u8>,
                           folder: Option<String>,
                           host_id: Option<String>|
         -> Result<UserInfo> {
            Ok(UserInfo {
                username: un,
                hash,
                folder,
                id: host_id,
            })
        };

        let users = statement
            .query_and_then(
                named_params! {
                    ":un": un
                },
                |row| {
                    row_to_user(
                        row.get(0).unwrap(),
                        row.get(1).unwrap(),
                        row.get(2).unwrap(),
                        row.get(3).unwrap(),
                    )
                },
            )
            .unwrap();

        Ok(users.last().unwrap().unwrap())
    }

    pub fn update_user(&self, u: UserInfo) -> Result<(), RcdProxyErr> {
        if !self.has_user(&u.username) {
            return Err(RcdProxyErr::UserNotFound(u.username.to_string()));
        }

        let cmd = UPDATE_USER;
        let conn = self.conn();
        let mut statement = conn.prepare(&cmd).unwrap();
        let r = statement.execute(named_params! {
            ":folder": u.folder.unwrap(),
            ":hash": u.hash,
            ":id": u.id,
            ":un": u.username,
        });

        match r {
            Ok(n) => {
                if n > 0 {
                    return Ok(());
                } else {
                    warn!("no rows affected for update user {}", u.username);
                    return Err(RcdProxyErr::NoRowsAffected);
                }
            }
            Err(e) => return Err(RcdProxyErr::DbError(e.to_string())),
        }
    }

    fn write(&self, sql: &str) -> Result<usize, ProxySqliteErr> {
        let result = self.conn().execute(sql, []);

        match result {
            Ok(n) => Ok(n),
            Err(e) => Err(ProxySqliteErr::General(e.to_string())),
        }
    }

    fn conn(&self) -> Connection {
        let path = Path::new(&self.config.dir).join(&self.config.db_name);
        trace!("{path:?}");
        Connection::open(path).unwrap()
    }

    fn get_scalar_as_u32(&self, cmd: &str) -> u32 {
        debug!("{cmd:?}");

        let mut value: u32 = 0;
        let conn = self.conn();
        let mut statement = conn.prepare(cmd).unwrap();
        let rows = statement.query_map([], |row| row.get(0)).unwrap();

        for item in rows {
            value = item.unwrap_or_default();
        }

        drop(statement);

        value
    }

    fn has_any_rows_with_table(&self, table_name: &str, where_clause: &str) -> bool {
        let cmd = "SELECT COUNT(*) cnt FROM :table WHERE :clause";
        let mut cmd = cmd.replace(":table", table_name);
        cmd = cmd.replace(":clause", where_clause);
        self.has_any_rows_with_cmd(&cmd)
    }

    fn has_any_rows_with_cmd(&self, cmd: &str) -> bool {
        self.get_scalar_as_u32(cmd) > 0
    }
}

#[test]
pub fn test_db_user_io() {
    use ignore_result::Ignore;
    use rcd_common::crypt::hash;
    use rcd_test_harness::get_test_temp_dir;
    use simple_logger::SimpleLogger;
    use std::env;

    SimpleLogger::new().env().init().ignore();

    let dir = get_test_temp_dir("rcd-proxy-db-test-sqlite-register-un");

    let config = DbConfigSqlite {
        dir: dir,
        db_name: PROXY_DB.to_string(),
    };

    let sql = ProxySqlite::new(config);
    sql.config();

    let hash = hash("db_test");
    sql.register_user("db_test", &hash.0).unwrap();
    let mut u = sql.get_user("db_test").unwrap();
    u.folder = Some("new-folder".to_string());
    let result_save = sql.update_user(u);

    debug!("{result_save:?}");

    match result_save {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }

    let x = sql.get_user("db_test").unwrap();
    debug!("{x:?}");
}
