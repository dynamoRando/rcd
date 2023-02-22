
#[test]
fn get_harness_value() {
    let current = crate::test_harness::TEST_SETTINGS
        .lock()
        .unwrap()
        .get_current_port();
    let next = crate::test_harness::TEST_SETTINGS
        .lock()
        .unwrap()
        .get_next_avail_port();
    assert_eq!(current + 1, next);
}


