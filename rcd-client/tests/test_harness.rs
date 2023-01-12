use core::time;
use lazy_static::lazy_static;
use log::info;
use rcd_client::client_type::RcdClientType;
use rcd_client::RcdClient;
use rcdx::rcd_service::get_service_from_config_file;
use rcdx::rcd_service::RcdService;
use std::env;
use std::fs;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::{path::Path, sync::Mutex};
use triggered::Trigger;

// http://oostens.me/posts/singletons-in-rust/
// we want to increment for all tests the ports used
// so that way we can run multiple client/servers

#[derive(Clone)]
pub enum AddrType {
    Client,
    Database,
}

#[derive(Clone)]
pub struct ServiceAddr {
    pub ip4_addr: String,
    pub port: u32,
    pub addr_type: AddrType,
}

impl ServiceAddr {
    #[allow(dead_code)]
    pub fn to_full_string(self: &Self) -> String {
        return format!("{}{}", self.ip4_addr, self.port.to_string());
    }
    #[allow(dead_code)]
    pub fn to_full_string_with_http(self: &Self) -> String {
        return format!("{}{}", String::from("http://"), self.to_full_string());
    }
}

lazy_static! {
    pub static ref TEST_SETTINGS: Mutex<TestSettings> = Mutex::new(TestSettings {
        max_port: 6000,
        ports: Vec::new()
    });
}

#[allow(dead_code)]
pub fn release_port(port: u32) {
    TEST_SETTINGS.lock().unwrap().release_port(port);
}

#[allow(dead_code)]
pub fn get_next_avail_port() -> u32 {
    return TEST_SETTINGS.lock().unwrap().get_next_avail_port();
}

#[allow(dead_code)]
/// returns a tuple for the addr_port of the client service and the db service
pub fn start_service_with_http(
    test_db_name: &str,
    root_dir: String,
) -> (ServiceAddr, Sender<bool>) {
    let http_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();
    let mut service = get_service_from_config_file(None);

    let http_addr = ServiceAddr {
        ip4_addr: "127.0.0.1".to_string(),
        port: http_port_num,
        addr_type: AddrType::Client,
    };

    println!("{:?}", &service);
    println!("{:?}", &root_dir);

    service.start_at_dir(root_dir.as_str());

    let cwd = service.cwd();
    delete_test_database(test_db_name, &cwd);

    println!("{:?}", &test_db_name);
    println!("{:?}", &cwd);

    let _ =
        service.start_http_at_addr_and_dir("127.0.0.1".to_string(), http_port_num as u16, root_dir);

    let keep_alive = start_keepalive_for_test(RcdClientType::Grpc, http_addr.clone());
    let _ = keep_alive.send(true);

    sleep_instance();

    return (http_addr, keep_alive);
}

#[allow(dead_code)]
pub fn shutdown_http(addr: String, port: u32) {
    RcdService::shutdown_http(addr, port);
}

#[allow(dead_code)]
pub fn sleep_test_for_seconds(seconds: u32) {
    let time = time::Duration::from_secs(seconds as u64);
    info!("sleeping for {} seconds...", seconds.to_string());
    thread::sleep(time);
    // tokio::time::sleep(time).await;
}

#[allow(dead_code)]
pub fn sleep_test() {
    sleep_test_for_seconds(1);
}

#[allow(dead_code)]
pub fn sleep_instance() {
    sleep_test_for_seconds(2);
}

#[allow(dead_code)]
pub fn start_keepalive_for_test(client_type: RcdClientType, addr: ServiceAddr) -> Sender<bool> {
    let (tx_main, rx_main) = mpsc::channel();

    // main - normal database setup
    thread::spawn(move || {
        let _ = keep_alive(client_type, addr, rx_main);
    })
    .join()
    .unwrap();

    return tx_main;
}

