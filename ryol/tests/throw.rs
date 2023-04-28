use ryol::prelude::*;

#[test]
fn throw_tests() {
    assert!(eval("(throw)").is_err());
    // todo: eventually test against format function
}
