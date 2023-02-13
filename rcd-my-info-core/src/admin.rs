use std::{path::Path};

use crate::{admin_db::DbType, admin_db_sqlite::SqliteDb};

#[allow(dead_code)]
pub struct Admin {
    db_type: DbType,
    root_dir: String,
    sqlite: Option<SqliteDb>
}

impl Admin {
    pub fn new(db_type: DbType, root_dir: String) -> Self {
        match db_type {
            DbType::Sqlite => {
                let path = Path::new(&root_dir).join("my_info.db");
                let db = SqliteDb::new(path.to_str().unwrap().to_string());
                let admin =  Admin { db_type: DbType::Sqlite, root_dir, sqlite: Some(db) };
                return admin;

            },
            DbType::MySql => todo!(),
            DbType::Postgres => todo!(),
        }
    }

    pub fn register_user(&self, email: &str, pw: &str) -> Result<bool, String> {
        match self.db_type {
            DbType::Sqlite => {
                self.sqlite.as_ref().unwrap().save_account(email, pw)
            },
            DbType::MySql => todo!(),
            DbType::Postgres => todo!(),
        }
    }

    #[allow(dead_code)]
    fn get_sqlite_path(&self) -> String {
        // let cwd = env::current_dir().unwrap();
        let db_path = Path::new(&self.root_dir).join("my_info.db");
        let path = db_path.as_os_str().to_str().unwrap();
        path.to_string()
    }
}
