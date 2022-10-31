use log::info;
use log4rs;
use rcd_common::defaults;
use rcd_core::comm::{RcdCommunication, RcdRemoteDbClient};
use rcd_core::dbi::Dbi;
use rcd_core::rcd::Rcd;
use rcd_core::remote_grpc::RemoteGrpc;
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
    process_cmd_args(args);
    set_default_config();
    let mut service = get_service_from_config_file();

    println!("rcd settings found:");
    println!("{:?}", service.rcd_settings);
    println!("root dir: {}", service.root_dir);
    service.start();

    let dbi_settings = service.get_dbi();
    let dbi_clone = dbi_settings.clone();

    let settings = service.rcd_settings.clone();
    let db_name = settings.backing_database_name.clone();
    let client_port = settings.client_service_addr_port.clone();
    let db_port = settings.database_service_addr_port.clone();
    let root_dir = service.root_dir.clone();

    let _ = task::spawn_blocking(move || {
        let _ = service.start_grpc_at_addrs_with_shutdown(
            db_name,
            client_port,
            db_port,
            root_dir.to_string(),
            client_listener,
            db_listener,
        );
    })
    .await;

    // start http, need to make this configurable
    let _ = task::spawn_blocking(move || {
        let http = RemoteHttp {};

        let remote_client = RcdRemoteDbClient {
            comm_type: RcdCommunication::Http,
            grpc: None,
            http: Some(http),
        };

        let core = Rcd {
            db_interface: Some(dbi_clone),
            remote_client: Some(remote_client),
        };

        let _ = http_srv::start_http(core);
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

fn process_cmd_args(args: Vec<String>) {
    if args.len() >= 2 {
        let cmd = &args[1];
        if cmd == "default_settings" {
            set_default_config();
        }
    }
}

fn set_default_config() {
    let cwd = get_current_directory();
    let default_settings_content = String::from(
        "
debug = false
database_type = 1
backing_database_name = \"rcd.db\"
rcd_schema = \"rcd\"
client_service_addr_port = \"0.0.0.0:50051\"
data_service_addr_port = \"0.0.0.0:50052\"
admin_un = \"tester\"
admin_pw = \"123456\"
    ",
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

#[allow(dead_code)]
fn configure_rcd_w_http_new(dbi: Dbi) -> Rcd {
    let http = RemoteHttp {};

    let remote_db = RcdRemoteDbClient {
        comm_type: RcdCommunication::Http,
        grpc: None,
        http: Some(http),
    };

    let rcd = Rcd {
        db_interface: Some(dbi.clone()),
        remote_client: Some(remote_db),
    };

    return rcd;
}

#[allow(dead_code)]
fn configure_rcd_w_grpc_new(dbi: Dbi, db_addr_port: String) -> Rcd {
    let grpc = RemoteGrpc {
        db_addr_port: db_addr_port,
    };

    let remote_db = RcdRemoteDbClient {
        comm_type: RcdCommunication::Grpc,
        grpc: Some(grpc),
        http: None,
    };

    let rcd = Rcd {
        db_interface: Some(dbi.clone()),
        remote_client: Some(remote_db),
    };

    return rcd;
}
