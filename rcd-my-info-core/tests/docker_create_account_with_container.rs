#[path = "test_harness.rs"]
mod test_harness;

use std::sync::Mutex;
use std::thread;

use crate::test_harness::get_test_temp_dir;
use crate::test_harness::is_docker_running;
use crate::test_harness::remove_container_if_exists;
use crate::test_harness::DOCKER_NOT_RUNNING_MESSAGE;
use log::debug;
use log::error;
use log::info;
use rcd_my_info_core::{admin::Admin, admin_db::DbType};
use simple_logger::SimpleLogger;
use std::sync::Arc;

#[test]
#[ignore = "code not finished"]
pub fn docker_create_account_with_container() {
    let res_log = SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init();
    if let Err(e) = res_log {
        println!("{e}");
    }

    let test_name = "CREATE_CONTAINER";
    let email = "tester@test.com";
    let docker_ip = Arc::new("tcp://127.0.0.1:2375");
    let pw = "dontlook";
    let docker_status = false;
    let docker_status = Mutex::new(docker_status);
    let docker_status = Arc::new(docker_status);

    {
        let docker_status = docker_status.clone();
        let docker_ip = docker_ip.clone();
        thread::spawn(move || {
            let mut data = docker_status.lock().unwrap();
            let is_running = is_docker_running(&docker_ip);
            debug!("is docker running: {is_running}");
            *data = is_running;
        })
        .join()
        .unwrap();
    }

    let docker_is_running = docker_status.lock().unwrap();

    if *docker_is_running {
        {
            let container_name = format!("{}{}", "/", email);
            let docker_ip = docker_ip.clone();
            thread::spawn(move || {
                test_setup(&docker_ip, &container_name);
            })
            .join()
            .unwrap();
        }

        let admin =
            Admin::new(DbType::Sqlite, get_test_temp_dir(test_name)).set_docker_ip(&docker_ip);
        let is_registered = admin.register_user(&email, pw);

        match is_registered {
            Ok(_) => {
                let is_provisioned = admin.provision_container_for_user(&email);
                match is_provisioned {
                    Ok(_is_provisioned) => {
                        todo!()
                    }
                    Err(e) => {
                        error!("{e}");
                    }
                }
            }
            Err(e) => {
                error!("{e}");
            }
        }
    } else {
        info!("{}", DOCKER_NOT_RUNNING_MESSAGE);
    }
}

#[tokio::main]
async fn test_setup(docker_ip: &str, container_name: &str) {
    remove_container_if_exists(docker_ip, &container_name).await;
}
