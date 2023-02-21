
    use crate::test_harness::{self, ServiceAddr};
    use log::{debug, info};
    use rcd_client::RcdClient;
    use std::sync::{mpsc, Arc};
    use std::thread;

    use super::test_core::{test_core, CoreTestConfig};

    #[test]
    fn test() {
        /*
            We will need to kick off two services, the host and the participant
            and we will need to also kick off two clients, one for each
        */

        let test_name = "insert_remote_row_http";
        let db = Arc::new(format!("{}{}", test_name, ".db"));
        let contract = Arc::new(String::from("insert remote row"));

        let dirs = test_harness::get_test_temp_dir_main_and_participant(test_name);
        let main_test_config = test_harness::http::start_service_with_http(&db, dirs.main_dir);
        let participant_test_config = test_harness::http::start_service_with_http(&db, dirs.participant_dir);

        test_harness::sleep_test();

        thread::spawn(move || {
            let mc = test_harness::get_http_rcd_client(main_test_config.http_address.clone());
            let pc = test_harness::get_http_rcd_client(participant_test_config.http_address.clone());
            let pda = participant_test_config.http_address.clone();
    
            let config = CoreTestConfig {
                main_client: &mc,
                participant_client: &pc,
                test_db_name: &db,
                contract_desc: &contract,
                participant_db_addr: &pda,
            };
    
            test_core(&config);
        })
        .join()
        .unwrap();

        test_harness::http::shutdown_http_test(&main_test_config, &participant_test_config);
    }
