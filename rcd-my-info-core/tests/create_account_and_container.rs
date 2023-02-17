#[path = "test_harness.rs"]
mod test_harness;

use std::thread;

use crate::test_harness::get_test_temp_dir;
use crate::test_harness::remove_container_if_exists;
use log::error;
use rcd_my_info_core::{admin::Admin, admin_db::DbType};
use simple_logger::SimpleLogger;
use std::sync::Arc;

#[test]
#[ignore = "code not finished"]
pub fn create_account_with_container() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let test_name = "CREATE_CONTAINER";
    let email = "tester@test.com";
    let docker_ip = Arc::new("tcp://127.0.0.1:2375");
    let pw = "dontlook";

    {
        let container_name = format!("{}{}", "/", email);
        let docker_ip = docker_ip.clone();
        thread::spawn(move || {
            test_setup(&docker_ip, &container_name);
        })
        .join()
        .unwrap();
    }

    let admin = Admin::new(DbType::Sqlite, get_test_temp_dir(test_name)).set_docker_ip(&docker_ip);
    let is_registered = admin.register_user(&email, pw);

    match is_registered {
        Ok(_) => {
            let is_provisioned = admin.provision_container_for_user(&email);
            match is_provisioned {
                Ok(is_provisioned) => {
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
}

#[tokio::main]
async fn test_setup(docker_ip: &str, container_name: &str) {
    remove_container_if_exists(docker_ip, &container_name).await;
}
