use rcd_client::client_type::RcdClientType;
use std::thread;

use crate::{
    get_test_temp_dir, get_test_temp_dir_main_and_participant,
    grpc::{shutdown_grpc_tests, start_service_with_grpc},
    http::{shutdown_http_tests, start_service_with_http},
    sleep_test,
    test_common::{GrpcTestSetup, HttpTestSetup},
    CoreTestConfig, 
};

#[derive(Debug, Clone)]
pub struct RunnerConfig {
    pub test_name: String,
    pub contract_desc: Option<String>,
    pub use_internal_logging: bool,
}

pub struct TestRunner {}

impl TestRunner {
    /// takes a config for a test and will begin an HTTP GRPC test, using the
    /// provided `test_core` function to run
    pub fn run_grpc_test(config: RunnerConfig, test_core: fn(CoreTestConfig)) {
        let db = format!("{}{}", config.test_name, ".db");
        let root_dir = get_test_temp_dir(&config.test_name);
        let main_test_config = start_service_with_grpc(&db, root_dir, config.use_internal_logging);

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let mc = crate::RcdClientConfig {
                addr: mtc.client_address,
                client_type: RcdClientType::Grpc,
            };

            thread::spawn(move || {
                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: None,
                    test_db_name: db,
                    contract_desc: None,
                    participant_db_addr: None,
                    grpc_test_setup: None,
                    http_test_setup: None,
                };

                test_core(config);
            })
            .join()
            .unwrap();
        }

        let instances = vec![&main_test_config];
        shutdown_grpc_tests(instances);
    }

    /// takes a config for a test and will begin an HTTP GRPC test, using the
    /// provided `test_core` function to run
    pub fn run_grpc_test_multi(config: RunnerConfig, test_core: fn(CoreTestConfig)) {
        let db = format!("{}{}", config.test_name, ".db");
        let dirs = get_test_temp_dir_main_and_participant(&config.test_name);

        let main_test_config =
            start_service_with_grpc(&db, dirs.main_dir, config.use_internal_logging);
        let participant_test_config =
            start_service_with_grpc(&db, dirs.participant_dir, config.use_internal_logging);

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();
            let contract = config.contract_desc;

            thread::spawn(move || {
                let mc = crate::RcdClientConfig {
                    addr: mtc.client_address.clone(),
                    client_type: RcdClientType::Grpc,
                };
                let pc = crate::RcdClientConfig {
                    addr: ptc.client_address.clone(),
                    client_type: RcdClientType::Grpc,
                };
                let pda = ptc.database_address.clone();

                let grpc_test_setup = GrpcTestSetup {
                    main_test_config: mtc.clone(),
                    participant_test_config: Some(ptc.clone()),
                    database_name: db.clone(),
                    contract_description: contract.as_ref().unwrap().clone(),
                    main_client: mc.clone(),
                    participant_client: Some(pc.clone()),
                };

                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: Some(pc),
                    test_db_name: db,
                    contract_desc: contract,
                    participant_db_addr: Some(pda),
                    grpc_test_setup: Some(grpc_test_setup),
                    http_test_setup: None,
                };

                test_core(config);
            })
            .join()
            .unwrap();
        }

        let instances = vec![&main_test_config, &participant_test_config];
        shutdown_grpc_tests(instances);
    }

    /// takes a config for a test and will begin an HTTP RCD test, using the
    /// provided `test_core` function to run for a main and a participant
    pub fn run_http_test_multi(config: RunnerConfig, test_core: fn(CoreTestConfig)) {
        let db = format!("{}{}", config.test_name, ".db");

        let dirs = get_test_temp_dir_main_and_participant(&config.test_name);
        let main_test_config =
            start_service_with_http(&db, dirs.main_dir, config.use_internal_logging);
        let participant_test_config =
            start_service_with_http(&db, dirs.participant_dir, config.use_internal_logging);

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();

            thread::spawn(move || {
                let mc = crate::RcdClientConfig {
                    addr: mtc.http_address.clone(),
                    client_type: RcdClientType::Http,
                };
                let pc = crate::RcdClientConfig {
                    addr: ptc.http_address.clone(),
                    client_type: RcdClientType::Http,
                };

                let ptc = ptc.clone();
                let pda = ptc.http_address.clone();
                let contract = config.contract_desc.clone();

                let http_test_setup = HttpTestSetup {
                    main_test_config: mtc.clone(),
                    participant_test_config: ptc,
                    database_name: db.clone(),
                    contract_description: contract.as_ref().unwrap().clone(),
                    main_client: mc.clone(),
                    participant_client: Some(pc.clone()),
                };

                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: Some(pc),
                    test_db_name: db,
                    contract_desc: contract,
                    participant_db_addr: Some(pda),
                    grpc_test_setup: None,
                    http_test_setup: Some(http_test_setup),
                };

                test_core(config);
            })
            .join()
            .unwrap();
        }

        let tests = vec![&main_test_config, &participant_test_config];

        shutdown_http_tests(tests);
    }

    /// takes a config for a test and will begin an HTTP RCD test, using the
    /// provided `test_core` function to run
    pub fn run_http_test(config: RunnerConfig, test_core: fn(CoreTestConfig)) {
        let db = format!("{}{}", config.test_name, ".db");

        let dirs = get_test_temp_dir_main_and_participant(&config.test_name);
        let main_test_config = 
        start_service_with_http(&db, dirs.main_dir, config.use_internal_logging);

        sleep_test();

        {
            let mtc = main_test_config.clone();

            thread::spawn(move || {
                let mc = crate::RcdClientConfig {
                    addr: mtc.http_address,
                    client_type: RcdClientType::Http,
                };

                let contract = config.contract_desc.clone();

                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: None,
                    test_db_name: db,
                    contract_desc: contract,
                    participant_db_addr: None,
                    grpc_test_setup: None,
                    http_test_setup: None,
                };

                test_core(config);
            })
            .join()
            .unwrap();
        }

        let tests = vec![&main_test_config];
        shutdown_http_tests(tests);
    }
}
