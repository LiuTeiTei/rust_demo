use automated_test;

mod common;

#[test]
fn it_add_two() {
    common::set_up();
    assert_eq!(4, automated_test::add_two(2));
}
