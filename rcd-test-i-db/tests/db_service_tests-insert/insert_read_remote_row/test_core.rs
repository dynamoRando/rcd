use crate::test_common::multi::setup_io::setup_main_and_participant;
use crate::test_harness::CoreTestConfig;

pub fn test_core(config: CoreTestConfig) {
    setup_main_and_participant(config.clone());
}
