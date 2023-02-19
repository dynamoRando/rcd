use std::path::Path;

use crate::{admin_db::DbType, admin_db_sqlite::SqliteDb, rcd_docker::RcdDocker};

#[allow(dead_code)]
pub struct Admin {
    db_type: DbType,
    root_dir: String,
    sqlite: Option<SqliteDb>,
    docker: Option<RcdDocker>,
}

impl Admin {
    pub fn new(db_type: DbType, root_dir: String) -> Self {
        match db_type {
            DbType::Sqlite => {
                let path = Path::new(&root_dir).join("my_info.db");
                let db = SqliteDb::new(path.to_str().unwrap().to_string());
                
                Admin {
                    db_type: DbType::Sqlite,
                    root_dir,
                    sqlite: Some(db),
                    docker: None,
                }
            }
            DbType::MySql => todo!(),
            DbType::Postgres => todo!(),
        }
    }

    pub fn set_docker_ip(mut self, docker_ip: &str) -> Self {
        let docker = RcdDocker::new(docker_ip.to_string()).unwrap();
        self.docker = Some(docker);
        self
    }

    pub fn verify_login(&self, email: &str, pw: &str) -> Result<bool, String> {
        match self.db_type {
            DbType::Sqlite => {
                return self.sqlite.as_ref().unwrap().validate_login(email, pw);
            }
            DbType::MySql => todo!(),
            DbType::Postgres => todo!(),
        }
    }

    pub fn register_user(&self, email: &str, pw: &str) -> Result<bool, String> {
        match self.db_type {
            DbType::Sqlite => {
                return self.sqlite.as_ref().unwrap().save_account(email, pw);
            }
            DbType::MySql => todo!(),
            DbType::Postgres => todo!(),
        }
    }

    #[allow(unused_variables)]
    pub fn provision_container_for_user(&self, email: &str) -> Result<bool, String> {
        match self.db_type {
            DbType::Sqlite => {
                let client_port = self.sqlite.as_ref().unwrap().get_last_used_port() + 1;
                let data_port = client_port + 1;
                let http_port = data_port + 1;

                // we need to create a docker instance, exposing 50051, 50052, 50055 
                // to the previously determined ports

                // if that provision works, then we will update the users' account
                // with these port numbers 

                // finally we should update the last port used for the next account that is provisioned
                todo!()
            }
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
