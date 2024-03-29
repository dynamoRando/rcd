use tracing::{debug, error, info, warn};
use rcd_common::crypt::{self, hash};
use rcd_sqlite::sqlite::{get_scalar_as_u32, has_any_rows};
use rusqlite::{named_params, Connection};
use std::path::Path;

use crate::account::Account;

pub struct SqliteDb {
    db_path: String,
}

impl SqliteDb {
    /// Creates a new instance of a SqliteDb for backing a my-info instance
    ///  Parameters: `db_path` the full path to the sqlite database including the name of the file
    pub fn new(db_path: String) -> Self {
        let db = SqliteDb { db_path };
        db.create_account_table();
        db.create_admin_port_table();
        db
    }

    pub fn validate_login(&self, email: &str, pw: &str) -> Result<bool, String> {
        let mut is_verified = false;
        if self.has_account(email) {
            let conn = self.get_db_conn();
            let cmd = "SELECT EMAIL, HASH FROM ACCOUNTS WHERE EMAIL = :email";
            let mut statement = conn.prepare(cmd).unwrap();

            let user_iter = statement
                .query_map([email], |row| {
                    Ok(Account {
                        email: row.get(0).unwrap(),
                        hash: row.get(1).unwrap(),
                    })
                })
                .unwrap();

            for user in user_iter {
                let returned_value = user.unwrap();

                let mut padded = [0u8; 128];
                returned_value
                    .hash
                    .as_bytes()
                    .iter()
                    .enumerate()
                    .for_each(|(i, val)| {
                        padded[i] = *val;
                    });

                if crypt::verify(padded, pw) {
                    is_verified = true;
                    break;
                }
            }
        }
        Ok(is_verified)
    }

    pub fn has_account(&self, email: &str) -> bool {
        let cmd = "SELECT COUNT(*) LOGINS FROM ACCOUNTS WHERE EMAIL = ':email'";
        let cmd = cmd.replace(":email", email);
        has_any_rows(cmd, &self.get_db_conn())
    }

    pub fn save_account(&self, email: &str, pw: &str) -> Result<bool, String> {
        if !self.has_account(email) {
            let pw = hash(pw);
            let cmd = "INSERT INTO ACCOUNTS (EMAIL, HASH) VALUES (:email, :hash);";
            let conn = self.get_db_conn();
            let mut statement = conn.prepare(cmd).unwrap();
            let result = statement.execute(named_params! { ":email": email, ":hash": pw.0 });

            debug!("{:?}", statement);

            match result {
                Ok(num_rows) => {
                    let account_created = num_rows > 0;
                    if account_created {
                        info!("account {email} created");
                    }
                    Ok(account_created)
                }
                Err(e) => {
                    error!("{e}");
                    Err(e.to_string())
                }
            }
        } else {
            warn!("account {email} already exists, will not create a second account");
            Ok(false)
        }
    }

    pub fn get_last_used_port(&self) -> u32 {
        let cmd = "SELECT LAST_ASSIGNED FROM PORT_CONFIG";
        get_scalar_as_u32(cmd.to_string(), &self.get_db_conn())
    }

    pub fn update_last_used_port(&self, port_num: u32) {
        let cmd = "UPDATE PORT_CONFIG SET LAST_ASSIGNED = :num";
        let cmd = cmd.replace(":port_num", &port_num.to_string());
        self.get_db_conn().execute(&cmd, []).unwrap();
    }

    fn get_db_conn(&self) -> Connection {
        let db_path = Path::new(&self.db_path);
        debug!("{db_path:?}");
        Connection::open(db_path).unwrap()
    }

    fn create_account_table(&self) {
        let cmd = "CREATE TABLE IF NOT EXISTS ACCOUNTS (
                    EMAIL VARCHAR(50) NOT NULL, 
                    HASH BLOB NOT NULL,
                    CONTAINER_ID TEXT,
                    CLIENT_PORT INT,
                    DATA_PORT INT,
                    HTTP_PORT INT);";
        let conn = self.get_db_conn();
        conn.execute(cmd, []).unwrap();
    }

    fn create_admin_port_table(&self) {
        let cmd = "CREATE TABLE IF NOT EXISTS PORT_CONFIG (
            START INT,
            LAST_ASSIGNED INT
        );";
        let conn = self.get_db_conn();
        conn.execute(cmd, []).unwrap();

        let cmd = "INSERT INTO PORT_CONFIG (START, LAST_ASSIGNED) VALUES (10000, 10000);";
        conn.execute(cmd, []).unwrap();
    }
}
