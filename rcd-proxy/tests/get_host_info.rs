use rcd_proxy::test_rcd_common_setup;

use ignore_result::Ignore;
use simple_logger::SimpleLogger;
use crate::proxy_test_harness::harness_common_setup;
mod proxy_test_harness;

#[tokio::test]
pub async fn test_get_host_info_grpc() {    
    SimpleLogger::new().env().init().ignore();

    let setup = harness_common_setup("proxy-t-i-get-host-info").unwrap();


}