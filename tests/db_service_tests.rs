pub mod test_harness;

#[path = "db_service_tests/accept_contract.rs"]
mod accept_contract;
#[path = "db_service_tests/insert_read_delete_remote_row.rs"]
mod insert_read_delete_remote_row;
#[path = "db_service_tests/insert_read_remote_row.rs"]
mod insert_read_remote_row;
#[path = "db_service_tests/insert_read_update_remote_row.rs"]
mod insert_read_update_remote_row;
#[path = "db_service_tests/insert_row.rs"]
mod insert_row;
#[path = "db_service_tests/save_contract.rs"]
mod save_contract;
