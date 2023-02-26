use rcd_test_harness::{CoreTestConfig, RcdClientConfig};

pub fn test_core(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let response = client(&config.test_db_name, &mc);
    assert!(response);
}

#[cfg(test)]
#[tokio::main]
async fn client(db_name: &str, config: &RcdClientConfig) -> bool {
    use rcd_enum::database_type::DatabaseType;
    use rcd_enum::logical_storage_policy::LogicalStoragePolicy;
    use rcd_enum::remote_delete_behavior::RemoteDeleteBehavior;

    let database_type = DatabaseType::to_u32(DatabaseType::Sqlite);

    let mut client = rcd_test_harness::get_rcd_client(config).await;

    client.create_user_database(db_name).await.unwrap();
    client.enable_cooperative_features(db_name).await.unwrap();
    client
        .execute_write_at_host(db_name, "DROP TABLE IF EXISTS EMPLOYEE;", database_type, "")
        .await
        .unwrap();

    let create_table_statement =
        String::from("CREATE TABLE IF NOT EXISTS EMPLOYEE (Id INT, Name TEXT);");

    client
        .execute_write_at_host(db_name, &create_table_statement, database_type, "")
        .await
        .unwrap();

    let logical_storage_policy = LogicalStoragePolicy::ParticpantOwned;

    client
        .set_logical_storage_policy(db_name, "EMPLOYEE", logical_storage_policy)
        .await
        .unwrap();

    let behavior = RemoteDeleteBehavior::Ignore;

    let _generate_result = client
        .generate_contract(db_name, "tester", "desc", behavior)
        .await
        .unwrap();

    let contract = client.get_active_contract(db_name).await.unwrap();
    let cd = contract.contract.unwrap();
    let schema = cd.schema.unwrap();

    schema.database_name == db_name
}
