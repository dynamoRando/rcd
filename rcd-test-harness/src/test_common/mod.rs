use crate::{RcdClientConfig, TestConfigGrpc, TestConfigHttp};
pub mod multi;
pub mod proxy;

#[derive(Debug, Clone)]
pub struct GrpcTestSetup {
    pub main_test_config: TestConfigGrpc,
    pub participant_test_config: Option<TestConfigGrpc>,
    pub database_name: String,
    pub contract_description: String,
    pub main_client: RcdClientConfig,
    pub participant_client: Option<RcdClientConfig>,
}

#[derive(Debug, Clone)]
pub struct HttpTestSetup {
    pub main_test_config: TestConfigHttp,
    pub participant_test_config: TestConfigHttp,
    pub database_name: String,
    pub contract_description: String,
    pub main_client: RcdClientConfig,
    pub participant_client: Option<RcdClientConfig>,
}
