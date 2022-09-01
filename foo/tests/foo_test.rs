use crate::common::setup_foo;
use foo::foobar;

pub mod common;

#[test]
fn foo_test() {
    setup_foo();

    foobar();
}