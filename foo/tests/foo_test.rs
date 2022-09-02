use foo::foobar;
use common::setup_foo;


#[test]
fn foo_test() {
    setup_foo();

    foobar();
}