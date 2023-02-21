#[path = "participant_tests-update/update_at_participant.rs"]
mod update_at_participant;

#[path = "participant_tests-update/update_at_participant_negative.rs"]
mod update_at_participant_negative;

#[path = "participant_tests-update/update_from_host_queue_http.rs"]
mod update_from_host_queue_http;

#[path = "participant_tests-update/update_from_host_queue_grpc.rs"]
mod update_from_host_queue_grpc;

#[path = "participant_tests-update/update_from_host_queue_with_log_grpc.rs"]
mod update_from_host_queue_with_log_grpc;

#[path = "participant_tests-update/update_from_host_queue_with_log_http.rs"]
mod update_from_host_queue_with_log_http;

#[path = "participant_tests-update/update_from_host_with_log_grpc.rs"]
mod update_from_host_with_log_grpc;

#[path = "participant_tests-update/update_from_host_with_log_http.rs"]
mod update_from_host_with_log_http;

#[path = "participant_tests-update/change_update_from_host_behavior.rs"]
mod change_update_from_host_behavior;

#[path = "participant_tests-update/get_update_from_host_behavior.rs"]
mod get_update_from_host_behavior;

#[path = "participant_tests-update/get_update_at_participant.rs"]
mod get_update_at_participant;

pub mod test_harness;
pub mod test_common;

#[path = "participant_tests-update/common_http.rs"]
pub mod common_http;

#[path = "participant_tests-update/common_grpc.rs"]
pub mod common_grpc;
