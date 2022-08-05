mod client_srv;
mod crypt;
mod db_srv;
pub mod rcd_db;
pub mod rcd_enum;
mod sql_text;
mod sqlitedb;
mod sqlitedbpart;
mod table;
pub mod cdata;
pub mod rcd_settings;
mod rcd_service;
mod database_contract;
mod database_participant;

use crate::rcd_enum::DatabaseType;
use crate::rcd_service::RcdService;
use crate::rcd_settings::RcdSettings;
use std::env;
use std::path::Path;
use config::Config;

/// Configures the backing cds based on the type in the apps current working directory
fn configure_backing_store(
    db_type: DatabaseType,
    backing_db_name: &str,
    admin_un: &str,
    admin_pw: &str,
) {
    let cwd = env::current_dir().unwrap();
    let _db_path = Path::new(&cwd.to_str().unwrap()).join(&backing_db_name);
    let db_location = _db_path.as_os_str().to_str().unwrap();

    match db_type {
        DatabaseType::Sqlite => {
            crate::rcd_db::configure(cwd.to_str().unwrap(), db_location);
            crate::rcd_db::configure_admin(admin_un, admin_pw, db_location)
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

/// Test function that returns a call from the rcd mod
pub fn hello() {
    println!("hello rcd_service");
}

/// Returns an RcdService from the config file
pub fn get_service_from_config_file() -> RcdService {
    let settings = get_config_from_settings_file();
    let service = RcdService {
        rcd_settings: settings,
    };
    return service;
}

/// Returns an RcdService from the supplied config (normally used in testing)
pub fn get_service_from_config(config: RcdSettings) -> RcdService {
    return RcdService {
        rcd_settings: config,
    };
}

pub fn get_config_from_settings_file() -> RcdSettings {
    let settings = Config::builder()
        .add_source(config::File::with_name("src/Settings"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

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

    let rcd_setting = RcdSettings {
        admin_un: String::from("tester"),
        admin_pw: String::from("123456"),
        database_type: database_type,
        backing_database_name: s_db_name,
        client_service_addr_port: s_client_service_addr_port,
        database_service_addr_port: d_client_service_addr_port,
    };

    return rcd_setting;
}
