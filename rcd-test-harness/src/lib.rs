use core::time;
use fern::colors::Color;
use fern::colors::ColoredLevelConfig;
use lazy_static::lazy_static;
use tracing::error;
use tracing::info;
use rcd_client::client_type::RcdClientType;
use rcd_client::RcdClient;
use rcd_test_harness_common::get_test_temp_dir;
use simple_logger::SimpleLogger;
use std::fs;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::{path::Path, sync::Mutex};
use test_common::GrpcTestSetup;
use test_common::HttpTestSetup;
use triggered::Trigger;

pub mod grpc;
pub mod http;
pub mod test_common;

// http://oostens.me/posts/singletons-in-rust/
// we want to increment for all tests the ports used
// so that way we can run multiple client/servers

#[derive(Debug, Clone)]
pub enum AddrType {
    Client,
    Database,
}

#[derive(Debug, Clone)]
pub struct RcdClientConfig {
    pub addr: ServiceAddr,
    pub client_type: RcdClientType,
    pub host_id: Option<String>,
    pub auth: Option<RcdClientAuth>,
}

#[derive(Debug, Clone)]
pub struct RcdClientAuth {
    pub un: String,
    pub pw: String,
}

#[derive(Debug, Clone)]
pub struct ServiceAddr {
    pub ip4_addr: String,
    pub port: u32,
    pub addr_type: AddrType,
}

#[derive(Debug, Clone)]
pub struct TestConfigGrpc {
    pub client_address: ServiceAddr,
    pub database_address: ServiceAddr,
    pub client_service_shutdown_trigger: Trigger,
    pub database_service_shutdown_trigger: Trigger,
    pub client_keep_alive: Sender<bool>,
}

#[derive(Debug, Clone)]
pub struct TestConfigHttp {
    pub http_address: ServiceAddr,
    pub keep_alive: Sender<bool>,
}

#[derive(Debug, Clone)]
pub struct TestDirectoryConfig {
    pub root_dir: String,
    pub main_dir: String,
    pub participant_dir: String,
}

#[derive(Debug, Clone)]
pub struct CoreTestConfig {
    pub main_client: RcdClientConfig,
    pub participant_client: Option<RcdClientConfig>,
    pub test_db_name: String,
    pub contract_desc: Option<String>,
    pub participant_db_addr: Option<ServiceAddr>,
    pub grpc_test_setup: Option<GrpcTestSetup>,
    pub http_test_setup: Option<HttpTestSetup>,
    pub participant_id: Option<String>,
}

impl ServiceAddr {
    #[allow(dead_code)]
    pub fn to_full_string(&self) -> String {
        format!("{}{}", self.ip4_addr, self.port)
    }
    #[allow(dead_code)]
    pub fn to_full_string_with_http(&self) -> String {
        format!("{}{}", String::from("http://"), self.to_full_string())
    }
}

lazy_static! {
    pub static ref TEST_SETTINGS: Mutex<TestSettings> = Mutex::new(TestSettings {
        max_port: 9000,
        ports: Vec::new()
    });
}

pub fn release_port(port: u32) {
    TEST_SETTINGS.lock().unwrap().release_port(port);
}

pub fn get_next_avail_port() -> u32 {
    return TEST_SETTINGS.lock().unwrap().get_next_avail_port();
}

pub fn sleep_test_for_seconds(seconds: u32) {
    let time = time::Duration::from_secs(seconds as u64);
    info!("sleeping for {} seconds...", seconds.to_string());
    thread::sleep(time);
    // tokio::time::sleep(time).await;
}

pub fn sleep_test() {
    sleep_test_for_seconds(1);
}

pub fn sleep_instance() {
    sleep_test_for_seconds(2);
}

/// overrides RCD's default logger to log to screen for the specified logging level with Simple Logger
pub fn init_log_to_screen(level: log::LevelFilter) {
    let res_log = SimpleLogger::new().with_level(level).init();
    if let Err(e) = res_log {
        error!("{e}");
    }
}

pub fn init_log_to_screen_fern(level: log::LevelFilter) {
    use ignore_result::Ignore;

    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Blue)
        .error(Color::BrightRed)
        .warn(Color::Magenta);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(level)
        .level_for("tokio", log::LevelFilter::Off)
        .level_for("hyper", log::LevelFilter::Off)
        .level_for("rocket", log::LevelFilter::Off)
        .level_for("h2", log::LevelFilter::Off)
        .level_for("tower", log::LevelFilter::Off)
        .level_for("_", log::LevelFilter::Off)
        .chain(std::io::stdout())
        .apply()
        .ignore();
}

