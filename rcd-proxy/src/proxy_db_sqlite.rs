use crate::sql_text::sqlite::{
    ADD_LOGIN, GET_HOST, GET_USER, SQLITE_CREATE_LOGIN_TABLE, SQLITE_CREATE_TOKENS_TABLE,
    UPDATE_USER,
};
use crate::user_info::UserInfo;
#[allow(unused_imports)]
use crate::PROXY_DB;
use crate::{proxy_db::DbConfigSqlite, RcdProxyErr};
use chrono::{DateTime, Utc};
use stdext::function_name;
use tracing::{debug, trace, warn};
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
        self.write(SQLITE_CREATE_TOKENS_TABLE).unwrap();
    }

    pub fn save_token(
        &self,
        login: &str,
        token: &str,
        expiration: DateTime<Utc>,
    ) -> Result<(), RcdProxyErr> {
        let conn = self.conn();
        let cmd = String::from(
            "
                INSERT INTO TOKENS
                (
                    USERNAME,
                    TOKEN,
                    ISSUED_UTC,
                    EXPIRATION_UTC
                )
                VALUES
                (
                    :un,
                    :token,
                    :issued,
                    :expiration
                );",
        );

        let issued = Utc::now().to_rfc3339();
        let expiration = expiration.to_rfc3339();

        let mut statement = conn.prepare(&cmd).unwrap();
        statement
            .execute(named_params! {
                ":un" : login.to_string(),
                ":token" : token,
                ":issued" : issued,
                ":expiration" : expiration,
            })
            .unwrap();

        Ok(())
    }

    pub fn get_user_with_token(&self, token: &str) -> Result<UserInfo, RcdProxyErr> {
        let cmd = "SELECT username FROM TOKENS WHERE token = ':token'";
        let cmd = cmd.replace(":token", token);
        let un = self.get_scalar_as_string(&cmd);
        self.get_user(&un)
    }

    pub fn verify_token(&self, token: &str) -> bool {
        let mut cmd = String::from("SELECT COUNT(*) FROM TOKENS WHERE TOKEN = ':token'");
        cmd = cmd.replace(":token", token);
        self.has_any_rows_with_cmd(&cmd)
    }

    pub fn revoke_tokens_for_login(&self, login: &str) -> bool {
        let mut cmd = String::from("DELETE FROM TOKENS WHERE USERNAME = ':login'");
        cmd = cmd.replace(":login", login);
        self.write(&cmd).unwrap() > 0
    }

    pub fn login_has_token(&self, login: &str) -> bool {
        let mut cmd = String::from("SELECT COUNT(*) FROM TOKENS WHERE USERNAME = ':login'");
        cmd = cmd.replace(":login", login);
        self.has_any_rows_with_cmd(&cmd)
    }

    pub fn delete_expired_tokens(&self) {
        let conn = self.conn();
        let now = Utc::now().to_rfc3339();

        let mut cmd = String::from("DELETE FROM TOKENS WHERE EXPIRATION_UTC < ':now'");
        cmd = cmd.replace(":now", &now);

        let _ = conn.execute(&cmd, []);
        let _ = conn.close();
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

    pub fn has_host(&self, id: &str) -> bool {
        let where_clause = format!("host_id = '{}'", id);
        self.has_any_rows_with_table("LOGIN", &where_clause)
    }

    pub fn get_host(&self, id: &str) -> Result<UserInfo, RcdProxyErr> {
        if !self.has_host(id) {
            return Err(RcdProxyErr::HostIdNotFound(id.to_string()));
        }

        let cmd = GET_HOST;

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
                    ":id": id
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

    pub fn update_user(&self, u: &UserInfo) -> Result<(), RcdProxyErr> {
        if !self.has_user(&u.username) {
            return Err(RcdProxyErr::UserNotFound(u.username.to_string()));
        }

        let cmd = UPDATE_USER;
        let conn = self.conn();
        let mut statement = conn.prepare(&cmd).unwrap();
        let r = statement.execute(named_params! {
            ":folder": u.folder.as_ref().unwrap(),
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
        trace!("[{}]: {path:?}", function_name!());
        Connection::open(path).unwrap()
    }

    fn get_scalar_as_u32(&self, cmd: &str) -> u32 {
        trace!("[{}]: {cmd:?}", function_name!());

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

    fn get_scalar_as_string(&self, cmd: &str) -> String {
        trace!("[{}]: {cmd:?}", function_name!());

        let conn = self.conn();
        let mut value = String::from("");
        let mut statement = conn.prepare(&cmd).unwrap();
        let rows = statement.query_map([], |row| row.get(0)).unwrap();

        for item in rows {
            value = item.unwrap();
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
    use rcd_test_harness_common::get_test_temp_dir;
    use simple_logger::SimpleLogger;

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
    let result_save = sql.update_user(&u);

    debug!("{result_save:?}");

    match result_save {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }

    let x = sql.get_user("db_test").unwrap();
    debug!("{x:?}");
}
