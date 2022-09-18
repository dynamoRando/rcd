#[path = "test_harness.rs"]
mod test_harness;

#[path = "client_service_tests/create_db_enable_coop_read_write.rs"]
mod create_db_enable_coop_read_write;
#[path = "client_service_tests/create_user_database.rs"]
mod create_user_database;
#[path = "client_service_tests/enable_cooperative_features.rs"]
mod enable_cooperative_features;
#[path = "client_service_tests/generate_contract.rs"]
mod generate_contract;
#[path = "client_service_tests/get_set_logical_storage_policy.rs"]
mod get_set_logical_storage_policy;
#[path = "client_service_tests/has_table.rs"]
mod has_table;
#[path = "client_service_tests/is_online.rs"]
mod is_online;

#[test]
fn get_harness_value() {
    let current = crate::test_harness::TEST_SETTINGS
        .lock()
        .unwrap()
        .get_current_port();
    let next = crate::test_harness::TEST_SETTINGS
        .lock()
        .unwrap()
        .get_next_avail_port();
    assert_eq!(current + 1, next);
}
