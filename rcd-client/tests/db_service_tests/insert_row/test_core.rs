use crate::test_harness::ServiceAddr;
use log::debug;
use rcd_client::RcdClient;
use std::sync::{mpsc, Arc};
use std::thread;

pub struct CoreTestConfig<'a> {
    pub main_client: &'a RcdClient,
    pub participant_client: &'a RcdClient,
    pub test_db_name: &'a str,
    pub contract_desc: &'a str,
    pub participant_db_addr: &'a ServiceAddr,
}

pub fn test_core(config: &CoreTestConfig) {
    let mc = Arc::new(config.main_client.clone());
    let pc = Arc::new(config.participant_client.clone());
    let db = Arc::new(config.test_db_name.clone());
    let contract = Arc::new(config.contract_desc.clone());
    let pda = Arc::new(config.participant_db_addr.clone());

    {
        let (tx, rx) = mpsc::channel();
        let db = db.clone();
        let contract = contract.clone();

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
        let mc = mc.clone();

        thread::spawn(move || {
            let res = main_execute_coop_write(&db, &mc);
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
async fn main_service_client(
    db_name: &str,
    main_client: &RcdClient,
    participant_db_addr: &ServiceAddr,
    contract_desc: &str,
) -> bool {
    use rcd_enum::database_type::DatabaseType;
    use rcd_enum::logical_storage_policy::LogicalStoragePolicy;
    use rcd_enum::remote_delete_behavior::RemoteDeleteBehavior;

    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    main_client.create_user_database(db_name).await.unwrap();
    main_client
        .enable_cooperative_features(db_name)
        .await
        .unwrap();
    main_client
        .execute_write_at_host(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type, "")
        .await
        .unwrap();

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    main_client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    let logical_storage_policy = LogicalStoragePolicy::ParticpantOwned;

    main_client
        .set_logical_storage_policy(db_name, "EMPLOYEE", logical_storage_policy)
        .await
        .unwrap();

    let behavior = RemoteDeleteBehavior::Ignore;

    main_client
        .generate_contract(db_name, "tester", &contract_desc, behavior)
        .await
        .unwrap();

    main_client
        .add_participant(
            db_name,
            "participant",
            &participant_db_addr.ip4_addr,
            participant_db_addr.port,
            "".to_string(),
            0,
        )
        .await
        .unwrap();

    return main_client
        .send_participant_contract(db_name, "participant")
        .await
        .unwrap();
}

#[cfg(test)]
#[tokio::main]

async fn main_execute_coop_write(db_name: &str, main_client: &RcdClient) -> bool {
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

#[cfg(test)]
#[tokio::main]
async fn participant_service_client(participant_client: &RcdClient, contract_desc: &str) -> bool {
    let mut has_contract = false;

    participant_client
        .generate_host_info("participant")
        .await
        .unwrap();

    let pending_contracts = participant_client.view_pending_contracts().await.unwrap();

    for contract in &pending_contracts {
        if contract.description == contract_desc {
            has_contract = true;
            break;
        }
    }

    let mut accepted_contract = false;

    if has_contract {
        accepted_contract = participant_client
            .accept_pending_contract("tester")
            .await
            .unwrap();
    }

    accepted_contract
}
