use foo::foobar;

mod common;
use common::setup_foo;


#[test]
fn other_test() {
    setup_foo();

    foobar();
}