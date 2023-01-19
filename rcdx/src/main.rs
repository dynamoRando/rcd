use log::info;
use log4rs;
use rcd_common::defaults;

use rcd_enum::{
    logical_storage_policy::LogicalStoragePolicy, remote_delete_behavior::RemoteDeleteBehavior,
};

use rcd_core::comm::{RcdCommunication, RcdRemoteDbClient};
use rcd_core::rcd::Rcd;
use rcd_core::rcd_data::RcdData;
use rcd_core::remote_http::RemoteHttp;
use rcd_http::http_srv;
use rcd_service::get_current_directory;
use std::io::Write;
use std::{env, fs::File, io, path::Path};
use tokio::task;
use triggered;

use crate::rcd_service::get_service_from_config_file;

pub mod rcd_service;

#[tokio::main]
async fn main() {
    let version_message = format!("rcdx version {}.", defaults::VERSION);
    println!("{}", version_message);
    set_default_logging();

    // https://tms-dev-blog.com/log-to-a-file-in-rust-with-log4rs/
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    info!("{}", version_message);

    let (client_trigger, client_listener) = triggered::trigger();
    let (db_trigger, db_listener) = triggered::trigger();

    let args: Vec<String> = env::args().collect();
    let alt_settings = process_cmd_args(args);
    set_default_config();
    let mut service = get_service_from_config_file(alt_settings);

    println!("rcd settings found:");
    println!("{:?}", service.rcd_settings);
    println!("root dir: {}", service.root_dir);
    service.start();

    let dbi_settings = service.get_dbi();
    let dbi_core_clone = dbi_settings.clone();
    let dbi_data_clone = dbi_settings.clone();

    let settings = service.rcd_settings.clone();
    let db_name = settings.backing_database_name.clone();
    let client_port = settings.grpc_client_service_addr_port.clone();
    let db_port = settings.grpc_data_service_addr_port.clone();
    let root_dir = service.root_dir.clone();
    let data_timeout = settings.data_grpc_timeout_in_seconds;

    let http_addr = settings.http_addr;
    let http_port = settings.http_port;

    let _ = task::spawn_blocking(move || {
        let _ = service.start_grpc_at_addrs_with_shutdown(
            db_name,
            client_port,
            db_port,
            root_dir.to_string(),
            client_listener,
            db_listener,
            data_timeout,
        );
    })
    .await;

    // start http, need to make this configurable
    let _ = task::spawn_blocking(move || {
        let http = RemoteHttp {
            own_http_addr: http_addr.clone(),
            own_http_port: http_port as u32,
        };

        let remote_client = RcdRemoteDbClient {
            comm_type: RcdCommunication::Http,
            grpc: None,
            http: Some(http),
        };

        let core = Rcd {
            db_interface: Some(dbi_core_clone),
            remote_client: Some(remote_client),
        };

        let data = RcdData {
            db_interface: Some(dbi_data_clone),
        };

        let _ = http_srv::start_http(core, data, http_addr, http_port);
    });

    let mut input = String::from("");
    println!("rcd is running. please press 'q' and enter to quit.");

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.contains("q") {
            info!("shutting down...");
            client_trigger.trigger();
            db_trigger.trigger();
            http_srv::shutdown_http().await;
            break;
        }
    }

    println!("rcd is exiting. i remain obediently yours.");
}

fn process_cmd_args(args: Vec<String>) -> Option<String> {
    if args.len() >= 2 {
        let cmd = args[1].as_str();

        match cmd {
            "default_settings" => {
                set_default_config();
                return None;
            }
            "make_test_db" => {
                make_test_db();
                return None;
            }
            "alt-config" => {
                let alt_settings = args[2].to_string();
                return Some(alt_settings);
            }
            _ => return None,
        }
    }

    return None;
}

fn set_default_config() {
    let cwd = get_current_directory();
    println!("cwd: {}", cwd);
    let default_settings_content = String::from(
        r#"
debug = false
database_type = 1
backing_database_name = "rcd.db"
rcd_schema = "rcd"
grpc_client_service_addr_port = "127.0.0.1:50051"
grpc_data_service_addr_port = "127.0.0.1:50052"
http_addr = "127.0.0.1"
http_port = "50055"
admin_un = "tester"
admin_pw = "123456"
client_grpc_timeout_in_seconds = 5
data_grpc_timeout_in_seconds = 5
    "#,
    );

    let default_src_path = Path::new(&cwd).join("src/Settings.toml");
    let path = Path::new(&cwd).join("Settings.toml");
    if !Path::exists(&default_src_path) && !Path::exists(&path) {
        println!(
            "creating default Settings.toml at: {}",
            &path.to_str().unwrap()
        );
        let mut output = File::create(path).unwrap();
        write!(output, "{}", default_settings_content).unwrap();
    } else {
        println!("Settings.toml was found, skipping default settings");
    }
}

fn set_default_logging() {
    let cwd = get_current_directory();
    let default_logging_content = r#"appenders:
   stdout:
     kind: console
     encoder:
       pattern: "{h({d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n})}"
   file_logger:
     kind: rolling_file
     path: "log/rcd.log"
     encoder:
       pattern: "{d(%Y-%m-%d %H:%M:%S)(utc)} - {h({l})}: {m}{n}"
     policy:
       trigger:
         kind: size
         limit: 50kb
       roller:
         kind: fixed_window
         base: 1
         count: 10
         pattern: "log/rcd{}.log"
root:
   level: trace
   appenders:
     - stdout
     - file_logger"#;

    let default_src_path = Path::new(&cwd).join("logging_config.yaml");
    if !Path::exists(&default_src_path) {
        println!(
            "creating default logging_config.yaml at: {}",
            &default_src_path.to_str().unwrap()
        );
        let mut output = File::create(default_src_path).unwrap();
        write!(output, "{}", default_logging_content).unwrap();
    } else {
        println!("logging_config.yaml was found, skipping default settings");
    }
}

fn make_test_db() {
    let test_db_name = "test.db";
    let cwd = get_current_directory();
    let default_src_path = Path::new(&cwd).join(test_db_name);
    if !Path::exists(&default_src_path) {
        println!(
            "creating test_db at: {}",
            &default_src_path.to_str().unwrap()
        );

        let mut service = get_service_from_config_file(None);
        service.start();
        let dbi = service.get_dbi();

        let _ = dbi.create_database(test_db_name);
        let _ = dbi.enable_coooperative_features(test_db_name);

        let drop_table = "DROP TABLE IF EXISTS Example";

        let _ = dbi.execute_write_at_host(test_db_name, &drop_table);

        let create_table_statement = "CREATE TABLE IF NOT EXISTS Example (Id INT, Name TEXT);";

        let _ = dbi.execute_write_at_host(test_db_name, &create_table_statement);

        let policy = LogicalStoragePolicy::HostOnly;

        let _ = dbi.set_logical_storage_policy(test_db_name, "Example", policy);

        let behavior = RemoteDeleteBehavior::Ignore;

        let _ = dbi.generate_contract(test_db_name, "test", "test", behavior);

        let add_example_record = "INSERT INTO Example (Id, Name) VALUES (1, 'Test_Record')";

        let _ = dbi.execute_write_at_host(test_db_name, add_example_record);
    } else {
        println!(
            "test_db already exists at: {}",
            &default_src_path.to_str().unwrap()
        );
    }
}
