pub mod test_harness;

#[path = "db_service_tests/accept_contract_grpc.rs"]
mod accept_contract_grpc;

#[path = "db_service_tests/accept_contract_http.rs"]
mod accept_contract_http;

#[path = "db_service_tests/insert_row.rs"]
mod insert_row;
#[path = "db_service_tests/save_contract.rs"]
mod save_contract;

/*
# Test Module Overview

This module is intended to group tests related to ensuring the rcd db service is working correctly. This module may be deprecated in favor of expanding
the suite of tests from the client and the participant.

## Test Module Background
We want to make sure that all functionality with the db service is working correctly. Things such as:

- ensuring that a particpant can accept a contract
- insuring CRUD operations are working successfully between a host and a participant

and so on.

*/
