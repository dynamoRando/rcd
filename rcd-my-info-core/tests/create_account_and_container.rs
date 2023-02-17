#[path = "test_harness.rs"]
mod test_harness;

use crate::test_harness::get_test_temp_dir;
use log::error;
use rcd_my_info_core::{admin::Admin, admin_db::DbType};
use simple_logger::SimpleLogger;

#[test]
#[ignore = "code not finished"]
pub fn create_account_with_container() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let email = "tester@test.com";
    let pw = "dontlook";
    let docker_ip = "tcp://127.0.0.1:2375";
    let admin =
        Admin::new(DbType::Sqlite, get_test_temp_dir("CREATE_CONTAINER")).set_docker_ip(docker_ip);
    let is_registered_result = admin.register_user(email, pw);

    match is_registered_result {
        Ok(_) => {
            let is_provisioned_result = admin.provision_container_for_user(email);
            match is_provisioned_result {
                Ok(is_provisioned) => {
                    todo!()
                },
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
