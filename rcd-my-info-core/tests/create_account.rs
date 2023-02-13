use crate::test_harness::get_test_temp_dir;
use rcd_my_info_core::{admin::Admin, admin_db::DbType};
use simple_logger::SimpleLogger;

#[path = "test_harness.rs"]
mod test_harness;

#[test]
pub fn create_account() {
    SimpleLogger::new().init().unwrap();

    let admin = Admin::new(DbType::Sqlite, get_test_temp_dir("CREATE_ACCOUNT"));
    let email = "tester@test.com";
    let pw = "dontlook";

    let result = admin.register_user(email, pw);
    if let Ok(registration) = result {
        assert!(registration);
        let is_valid = admin.verify_login(email, pw);
        if let Ok(validated) = is_valid {
            assert!(validated)
        }

        let is_invalid_result = admin.verify_login(email, "THISISWRONG");
        if let Ok(is_invalid) = is_invalid_result {
            assert!(!is_invalid);
        }
    }
}
