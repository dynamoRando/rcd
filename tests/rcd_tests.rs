use log::info;
use env_logger::{Builder, Target};
use rcd::rcd_enum::{DatabaseType};
use rcd::rcd_settings::RcdSettings;
use rcd::{get_service_from_config};
use std::env;
use std::path::Path;
use std::fs;
use rcd::rcd_db;
use rusqlite::Connection;
use rcd::dbi::{Dbi, DbiConfigSqlite};

#[path = "test_harness.rs"]
mod test_harness;

#[cfg(test)]
/// Attempts to set the log builder for tests
pub fn init() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    let _init = builder.is_test(true).try_init();
}

#[test]
/// Attempts to read settings from the Settings.toml
fn read_settings_from_config() {
    // ARRANGE
    init();
    let rcd_setting = RcdSettings {
        admin_un: String::from("tester"),
        admin_pw: String::from("123456"),
        database_type: rcd::rcd_enum::DatabaseType::Unknown,
        backing_database_name: String::from(""),
        client_service_addr_port: String::from("[::1]:50051"),
        database_service_addr_port: String::from(""),
    };

    // ACT
    let service = get_service_from_config(rcd_setting);

    // ASSERT
    assert_eq!(
        service.rcd_settings.database_type,
        DatabaseType::Unknown
    );
}

#[test]
/// Attempts to set the backing RCD database name
fn configure_backing_db() {
    init();
    // to see the output, run the test with the following
    // cargo test -- --nocapture
    // RUST_LOG=debug cargo test -- --nocapture

    // ARRANGE
    let rcd_setting = RcdSettings {
        admin_un: String::from("tester"),
        admin_pw: String::from("123456"),
        database_type: DatabaseType::Sqlite,
        backing_database_name: String::from("rcd_test.db"),
        client_service_addr_port: String::from("[::1]:50051"),
        database_service_addr_port: String::from(""),
    };

    let cwd = env::current_dir().unwrap();
    let db_path = Path::new(&cwd).join(&rcd_setting.backing_database_name);

    if db_path.exists() {
        fs::remove_file(&db_path).unwrap();
    }

    // ACT
    let mut service = get_service_from_config(rcd_setting);
    service.start();

    // ASSERT
    assert!(db_path.exists());
}

#[test]
/// Attempts to validate the username and pw are hashing correctly
fn hash() {
    // ARRANGE
    init();

    info!("test_hash: running");

    let cwd = env::current_dir().unwrap();
    let backing_database_name = String::from("test.db");
    let db_path = Path::new(&cwd).join(&backing_database_name);

    if db_path.exists() {
        fs::remove_file(&db_path).unwrap();
    }

    rcd_db::configure(&cwd.to_str().unwrap(), &backing_database_name);

    let db_conn = Connection::open(&db_path).unwrap();

    let un = String::from("tester");
    let pw = String::from("1234");

    // ACT
    crate::rcd_db::create_login(&un, &pw, &db_conn);
    let has_login = crate::rcd_db::has_login(&un, &db_conn).unwrap();

    info!("test_hash: has_login {}", &has_login);

    let config = DbiConfigSqlite {
        root_folder: cwd.to_str().unwrap().to_string(),
        rcd_db_name: backing_database_name,
    };

    let dbi = Dbi {
        db_type: DatabaseType::Sqlite,
        mysql_config: None,
        postgres_config: None,
        sqlite_config: Some(config),
    };

    let is_valid = crate::rcd_db::verify_login(&un, &pw, dbi);

    info!("test_hash: is_valid {}", is_valid);

    // ASSERT
    assert!(&has_login);
    assert!(is_valid);
}

#[test]
/// Tests the functionality of getting the next available testing port for the client service
fn get_harness_value() {
    // ARRANGE, ACT
    let current = test_harness::TEST_SETTINGS
        .lock()
        .unwrap()
        .get_current_port();
    let next = crate::test_harness::TEST_SETTINGS
        .lock()
        .unwrap()
        .get_next_avail_port();

    // ASSERT
    assert_eq!(current + 1, next);
}

#[test]
/// Attempts a negative test of hashing the un and pw to make sure that it can fail
fn hash_negative() {
    // ARRANGE
    init();
    info!("test_hash_false: running");

    let cwd = env::current_dir().unwrap();
    let backing_database_name = String::from("test.db");
    let db_path = Path::new(&cwd).join(&backing_database_name);

    if db_path.exists() {
        fs::remove_file(&db_path).unwrap();
    }

    crate::rcd_db::configure(&cwd.to_str().unwrap(), &backing_database_name);

    let db_conn = Connection::open(&db_path).unwrap();

    let un = String::from("tester_fail");
    let pw = String::from("1234");

    // ACT
    crate::rcd_db::create_login(&un, &pw, &db_conn);
    let has_login = crate::rcd_db::has_login(&un, &db_conn).unwrap();

    info!("test_hash_false: has_login {}", &has_login);

    let config = DbiConfigSqlite {
        root_folder: cwd.to_str().unwrap().to_string(),
        rcd_db_name: backing_database_name,
    };

    let dbi = Dbi {
        db_type: DatabaseType::Sqlite,
        mysql_config: None,
        postgres_config: None,
        sqlite_config: Some(config),
    };

    let wrong_pw = String::from("43210");
    let is_valid = crate::rcd_db::verify_login(&un, &wrong_pw, dbi);

    info!("test_hash_false: is_valid {}", is_valid);

    // ASSERT
    assert!(&has_login);
    assert!(!is_valid);
}

#[test]
/// Attempts to read a value from the Settings.toml file
fn read_settings_from_file() {
    init();
    let service = rcd::get_service_from_config_file();
    let db_type = service.rcd_settings.database_type;
    assert_eq!(db_type, DatabaseType::Sqlite);
}
