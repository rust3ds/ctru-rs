#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run_gdb)]

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[should_panic]
fn it_panics() {
    assert_eq!(2 + 2, 5);
}
