use log::info;
use rcd_core::rcd_enum::LogicalStoragePolicy;
extern crate futures;
extern crate tokio;
use crate::test_harness;
use std::sync::mpsc;
use std::{thread, time};

#[test]
pub fn test() {
    let test_name = "get_set_logical_storage_policy";
    let test_db_name = format!("{}{}", test_name, ".db");
    let (tx, rx) = mpsc::channel();
    let port_num = test_harness::TEST_SETTINGS
        .lock()
        .unwrap()
        .get_next_avail_port();

    let root_dir = super::test_harness::get_test_temp_dir(&test_name);
    println!("{}", root_dir);
    let mut service = rcdx::get_service_from_config_file();
    let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
    let target_client_address_port = client_address_port.clone();
    println!("{:?}", &service);
    let policy = LogicalStoragePolicy::ParticpantOwned;
    let i_policy = LogicalStoragePolicy::to_u32(policy);

    service.start_at_dir(&root_dir);

    info!("starting client at {}", &client_address_port);
    info!("starting client service");

    thread::spawn(move || {
        let _service = service.start_client_service_at_addr(client_address_port, root_dir);
    });

    let time = time::Duration::from_secs(1);

    info!("sleeping for 1 seconds...");

    thread::sleep(time);

    thread::spawn(move || {
        let res = client(&test_db_name, &target_client_address_port, i_policy);
        tx.send(res).unwrap();
    })
    .join()
    .unwrap();

    let response = rx.try_recv().unwrap();

    println!(
        "get_set_logical_storage_policy: got: policy_num: {}",
        response
    );

    assert_eq!(i_policy, response);

    test_harness::release_port(port_num);
}

#[cfg(test)]
#[tokio::main]
async fn client(db_name: &str, addr_port: &str, policy_num: u32) -> u32 {
    #[allow(unused_imports)]
    use log::Log;
    use rcdclient::RcdClient;
    use rcd_core::rcd_enum::DatabaseType;
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    let addr_port = format!("{}{}", String::from("http://"), addr_port);
    info!(
        "create_db_enable_coop_read_write attempting to connect {}",
        addr_port
    );

    let client = RcdClient::new(addr_port, String::from("tester"), String::from("123456"));

    let create_db_is_successful = client.create_user_database(db_name).await.unwrap();

    assert!(create_db_is_successful);

    let enable_coop_features_is_successful =
        client.enable_cooperative_features(db_name).await.unwrap();

    let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

    assert!(enable_coop_features_is_successful);

    let drop_table_is_successful = client
        .execute_write_at_host(db_name, &drop_table_statement, database_type, "")
        .await
        .unwrap();

    assert!(drop_table_is_successful);

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    let create_table_is_successful = client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    assert!(create_table_is_successful);

    let add_record_statement = String::from("INSERT INTO EMPLOYEE (Id, Name) VALUES (1, 'Randy');");

    let execute_write_is_successful = client
        .execute_write_at_host(db_name, &add_record_statement, database_type, "")
        .await
        .unwrap();

    assert!(execute_write_is_successful);

    let read_record_statement = String::from("SELECT Id, Name FROM EMPLOYEE");

    let result = client
        .execute_read_at_host(db_name, &read_record_statement, database_type)
        .await
        .unwrap();

    assert!(!result.is_error);

    let _set_policy_is_successful = client
        .set_logical_storage_policy(
            db_name,
            "EMPLOYEE",
            LogicalStoragePolicy::from_i64(policy_num as i64),
        )
        .await
        .unwrap();

    let policy_response = client
        .get_logical_storage_policy(db_name, "EMPLOYEE")
        .await
        .unwrap();

    let i_res_policy = LogicalStoragePolicy::to_u32(policy_response);
    return i_res_policy;
}
