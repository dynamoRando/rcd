use std::thread;

use crate::{
    get_grpc_rcd_client, get_http_rcd_client, get_test_temp_dir,
    get_test_temp_dir_main_and_participant,
    grpc::{shutdown_grpc_tests, start_service_with_grpc},
    http::{shutdown_http_tests, start_service_with_http},
    sleep_test, CoreTestConfig,
};

#[derive(Debug, Clone)]
pub struct RunnerConfig {
    pub test_name: String,
    pub contract_desc: Option<String>,
}

pub struct TestRunner {}

impl TestRunner {
    /// takes a config for a test and will begin an HTTP GRPC test, using the
    /// provided `test_core` function to run
    pub fn run_grpc_test(config: RunnerConfig, test_core: fn(CoreTestConfig)) {
        let db = format!("{}{}", config.test_name, ".db");
        let root_dir = get_test_temp_dir(&config.test_name);
        let main_test_config = start_service_with_grpc(&db, root_dir);

        sleep_test();

        {
            let mtc = main_test_config.clone();
        
            thread::spawn(move || {
                let mc = get_grpc_rcd_client(mtc.client_address.clone());

                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: None,
                    test_db_name: db,
                    contract_desc: None,
                    participant_db_addr: None,
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

        let main_test_config = start_service_with_grpc(&db, dirs.main_dir);
        let participant_test_config = start_service_with_grpc(&db, dirs.participant_dir);

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();
            let contract = config.contract_desc.clone();

            thread::spawn(move || {
                let mc = get_grpc_rcd_client(mtc.client_address.clone());
                let pc = get_grpc_rcd_client(ptc.client_address.clone());
                let pda = ptc.database_address.clone();

                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: Some(pc),
                    test_db_name: db,
                    contract_desc: contract,
                    participant_db_addr: Some(pda),
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
        let main_test_config = start_service_with_http(&db, dirs.main_dir);
        let participant_test_config = start_service_with_http(&db, dirs.participant_dir);

        sleep_test();

        {
            let mtc = main_test_config.clone();
            let ptc = participant_test_config.clone();

            thread::spawn(move || {
                let mc = get_http_rcd_client(mtc.http_address.clone());
                let pc = get_http_rcd_client(ptc.http_address.clone());
                let pda = ptc.http_address.clone();
                let contract = config.contract_desc.clone();

                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: Some(pc),
                    test_db_name: db,
                    contract_desc: contract,
                    participant_db_addr: Some(pda),
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
        let main_test_config = start_service_with_http(&db, dirs.main_dir);

        sleep_test();

        {
            let mtc = main_test_config.clone();

            thread::spawn(move || {
                let mc = get_http_rcd_client(mtc.http_address.clone());

                let contract = config.contract_desc.clone();

                let config = CoreTestConfig {
                    main_client: mc,
                    participant_client: None,
                    test_db_name: db,
                    contract_desc: contract,
                    participant_db_addr: None,
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
