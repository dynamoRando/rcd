use crate::test_harness::CoreTestConfig;
use log::debug;
use std::sync::{mpsc, Arc};
use std::thread;

use crate::test_common::multi::common_actions::
{main_service_client, participant_service_client, main_execute_coop_write_and_read };

/// sets up a main and a participant to accept a contract and verifies via i/o that the 
/// main has i/o at the participant
pub fn setup_main_and_participant(config: &CoreTestConfig) {
    
    let mc = config.main_client;
    let pc = config.participant_client;
    let db = config.test_db_name;
    let contract = config.contract_desc;
    let pda = config.participant_db_addr;

    {
        let (tx, rx) = mpsc::channel();
        let db = db.to_owned();
        let pda = pda.clone();
    
        thread::spawn(move || {
            let res = main_service_client(&db, &mc, &pda, &contract);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let sent_participant_contract = rx.try_recv().unwrap();
        debug!("send_participant_contract: got: {sent_participant_contract}");

        assert!(sent_participant_contract);
    }

    {
        let (tx, rx) = mpsc::channel();
        let contract = contract.clone();

        thread::spawn(move || {
            let res = participant_service_client(&pc, &contract);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let participant_accepted_contract = rx.try_recv().unwrap();
        debug!("participant_accpeted_contract: got: {participant_accepted_contract}");

        assert!(participant_accepted_contract);
    }

    {
        let (tx, rx) = mpsc::channel();
        let db = db.clone();

        thread::spawn(move || {
            let res = main_execute_coop_write_and_read(&db, &mc);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let write_is_successful = rx.try_recv().unwrap();
        assert!(write_is_successful);
    }
}