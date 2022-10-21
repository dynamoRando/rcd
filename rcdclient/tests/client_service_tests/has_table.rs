use log::info;
extern crate futures;
extern crate tokio;
use crate::test_harness;
use std::sync::mpsc;
use std::{thread, time};

#[test]
pub fn test() {
    let test_name = "has_table";
    let test_db_name = format!("{}{}", test_name, ".db");
    let (tx, rx) = mpsc::channel();
    let port_num = test_harness::TEST_SETTINGS
        .lock()
        .unwrap()
        .get_next_avail_port();

    let root_dir = super::test_harness::get_test_temp_dir(test_name);
    println!("{}", root_dir);
    let mut service = rcdx::get_service_from_config_file();
    let client_address_port = format!("{}{}", String::from("[::1]:"), port_num.to_string());
    let target_client_address_port = client_address_port.clone();
    println!("{:?}", &service);

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
        let res = client(&test_db_name, &target_client_address_port);
        tx.send(res).unwrap();
    })
    .join()
    .unwrap();

    let response = rx.try_recv().unwrap();

    println!("has table: got: {}", response);

    assert!(response);

    test_harness::release_port(port_num);
}

#[cfg(test)]
#[tokio::main]
async fn client(db_name: &str, addr_port: &str) -> bool {
    #[allow(unused_imports)]
    use log::Log;
    use rcdclient::RcdClient;
    use rcd_core::rcd_enum::DatabaseType;
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    let addr_port = format!("{}{}", String::from("http://"), addr_port);
    info!("has_table attempting to connect {}", addr_port);

    let client = RcdClient::new(addr_port, String::from("tester"), String::from("123456"));

    client.create_user_database(db_name).await.unwrap();
    client.enable_cooperative_features(db_name).await.unwrap();

    let drop_table_statement = String::from("DROP TABLE IF EXISTS EMPLOYEE;");

    client
        .execute_write_at_host(db_name, &drop_table_statement, database_type, "")
        .await
        .unwrap();

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    return client.has_table(db_name, "EMPLOYEE").await.unwrap();
}
