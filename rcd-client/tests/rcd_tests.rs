use env_logger::{Builder, Target};
use log::info;
use rcd_common::db::DbiConfigSqlite;
use rcd_common::rcd_enum::DatabaseType;
use rcd_common::rcd_settings::RcdSettings;
use rcd_core::dbi::Dbi;
use rcdx::rcd_service::get_service_from_config;
use rcdx::rcd_service::get_service_from_config_file;
use std::env;
use std::fs;
use std::path::Path;

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
        database_type: rcd_common::rcd_enum::DatabaseType::Unknown,
        backing_database_name: String::from(""),
        grpc_client_service_addr_port: String::from("[::1]:50051"),
        grpc_data_service_addr_port: String::from(""),
        data_grpc_timeout_in_seconds: 5,
        client_grpc_timeout_in_seconds: 5,
        http_addr: "".to_string(),
        http_port: 0
    };

    // ACT
    let service = get_service_from_config(rcd_setting);

    // ASSERT
    assert_eq!(service.rcd_settings.database_type, DatabaseType::Unknown);
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
        grpc_client_service_addr_port: String::from("[::1]:50051"),
        grpc_data_service_addr_port: String::from(""),
        data_grpc_timeout_in_seconds: 5,
        client_grpc_timeout_in_seconds: 5,
        http_addr: "".to_string(),
        http_port: 0
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

    let config = DbiConfigSqlite {
        root_folder: cwd.as_os_str().to_str().unwrap().to_string(),
        rcd_db_name: backing_database_name,
    };

    let dbi = Dbi {
        db_type: DatabaseType::Sqlite,
        mysql_config: None,
        postgres_config: None,
        sqlite_config: Some(config),
    };

    dbi.configure_rcd_db();

    let un = String::from("tester");
    let pw = String::from("1234");

    // ACT
    dbi.create_login(&un, &pw);
    let has_login = dbi.has_login(&un);

    info!("test_hash: has_login {}", &has_login);

    let is_valid = dbi.verify_login(&un, &pw);

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

    let config = DbiConfigSqlite {
        root_folder: cwd.as_os_str().to_str().unwrap().to_string(),
        rcd_db_name: backing_database_name.clone(),
    };

    let dbi = Dbi {
        db_type: DatabaseType::Sqlite,
        mysql_config: None,
        postgres_config: None,
        sqlite_config: Some(config),
    };

    dbi.configure_rcd_db();

    let un = String::from("tester_fail");
    let pw = String::from("1234");

    // ACT
    dbi.create_login(&un, &pw);
    let has_login = dbi.has_login(&un);

    info!("test_hash_false: has_login {}", &has_login);

    let config = DbiConfigSqlite {
        root_folder: cwd.to_str().unwrap().to_string(),
        rcd_db_name: backing_database_name.clone(),
    };

    let dbi = Dbi {
        db_type: DatabaseType::Sqlite,
        mysql_config: None,
        postgres_config: None,
        sqlite_config: Some(config),
    };

    let wrong_pw = String::from("43210");
    let is_valid = dbi.verify_login(&un, &wrong_pw);

    info!("test_hash_false: is_valid {}", is_valid);

    // ASSERT
    assert!(&has_login);
    assert!(!is_valid);
}

#[test]
/// Attempts to read a value from the Settings.toml file
fn read_settings_from_file() {
    init();
    let service = get_service_from_config_file();
    let db_type = service.rcd_settings.database_type;
    assert_eq!(db_type, DatabaseType::Sqlite);
}
