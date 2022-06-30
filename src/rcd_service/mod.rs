use config::Config;
use std::collections::HashMap;
use std::cmp::PartialEq;

/// Represents settings for rcd that can be passed in on a test case
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
#[derive(Debug)]
#[derive(PartialEq)]
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

pub struct RcdService {
    rcd_settings: RcdSettings,
}

pub fn hello() {
    println!("hello rcd_service");
    read_config();
}

pub fn start() {
    unimplemented!("not completed yet");

    //read_config();
    //configure_backing_store();
}

pub fn start_with_test_settings(test_settings: RcdSettings) {
    unimplemented!("not completed yet");
}

impl RcdService {
    pub fn read_and_return_config(self: &Self) -> RcdSettings {
        let settings = Config::builder()
            .add_source(config::File::with_name("src/rcd_service/Settings"))
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
}

/// reads the Settings.toml config file
fn read_config() {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("src/rcd_service/Settings"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let priority_setting = String::from("priority");
    let priority_value = settings.get_int(&priority_setting).unwrap();

    println!("{:?}", priority_value);

    let i_database_type = settings.get_int(&String::from("database_type")).unwrap();
    let database_type = DatabaseType::from_i64(i_database_type);

    println!("database type: {:?}", database_type);

    // Print out our settings (as a HashMap)
    println!(
        "{:?}",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    )
}

/// checks the backing database to see if it needs to be setup
fn configure_backing_store(database_type: DatabaseType) {
    println!("database type: {:?}", database_type);
}

#[test]
fn test_read_settings() {
    let sett = RcdSettings {
        ip4_address: String::from(""),
        database_port: 0,
        client_port: 0,
        admin_un: String::from(""),
        admin_pw: String::from(""),
        database_type: DatabaseType::Unknown,
    };

    let service = RcdService { rcd_settings: sett };
    let returned_setting = service.read_and_return_config();
    assert_eq!(returned_setting.database_type, DatabaseType::Sqlite);
}
