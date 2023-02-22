use log::debug;
use rcd_client::RcdClient;
use rcd_enum::database_type::DatabaseType;
use rcd_test_harness::CoreTestConfig;
use std::sync::{mpsc, Arc};
use std::thread;

pub fn test_core(config: CoreTestConfig) {
    let mc = Arc::new(config.main_client.clone());
    
    {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let res = client(&config.test_db_name, &mc);
            tx.send(res).unwrap();
        })
        .join()
        .unwrap();

        let response = rx.try_recv().unwrap();

        debug!("create_db_enable_coop_read_write: got: is_error: {response}");

        assert!(!response);
    }
}

#[cfg(test)]
#[tokio::main]
async fn client(db_name: &str, main_client: &RcdClient) -> bool {
    let mut client = (*main_client).clone();
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    let is_db_created = client.create_user_database(db_name).await.unwrap();

    assert!(is_db_created);

    let enable_coop_features = client.enable_cooperative_features(db_name).await.unwrap();
    let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

    assert!(enable_coop_features);
    let mut execute_write_drop_is_successful = false;
    execute_write_drop_is_successful = client
        .execute_write_at_host(db_name, &drop_table_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_drop_is_successful);

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    let mut execute_write_create_reply_is_successful = false;
    execute_write_create_reply_is_successful = client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_create_reply_is_successful);

    let add_record_statement = String::from("INSERT INTO EMPLOYEE (Id, Name) VALUES (1, 'Randy');");

    let mut execute_write_add_record_is_successful = false;
    execute_write_add_record_is_successful = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_add_record_is_successful);

    let read_record_statement = String::from("SELECT Id, Name FROM EMPLOYEE");
    let read_reply = client
        .execute_read_at_host(db_name, &read_record_statement, database_type)
        .await
        .unwrap();

    read_reply.is_error
}
