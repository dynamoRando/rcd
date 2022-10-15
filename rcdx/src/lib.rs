mod coop_database_contract;
mod coop_database_participant;
mod crypt;
mod data_srv;
pub mod dbi;
pub mod defaults;
mod host_info;
pub mod query_parser;
pub mod rcd_db;
pub mod rcd_enum;
mod rcd_service;
pub mod rcd_settings;
mod remote_db_srv;
mod sqlclient_srv;
mod table;


use crate::rcd_enum::DatabaseType;
use crate::rcd_service::RcdService;
use crate::rcd_settings::RcdSettings;
use config::Config;
use dbi::{Dbi, DbiConfigSqlite};
use std::env;
use std::path::Path;

/// Configures the backing cds based on the type in the apps current working directory
fn configure_backing_store(
    db_type: DatabaseType,
    backing_db_name: &str,
    admin_un: &str,
    admin_pw: &str,
) {
    let cwd = env::current_dir().unwrap();

    match db_type {
        DatabaseType::Sqlite => {
            let config = DbiConfigSqlite {
                root_folder: cwd.as_os_str().to_str().unwrap().to_string(),
                rcd_db_name: backing_db_name.to_string(),
            };

            let dbi = Dbi {
                db_type: DatabaseType::Sqlite,
                mysql_config: None,
                postgres_config: None,
                sqlite_config: Some(config),
            };

            crate::rcd_db::configure(&dbi);
            crate::rcd_db::configure_admin(admin_un, admin_pw, &dbi);
        }
        DatabaseType::Mysql => do_nothing(),
        DatabaseType::Postgres => do_nothing(),
        DatabaseType::Sqlserver => do_nothing(),
        _ => panic!("Unknown db type"),
    }
}

/// Configures the backing cds based on the type in the apps current working directory
fn configure_backing_store_at_dir(
    db_type: DatabaseType,
    backing_db_name: &str,
    admin_un: &str,
    admin_pw: &str,
    root_dir: &str,
) {
    match db_type {
        DatabaseType::Sqlite => {
            let config = DbiConfigSqlite {
                root_folder: root_dir.to_string(),
                rcd_db_name: backing_db_name.to_string(),
            };

            let dbi = Dbi {
                db_type: DatabaseType::Sqlite,
                mysql_config: None,
                postgres_config: None,
                sqlite_config: Some(config),
            };

            crate::rcd_db::configure(&dbi);
            crate::rcd_db::configure_admin(admin_un, admin_pw, &dbi);
        }
        DatabaseType::Mysql => do_nothing(),
        DatabaseType::Postgres => do_nothing(),
        DatabaseType::Sqlserver => do_nothing(),
        _ => panic!("Unknown db type"),
    }
}

fn do_nothing() {
    println!("do nothing");
}

/// Returns an RcdService from the config file
pub fn get_service_from_config_file() -> RcdService {
    let settings = get_config_from_settings_file();
    let mut service = RcdService {
        rcd_settings: settings,
        root_dir: String::from(""),
        db_interface: None,
        sql_client_channel: None,
        db_client_channel: None
    };

    if service.root_dir == "" {
        let wd = env::current_dir().unwrap().clone();
        let cwd = wd.to_str().unwrap().to_string().clone();
        service.root_dir = cwd.to_string();
    }

    return service;
}

/// Returns an RcdService from the supplied config (normally used in testing)
pub fn get_service_from_config(config: RcdSettings) -> RcdService {
    return RcdService {
        rcd_settings: config,
        root_dir: String::from(""),
        db_interface: None,
        sql_client_channel: None,
        db_client_channel: None,
    };
}

pub fn get_config_from_settings_file() -> RcdSettings {
    let wd = env::current_dir().unwrap();
    let cwd = wd.to_str().unwrap();
    let settings_in_cwd = Path::new(cwd).join("Settings.toml");

    let settings_location;

    if Path::exists(&settings_in_cwd) {
        settings_location = settings_in_cwd.to_str().unwrap();
    } else {
        settings_location = "src/Settings";
    }

    let error_message = format!(
        "{}{}",
        "Could not find Settings.toml in current directort or in default ", settings_location
    );

    let settings = Config::builder()
        .add_source(config::File::with_name(settings_location))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .expect(&error_message);

    let i_database_type = settings.get_int(&String::from("database_type")).unwrap();
    let database_type = DatabaseType::from_i64(i_database_type);

    let s_db_name = settings
        .get_string(&String::from("backing_database_name"))
        .unwrap();

    let s_client_service_addr_port = settings
        .get_string(&String::from("client_service_addr_port"))
        .unwrap();

    let d_client_service_addr_port = settings
        .get_string(&String::from("data_service_addr_port"))
        .unwrap();

    let admin_un = settings.get_string(&String::from("admin_un")).unwrap();

    let admin_pw = settings.get_string(&String::from("admin_pw")).unwrap();

    let rcd_setting = RcdSettings {
        admin_un: admin_un,
        admin_pw: admin_pw,
        database_type: database_type,
        backing_database_name: s_db_name,
        client_service_addr_port: s_client_service_addr_port,
        database_service_addr_port: d_client_service_addr_port,
    };

    return rcd_setting;
}

pub fn get_current_directory() -> String {
    let wd = env::current_dir().unwrap().clone();
    let cwd = wd.to_str().unwrap().to_string().clone();
    return cwd.to_string();
}
