use log::{debug, error, info, warn};
use rcd_common::crypt::hash;
use rcd_sqlite::sqlite::has_any_rows;
use rusqlite::{named_params, Connection};
use std::path::Path;

pub struct SqliteDb {
    db_path: String,
}

impl SqliteDb {
    /// Creates a new instance of a SqliteDb for backing a my-info instance
    ///  Parameters: `db_path` the full path to the sqlite database including the name of the file
    pub fn new(db_path: String) -> Self {
        let db = SqliteDb { db_path };
        db.create_account_table();
        db
    }

    pub fn save_account(&self, email: &str, pw: &str) -> Result<bool, String> {
        let cmd = "SELECT COUNT(*) LOGINS FROM ACCOUNTS WHERE EMAIL = :email";
        let cmd = cmd.replace(":email", email);
        if !has_any_rows(cmd, &self.get_db_conn()) {
            let pw = hash(pw);
            let cmd = "INSERT INTO ACCOUNTS (EMAIL, HASH) VALUES (:email, :hash);";
            let conn = self.get_db_conn();
            let mut statement = conn.prepare(&cmd).unwrap();
            let result = statement.execute(named_params! { ":email": email, ":hash": pw.0 });

            match result {
                Ok(num_rows) => {
                    let account_created = num_rows > 0;
                    if account_created {
                        info!("account {email} created");
                    }
                    return Ok(account_created);
                }
                Err(e) => {
                    error!("{e}");
                    return Err(e.to_string());
                }
            }
        } else {
            warn!("account {email} already exists, will not create a second account");
            return Ok(false);
        }
    }

    fn get_db_conn(&self) -> Connection {
        let db_path = Path::new(&self.db_path);
        debug!("{db_path:?}");
        Connection::open(db_path).unwrap()
    }

    fn create_account_table(&self) {
        let cmd =
            "CREATE TABLE IF NOT EXISTS ACCOUNTS (EMAIL VARCHAR(50) NOT NULL, HASH BLOB NOT NULL);";
        let conn = self.get_db_conn();
        conn.execute(cmd, []).unwrap();
    }
}
