use adder;
mod common;

#[test]
fn it_works() {
    common::setup();
    assert_eq!(4 , adder::add(2, 2));
}


//cargo test --test integration_test