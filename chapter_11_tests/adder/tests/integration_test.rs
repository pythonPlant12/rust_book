use adder::add_two;
mod common;
// To run only integration tests you should do:
// "cargo test --test integration_test"
// NOTE: Very important, you can use a integration tests only for library crates and not binary
// crate (like main.rs)
#[test]
fn it_adds_two() {

    common::say_hello();
    let result = add_two(2);
    assert_eq!(result, 4);
}
// Sometimes you want to have multiple files testing or for example a common file with common
// function to then use them in different tests, if you create a file common.rs it will treat is a
// a test file. Instead, you should create it in /tests/common/mod.rs
