#[path = "test_harness.rs"]
mod test_harness;

#[path = "client_service_tests/create_db_enable_coop_read_write.rs"]
mod create_db_enable_coop_read_write;
#[path = "client_service_tests/create_user_database.rs"]
mod create_user_database;
#[path = "client_service_tests/enable_cooperative_features.rs"]
mod enable_cooperative_features;

#[path = "client_service_tests/get_set_logical_storage_policy.rs"]
mod get_set_logical_storage_policy;
#[path = "client_service_tests/has_table.rs"]
mod has_table;
#[path = "client_service_tests/host_only.rs"]
mod host_only;
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

/*
# Test Module Overview

This module is intended to group tests related to ensuring the rcd sql client service is working correctly - mainly from the perspective of the host. While
the rcd sql client service presents actions for both a host and a participant, this module will mainly focus on actions that would be taken from a host.

For test that relate to the perspective of a participant, see the `participant_tests` module.

## Test Module Background
We want to make sure that all functionality with a host is working correctly. This includes basic actions such as:

- being able to create a database
- being able to generate a contract
- being able to enable cooperative features

and so on.

*/
