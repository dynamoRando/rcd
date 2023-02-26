use rcd_test_harness::{
    test_common::multi::common_contract_setup::main_and_participant_setup, CoreTestConfig,
};

pub fn test_core(config: CoreTestConfig) {
    go(config)
}

#[tokio::main]
async fn go(config: CoreTestConfig) {
    let mc = config.main_client.clone();
    // let pc = config.participant_client.as_ref().unwrap().clone();
    let db_name = config.test_db_name.clone();
    let result = main_and_participant_setup(config.clone()).await;
    assert!(result);

    let mut mc = rcd_test_harness::get_rcd_client(&mc).await;

    let mut main_has_participant: bool = false;

    let data = mc.get_participants_for_database(&db_name).await.unwrap();

    for participant in &data.participants {
        if participant.participant.as_ref().unwrap().alias == "participant" {
            main_has_participant = true;
        }
    }

    assert!(main_has_participant);
}
