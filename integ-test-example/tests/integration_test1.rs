use integ_test_example;
#[test]
fn files_test1() {
    assert_ne!(integ_test_example::get_process_id(), 0, "Invalid process id");
}

#[test]
fn files_test2() {
    assert_eq!(1+1, 2);
}

#[test]
fn process_test1() {
    assert!(true);
}