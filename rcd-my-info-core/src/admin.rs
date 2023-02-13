use std::{path::Path, env};

use crate::admin_db::DbType;

pub struct Admin {
    pub db_type: DbType
}

impl Admin {
    pub fn new(db_type: DbType) -> Self {
        Admin {
            db_type
        }
    }

    #[allow(unused_variables)]
    pub fn register_user(&self, un: &str, pw: &str) -> Result<bool, String> {
        todo!()
    }

    #[allow(dead_code)]
    fn get_sqlite_path() -> String {
        let cwd = env::current_dir().unwrap();
        let db_path = Path::new(&cwd).join("my_info.db");
        let x = db_path.as_os_str().to_str().unwrap();
        x.to_string()
    }
}