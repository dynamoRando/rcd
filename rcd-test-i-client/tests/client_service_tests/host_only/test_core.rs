use log::debug;
use rcd_enum::remote_delete_behavior::RemoteDeleteBehavior;
use rcd_test_harness::{CoreTestConfig, RcdClientConfig};

pub fn test_core(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let response = client(&config.test_db_name, &mc);
    debug!("has table: got: {response}");

    assert!(response);
}

#[tokio::main]
async fn client(db_name: &str, config: &RcdClientConfig) -> bool {
    #[allow(unused_imports)]
    use log::Log;
    use rcd_enum::database_type::DatabaseType;
    use rcd_enum::logical_storage_policy::LogicalStoragePolicy;
    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    let mut client = rcd_test_harness::get_rcd_client(config).await;

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

    let logical_storage_policy = LogicalStoragePolicy::HostOnly;

    client
        .set_logical_storage_policy(db_name, "EMPLOYEE", logical_storage_policy)
        .await
        .unwrap();

    let behavior = RemoteDeleteBehavior::Ignore;

    client
        .generate_contract(db_name, "tester", "desc", behavior)
        .await
        .unwrap();

    client
        .execute_write_at_host(
            db_name,
            "INSERT INTO EMPLOYEE ( Id, Name ) VALUES ( 1234, 'Rando');",
            DatabaseType::to_u32(DatabaseType::Sqlite),
            "",
        )
        .await
        .unwrap();

    return client.has_table(db_name, "EMPLOYEE").await.unwrap();
}
