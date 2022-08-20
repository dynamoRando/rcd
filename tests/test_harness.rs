use lazy_static::lazy_static;
use std::env;
use std::fs;
use std::{path::Path, sync::Mutex};
use log::info;
use std::{thread};

// http://oostens.me/posts/singletons-in-rust/
// we want to increment for all tests the ports used
// so that way we can run multiple client/servers

lazy_static! {
    pub static ref TEST_SETTINGS: Mutex<TestSettings> = Mutex::new(TestSettings { max_port: 6000 });
}

 /// returns a tuple for the addr_port of the client service and the db service
 #[allow(dead_code)]
 pub fn start_service(test_db_name: &str, root_dir: String) -> (String, String) {
    let client_port_num = TEST_SETTINGS
        .lock()
        .unwrap()
        .get_next_avail_port();

    let db_port_num = TEST_SETTINGS
        .lock()
        .unwrap()
        .get_next_avail_port();

    let service = rcd::get_service_from_config_file();

    let client_address_port =
        format!("{}{}", String::from("[::1]:"), client_port_num.to_string());
    let target_client_address_port = client_address_port.clone();

    let db_address_port = format!("{}{}", String::from("[::1]:"), db_port_num.to_string());
    let target_db_address_port = db_address_port.clone();

    println!("{:?}", &service);

    service.start_at_dir(root_dir.as_str());

    let cwd = service.cwd();
    delete_test_database(test_db_name, &cwd);

    info!("starting main client at {}", &client_address_port);
    info!("starting client service");

    let dir = root_dir.clone();

    thread::spawn(move || {
        let d = dir.clone();
        let e = d.clone();
        service
            .start_client_service_at_addr(client_address_port, d)
            .unwrap();
            service
            .start_db_service_at_addr(db_address_port, e)
            .unwrap();
    });

    return (target_client_address_port, target_db_address_port);
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

/// returns a tuple for the root directory, the "main" directory, and the "participant" directory 
/// in the temp folder
#[allow(dead_code)]
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

#[allow(dead_code)]
pub struct TestSettings {
    max_port: u32,
}

impl TestSettings {
    #[allow(dead_code)]
    pub fn get_next_avail_port(&mut self) -> u32 {
        self.max_port = self.max_port + 1;
        return self.max_port;
    }
    #[allow(dead_code)]
    pub fn get_current_port(&self) -> u32 {
        return self.max_port;
    }
}

#[allow(dead_code)]
pub fn delete_test_database(db_name: &str, cwd: &str) {
    let db_path = Path::new(&cwd).join(db_name);

    if db_path.exists() {
        fs::remove_file(&db_path).unwrap();
    }
}
