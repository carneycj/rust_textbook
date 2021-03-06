// It's probably a good idea to use 'extern crate' instead of 'use' to really
// drive home the fact that it's external.  Something I saw on MIT site, not
// Rustlang book.
extern crate testing;

// Integration tests are only included when running 'cargo test' when all unit
// tests pass.  Otherwise, you will have to individually call each file using
// 'cargo test --test <file name>'
#[test]
fn it_adds_two_integration() {
    assert_eq!(testing::add_two(2), 4);
    //println!("test");
    //assert_eq!(4, 3);
}

#[test]
fn integration_test() {
    assert_eq!(testing::prints_and_returns_10(5), 10);
}

// This is how we can access everything in 'common' that would be used to help
// with integration tests.
mod common;

#[test]
fn using_common() {
    common::setup();
    assert_eq!(4, 4);
}
