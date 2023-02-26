use rcd_test_harness::{CoreTestConfig, RcdClientConfig};

/*

Because of the way logging works (which must initalize a logger for lifetime of STATIC) the 
logging tests are the only tests that are broken into their own crate for HTTP/GRPC. In other words,
we MUST treat them as seperate programs.

The actual common test code is stored in this file.

Also, because you can't dashes in module names, these are the only files that don't follow the test naming pattern.

*/

pub fn test_core(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let db = config.test_db_name.clone();
    let got_logs = client(&db, &mc);
    assert!(got_logs);
}

#[cfg(test)]
#[tokio::main]
async fn client(db_name: &str, config: &RcdClientConfig) -> bool {
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

    let behavior = rcd_enum::remote_delete_behavior::RemoteDeleteBehavior::Ignore;

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

    client.has_table(db_name, "EMPLOYEE").await.unwrap();

    let logs = client.get_last_log_entries(5).await.unwrap().logs;
    !logs.is_empty()
}
