use tracing::{debug, trace, warn};
use rcd_test_harness::{
    test_common::multi::common_contract_setup::main_and_participant_setup, CoreTestConfig,
};

pub fn test_core(config: CoreTestConfig) {
    go(config)
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    let pc = config.participant_client.as_ref().unwrap().clone();
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);

    let mut pc = rcd_test_harness::get_rcd_client(&pc).await;
    let mut mc = rcd_test_harness::get_rcd_client(&mc).await;

    // create main dbs
    {
        mc.create_user_database("get_db_names2.db").await.unwrap();
        mc.create_user_database("get_db_names3.db").await.unwrap();
    }

    // create part dbs
    {
        pc.create_user_database("part_example.db").await.unwrap();
        pc.create_user_database("part_example2.db").await.unwrap();
        pc.create_user_database("part_example3.db").await.unwrap();
    }

    // participant should have all databases
    {
        let mut has_all_databases = true;

        let result = pc.get_databases().await;

        let dbs_reply = result.unwrap();

        let mut actual_db_names: Vec<String> = Vec::new();

        trace!("actual names");

        for db in &dbs_reply.databases {
            trace!("{}", db.database_name.clone());
            actual_db_names.push(db.database_name.clone());
        }

        let expected_db_names: [&str; 5];

        match config
            .participant_client
            .as_ref()
            .unwrap()
            .clone()
            .client_type
        {
            rcd_client::client_type::RcdClientType::Grpc => {
                if config
                    .participant_client
                    .as_ref()
                    .unwrap()
                    .host_id
                    .is_none()
                {
                    expected_db_names = [
                        "part_example.db",
                        "part_example2.db",
                        "part_example3.db",
                        "get_db_names_grpc.dbpart",
                        "rcd.db",
                    ];
                } else {
                    expected_db_names = [
                        "get_db_names_grpc-proxy.dbpart",
                        "part_example3.db",
                        "part_example2.db",
                        "part_example.db",
                        "rcd.db",
                    ];
                }
            }
            rcd_client::client_type::RcdClientType::Http => {
                expected_db_names = [
                    "part_example.db",
                    "part_example2.db",
                    "part_example3.db",
                    "get_db_names_http.dbpart",
                    "rcd.db",
                ];
            }
        }

        trace!("expected names");
        for name in &expected_db_names {
            trace!("{name}");
        }

        debug!("actual: {:?}", actual_db_names);
        debug!("expected: {:?}", expected_db_names);

        for name in &expected_db_names {
            if !actual_db_names.iter().any(|n| n == name) {
                warn!("missing database: {:?}", name);
                has_all_databases = false;
            }
        }

        assert!(has_all_databases);
    }

    // main should have all databases
    {
        let mut has_all_databases = true;
        let result = mc.get_databases().await;

        let dbs_reply = result.unwrap();

        let mut actual_db_names: Vec<String> = Vec::new();

        trace!("actual names");

        for db in &dbs_reply.databases {
            trace!("{}", db.database_name.clone());
            actual_db_names.push(db.database_name.clone());
        }

        let expected_db_names: [&str; 4];

        match config.main_client.client_type {
            rcd_client::client_type::RcdClientType::Grpc => {
                if config
                    .participant_client
                    .as_ref()
                    .unwrap()
                    .host_id
                    .is_none()
                {
                    expected_db_names = [
                        "get_db_names2.db",
                        "get_db_names3.db",
                        "get_db_names_grpc.db",
                        "rcd.db",
                    ];
                } else {
                    expected_db_names = [
                        "get_db_names2.db",
                        "get_db_names3.db",
                        "get_db_names_grpc-proxy.db",
                        "rcd.db",
                    ];
                }
            }
            rcd_client::client_type::RcdClientType::Http => {
                expected_db_names = [
                    "get_db_names2.db",
                    "get_db_names3.db",
                    "get_db_names_http.db",
                    "rcd.db",
                ];
            }
        }

        trace!("expected names");
        for name in &expected_db_names {
            trace!("{name}");
        }

        for name in &expected_db_names {
            if !actual_db_names.contains(&(*name).to_string()) {
                warn!("missing database: {:?}", name);
                has_all_databases = false;
            }
        }

        assert!(has_all_databases);
    }
}
