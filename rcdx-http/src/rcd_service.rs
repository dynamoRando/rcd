

#[derive(Debug)]
pub struct RcdService {
    pub rcd_settings: RcdSettings,
    pub root_dir: String,
    pub db_interface: Option<Dbi>,
}

impl RcdService {
    pub fn cwd(&self) -> String {
        if self.root_dir == "" {
            let wd = env::current_dir().unwrap();
            let cwd = wd.to_str().unwrap();
            return cwd.to_string();
        } else {
            return self.root_dir.clone();
        }
    }

    pub fn start_at_dir(self: &mut Self, root_dir: &str) {
        configure_backing_store_at_dir(
            self.rcd_settings.database_type,
            &self.rcd_settings.backing_database_name,
            &self.rcd_settings.admin_un,
            &self.rcd_settings.admin_pw,
            &root_dir,
        );

        let db_type = self.rcd_settings.database_type;

        match db_type {
            DatabaseType::Sqlite => {
                let sqlite_config = DbiConfigSqlite {
                    root_folder: root_dir.to_string(),
                    rcd_db_name: self.rcd_settings.backing_database_name.clone(),
                };

                let config = Dbi {
                    db_type: db_type,
                    mysql_config: None,
                    postgres_config: None,
                    sqlite_config: Some(sqlite_config),
                };

                self.db_interface = Some(config);
            }

            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
            _ => panic!("Unknown db type"),
        }
    }

    pub fn start(self: &mut Self) {
        configure_backing_store(
            self.rcd_settings.database_type,
            &self.rcd_settings.backing_database_name,
            &self.rcd_settings.admin_un,
            &self.rcd_settings.admin_pw,
        );

        let db_type = self.rcd_settings.database_type;

        match db_type {
            DatabaseType::Sqlite => {
                let sqlite_config = DbiConfigSqlite {
                    root_folder: self.cwd(),
                    rcd_db_name: self.rcd_settings.backing_database_name.clone(),
                };

                let config = Dbi {
                    db_type: db_type,
                    mysql_config: None,
                    postgres_config: None,
                    sqlite_config: Some(sqlite_config),
                };

                self.db_interface = Some(config);
            }

            DatabaseType::Mysql => unimplemented!(),
            DatabaseType::Postgres => unimplemented!(),
            DatabaseType::Sqlserver => unimplemented!(),
            _ => panic!("Unknown db type"),
        }
    }
}