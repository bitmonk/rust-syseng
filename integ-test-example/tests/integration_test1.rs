use integ_test_example;
#[test]
fn integration_test1() {
    assert_ne!(integ_test_example::get_process_id(), 0, "Invalid process id");
}
