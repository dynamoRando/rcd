use crate::test_common::multi::common_actions::{main_service_client, participant_service_client};
use crate::test_harness::CoreTestConfig;
use log::debug;
use rcd_client::RcdClient;
use std::sync::{mpsc, Arc};
use std::thread;

pub fn test_core(config: CoreTestConfig) {
    let mc = Arc::new(config.main_client.clone());
    let pc = Arc::new(config.participant_client.clone());
    let db = Arc::new(config.test_db_name.clone());
    let contract = Arc::new(config.contract_desc.clone());
    let pda = Arc::new(config.participant_db_addr.clone());

    {
        let (tx, rx) = mpsc::channel();
        let db = db.clone();
        let contract = contract.clone();
        let mc = mc.clone();

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
        let pc = pc.clone();
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
        let mut mc = (*mc).clone();

        thread::spawn(move || {
            let res = main_execute_coop_write(&db, &mut mc);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let write_is_successful = rx.try_recv().unwrap();
        assert!(write_is_successful);
    }
}

#[cfg(test)]
#[tokio::main]

async fn main_execute_coop_write(db_name: &str, main_client: &mut RcdClient) -> bool {
    return main_client
        .execute_cooperative_write_at_host(
            db_name,
            "INSERT INTO EMPLOYEE ( Id, Name ) VALUES ( 999, 'ASDF');",
            "participant",
            "",
        )
        .await
        .unwrap();
}
