use foo::foobar;

mod common;
use common::setup_foo;


#[test]
fn foo_foo_test() {
    setup_foo();

    foobar();
}