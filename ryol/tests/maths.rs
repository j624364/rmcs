use ryol::prelude::*;

#[test]
fn addition_tests() {
    // todo: add tests for floats

    assert_eq!(eval("(+ 1 1)").unwrap(), Value::Integer(1 + 1));
    assert_eq!(eval("(+ 1 2)").unwrap(), Value::Integer(1 + 2));
    assert_eq!(eval("(+ 1 2 3)").unwrap(), Value::Integer(1 + 2 + 3));
    assert_eq!(eval("(+ 1 (+ 2 3))").unwrap(), Value::Integer(1 + (2 + 3)));
}

#[test]
fn subtraction_tests() {
    assert_eq!(eval("(- 3 2)").unwrap(), Value::Integer(3 - 2));
    assert_eq!(eval("(- 2 3)").unwrap(), Value::Integer(2 - 3));
    assert_eq!(eval("(- 1 2 3)").unwrap(), Value::Integer(1 - 2 - 3));
}

#[test]
fn multiplication_tests() {
    assert_eq!(eval("(* 1 1)").unwrap(), Value::Integer(1 * 1));
    assert_eq!(eval("(* 1 2)").unwrap(), Value::Integer(1 * 2));
    assert_eq!(eval("(* 1 2 3)").unwrap(), Value::Integer(1 * 2 * 3));
    assert_eq!(eval("(* 100 100)").unwrap(), Value::Integer(100 * 100));
}

#[test]
fn division_tests() {
    // usually would have issues with floating point inaccuracy but as the same
    // operations should be carried out here and in the code, it should all be good
    assert_eq!(eval("(/ 3 2)").unwrap(), Value::Integer(3 / 2));
    assert_eq!(eval("(/ 2 3)").unwrap(), Value::Integer(2 / 3));
    assert_eq!(eval("(/ 1 2 3)").unwrap(), Value::Integer(1 / 2 / 3));
}
