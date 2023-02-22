use crate::{TestConfigGrpc, TestConfigHttp};
pub mod multi;

#[derive(Debug, Clone)]
pub struct GrpcTestSetup {
    pub main_test_config: TestConfigGrpc,
    pub participant_test_config: TestConfigGrpc,
    pub database_name: String,
    pub contract_description: String,
}

#[derive(Debug, Clone)]
pub struct HttpTestSetup {
    pub main_test_config: TestConfigHttp,
    pub participant_test_config: TestConfigHttp,
    pub database_name: String,
    pub contract_description: String,
}
