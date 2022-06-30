use config::Config;
use std::cmp::PartialEq;

/// Represents settings for rcd that can be passed in on a test case
#[derive(Debug)]
pub struct RcdSettings {
    ip4_address: String,
    database_port: u64,
    client_port: u64,
    admin_un: String,
    admin_pw: String,
    database_type: DatabaseType,
}

/// Represents the type of backing database rcd is hosting
/// # Types
/// * 0 - Unknown
/// * 1 - Sqlite
/// * 2 - Mysql
/// * 3 - Postgres
/// * 4 - Sqlserver
#[derive(Debug, PartialEq)]
pub enum DatabaseType {
    Unknown = 0,
    Sqlite = 1,
    Mysql = 2,
    Postgres = 3,
    Sqlserver = 4,
}

// https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
impl DatabaseType {
    fn from_i64(value: i64) -> DatabaseType {
        match value {
            0 => DatabaseType::Unknown,
            1 => DatabaseType::Sqlite,
            2 => DatabaseType::Mysql,
            3 => DatabaseType::Postgres,
            4 => DatabaseType::Sqlserver,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Debug)]
pub struct RcdService {
    rcd_settings: RcdSettings,
}

/// Test function that returns a call from the rcd mod
pub fn hello() {
    println!("hello rcd_service");
}

/// Returns an RcdService from the
pub fn get_service_from_config_file() -> RcdService {
    let settings = get_config_from_settings_file();
    let service = RcdService {
        rcd_settings: settings,
    };
    return service;
}

pub fn get_service_from_config(config: RcdSettings) -> RcdService {
    return RcdService {
        rcd_settings: config,
    };
}

fn get_config_from_settings_file() -> RcdSettings {
    let settings = Config::builder()
        .add_source(config::File::with_name("src/rcd/Settings"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let i_database_type = settings.get_int(&String::from("database_type")).unwrap();
    let database_type = DatabaseType::from_i64(i_database_type);

    let rcd_setting = RcdSettings {
        ip4_address: String::from(""),
        database_port: 0,
        client_port: 0,
        admin_un: String::from(""),
        admin_pw: String::from(""),
        database_type: database_type,
    };

    return rcd_setting;
}

#[test]
fn test_read_settings_from_file() {
    let service = get_service_from_config_file();
    let db_type = service.rcd_settings.database_type;
    assert_eq!(db_type, DatabaseType::Sqlite);
}

#[test]
fn test_read_settings_from_config() {
    let rcd_setting = RcdSettings {
        ip4_address: String::from(""),
        database_port: 0,
        client_port: 0,
        admin_un: String::from(""),
        admin_pw: String::from(""),
        database_type: DatabaseType::Unknown,
    };

    let service = get_service_from_config(rcd_setting);

    assert_eq!(service.rcd_settings.database_type, DatabaseType::Unknown);
}