#[allow(dead_code)]
async fn keep_alive(client_type: RcdClientType, addr: ServiceAddr, reciever: Receiver<bool>) {
    let sleep_in_seconds = 1;

    match client_type {
        RcdClientType::Grpc => {
            let mut client = RcdClient::new_grpc_client(
                addr.to_full_string_with_http(),
                String::from("tester"),
                String::from("123456"),
                5,
            ).await;

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

#[allow(dead_code)]
/// returns a tuple for the addr_port of the client service and the db service
pub fn start_service_with_grpc(
    test_db_name: &str,
    root_dir: String,
) -> (
    ServiceAddr,
    ServiceAddr,
    u32,
    u32,
    Trigger,
    Trigger,
    Sender<bool>,
) {
    let (client_trigger, client_listener) = triggered::trigger();
    let (db_trigger, db_listener) = triggered::trigger();

    let client_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();
    let db_port_num = TEST_SETTINGS.lock().unwrap().get_next_avail_port();

    let mut service = get_service_from_config_file(None);

    let client_address_port = format!("{}{}", String::from("[::1]:"), client_port_num.to_string());

    let client_addr = ServiceAddr {
        ip4_addr: "[::1]:".to_string(),
        port: client_port_num,
        addr_type: AddrType::Client,
    };

    let db_address_port = format!("{}{}", String::from("[::1]:"), db_port_num.to_string());

    let db_addr = ServiceAddr {
        ip4_addr: "[::1]:".to_string(),
        port: db_port_num,
        addr_type: AddrType::Database,
    };

    println!("{:?}", &service);
    println!("{:?}", &root_dir);

    service.start_at_dir(root_dir.as_str());

    let db_name = service.rcd_settings.backing_database_name.clone();

    let cwd = service.cwd();
    delete_test_database(test_db_name, &cwd);

    let dir = root_dir.clone();

    let _ = service.start_grpc_at_addrs_with_shutdown(
        db_name,
        client_address_port,
        db_address_port,
        dir.clone(),
        client_listener,
        db_listener,
        5,
    );

    let keep_alive = start_keepalive_for_test(RcdClientType::Grpc, client_addr.clone());
    let _ = keep_alive.send(true);

    sleep_instance();

    return (
        client_addr,
        db_addr,
        client_port_num,
        db_port_num,
        client_trigger,
        db_trigger,
        keep_alive,
    );
}

#[allow(dead_code)]
pub fn get_test_temp_dir(test_name: &str) -> String {
    let dir = env::temp_dir();
    let tmp = dir.as_os_str().to_str().unwrap();
    let path = Path::new(&tmp).join("RCD_TESTS").join(test_name);

    if path.exists() {
        fs::remove_dir_all(&path).unwrap();
    }

    fs::create_dir_all(&path).unwrap();

    return path.as_path().to_str().unwrap().to_string();
}

#[allow(dead_code)]
/// returns a tuple for the root directory, the "main" directory, and the "participant" directory
/// in the temp folder
pub fn get_test_temp_dir_main_and_participant(test_name: &str) -> (String, String, String) {
    let root_dir = get_test_temp_dir(&test_name);

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

    return (root_dir, main_dir.to_string(), participant_dir.to_string());
}

pub struct TestSettings {
    max_port: u32,
    ports: Vec<u32>,
}

impl TestSettings {
    pub fn get_next_avail_port(&mut self) -> u32 {
        sleep_test_for_seconds(1);

        if self.ports.len() == 0 {
            self.max_port = self.max_port + 1;
            self.ports.push(self.max_port);
            return self.max_port;
        } else {
            let val = *self.ports.iter().max().unwrap() + 1;
            self.ports.push(val);
            return val;
        }
    }

    #[allow(dead_code)]
    pub fn get_current_port(&self) -> u32 {
        if self.ports.len() == 0 {
            return self.max_port;
        } else {
            *self.ports.iter().max().unwrap()
        }
    }

    pub fn release_port(&mut self, port: u32) {
        let index = self.ports.iter().position(|x| *x == port).unwrap();
        self.ports.remove(index);
    }
}

#[allow(dead_code)]
pub fn delete_test_database(db_name: &str, cwd: &str) {
    let db_path = Path::new(&cwd).join(db_name);

    if db_path.exists() {
        fs::remove_file(&db_path).unwrap();
    }
}