pub fn start_keepalive_for_test(client_type: RcdClientType, addr: ServiceAddr) -> Sender<bool> {
    let (tx_main, rx_main) = mpsc::channel();

    // main - normal database setup
    thread::spawn(move || {
        let _ = keep_alive(client_type, addr, rx_main);
    })
    .join()
    .unwrap();

    tx_main
}

async fn keep_alive(client_type: RcdClientType, addr: ServiceAddr, reciever: Receiver<bool>) {
    let sleep_in_seconds = 1;

    match client_type {
        RcdClientType::Grpc => {
            let mut client = RcdClient::new_grpc_client(
                addr.to_full_string_with_http(),
                String::from("tester"),
                String::from("123456"),
                5,
            )
            .await;

            while reciever.try_recv().unwrap() {
                let time = time::Duration::from_secs(sleep_in_seconds as u64);
                tokio::time::sleep(time).await;
                let _ = client.is_online().await;
            }
        }
        RcdClientType::Http => {
            let mut client = RcdClient::new_http_client(
                String::from("tester"),
                String::from("123456"),
                5,
                addr.ip4_addr,
                addr.port,
            );

            while reciever.try_recv().unwrap() {
                let time = time::Duration::from_secs(sleep_in_seconds as u64);
                tokio::time::sleep(time).await;
                let _ = client.is_online().await;
            }
        }
    };
}

/// returns a tuple for the root directory, the "main" directory, and the "participant" directory
/// in the temp folder
pub fn get_test_temp_dir_main_and_participant(test_name: &str) -> TestDirectoryConfig {
    let root_dir = get_test_temp_dir(test_name);

    let main_path = Path::new(&root_dir).join("main");

    if main_path.exists() {
        fs::remove_dir_all(&main_path).unwrap();
    }

    fs::create_dir_all(&main_path).unwrap();

    let main_dir = main_path.as_os_str().to_str().unwrap();

    let participant_path = Path::new(&root_dir).join("participant");

    if participant_path.exists() {
        fs::remove_dir_all(&participant_path).unwrap();
    }

    fs::create_dir_all(&participant_path).unwrap();

    let participant_dir = participant_path.as_os_str().to_str().unwrap();

    TestDirectoryConfig {
        root_dir,
        main_dir: main_dir.to_string(),
        participant_dir: participant_dir.to_string(),
    }
}

pub struct TestSettings {
    max_port: u32,
    ports: Vec<u32>,
}

impl TestSettings {
    /// tracks the next defined port available in the collection
    /// note: this will sleep the thread for 1 second
    pub fn get_next_avail_port(&mut self) -> u32 {
        sleep_test_for_seconds(1);

        if self.ports.is_empty() {
            self.max_port += 1;
            self.ports.push(self.max_port);
            self.max_port
        } else {
            let val = *self.ports.iter().max().unwrap() + 1;
            self.ports.push(val);
            val
        }
    }

    pub fn get_current_port(&self) -> u32 {
        if self.ports.is_empty() {
            self.max_port
        } else {
            *self.ports.iter().max().unwrap()
        }
    }

    pub fn release_port(&mut self, port: u32) {
        if self.ports.contains(&port) {
            let index = self.ports.iter().position(|x| *x == port).unwrap();
            self.ports.remove(index);
        }
    }
}

pub fn delete_test_database(db_name: &str, cwd: &str) {
    let db_path = Path::new(&cwd).join(db_name);

    if db_path.exists() {
        fs::remove_file(&db_path).unwrap();
    }
}

pub async fn get_rcd_client(config: &RcdClientConfig) -> RcdClient {
    info!("get_rcd_client: {config:?}");

    match config.client_type {
        RcdClientType::Grpc => {
            let un: String;
            let pw: String;

            if config.auth.is_none() {
                un = String::from("tester");
                pw = String::from("123456");
            } else {
                un = config.auth.as_ref().unwrap().un.clone();
                pw = config.auth.as_ref().unwrap().pw.clone();
            }

            if config.host_id.is_none() {
                return RcdClient::new_grpc_client(
                    config.addr.to_full_string_with_http(),
                    un,
                    pw,
                    60,
                )
                .await;
            }

            let mut client =
                RcdClient::new_grpc_client(config.addr.to_full_string_with_http(), un, pw, 60)
                    .await;

            client.set_host_id(&config.host_id.as_ref().unwrap());
            client
        }
        RcdClientType::Http => {
            if config.host_id.is_none() {
                return RcdClient::new_http_client(
                    String::from("tester"),
                    String::from("123456"),
                    60,
                    config.addr.ip4_addr.clone(),
                    config.addr.port,
                );
            }
            let mut client = RcdClient::new_http_client(
                String::from("tester"),
                String::from("123456"),
                60,
                config.addr.ip4_addr.clone(),
                config.addr.port,
            );

            client.set_host_id(&config.host_id.as_ref().unwrap());
            client
        }
    }
}
