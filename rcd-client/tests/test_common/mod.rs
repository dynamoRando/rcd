pub mod multi;

use crate::test_harness::{TestConfigGrpc, TestConfigHttp};

#[derive(Debug, Clone)]
pub struct GrpcTestSetup<'a> {
    pub main_test_config: TestConfigGrpc,
    pub participant_test_config: TestConfigGrpc,
    pub database_name: &'a str,
    pub contract_description: &'a str,
}

#[derive(Debug, Clone)]
pub struct HttpTestSetup<'a> {
    pub main_test_config: TestConfigHttp,
    pub participant_test_config: TestConfigHttp,
    pub database_name: &'a str,
    pub contract_description: &'a str,
}
