use ryol::prelude::*;

#[test]
fn basic_literals_tests() {
    assert_eq!(eval("true").unwrap(), Value::Boolean(true));
    assert_eq!(eval("false").unwrap(), Value::Boolean(false));
    assert_eq!(eval("1").unwrap(), Value::Integer(1));
    assert_eq!(eval("1.5").unwrap(), Value::Float(1.5));
    assert_eq!(eval("\"asdf\"").unwrap(), Value::String("asdf".to_string()));
    // todo: make work later
    // assert_eq!(eval("()", None), Value::Null);
}

#[test]
fn global_variable_test() {
    let mut run_state = RunState::new();
    let value = Value::Integer(5);

    run_state.expose("x", value.clone()).unwrap();
    assert_eq!(run_state.eval("x").unwrap(), value.clone());
    assert_eq!(run_state.eval("(x)").unwrap(), value);
    assert_eq!(run_state.eval("((x))").unwrap(), value);
}

#[test]
fn variable_set_tests() {
    // syntax
    assert!(eval("(set)").is_err());
    assert!(eval("(set x)").is_err());
    assert!(eval("(set x 1 y)").is_err());
    assert!(eval("(set x 1)").is_ok());
    assert!(eval("set x 1 y 2").is_ok());
    assert!(eval("(set x 1 y 2)").is_ok());

    let mut run_state = RunState::new();
    let identifier = "x".to_string();
    let value = Value::Integer(5);

    // should return null when setting value
    assert_eq!(run_state.eval("set x 5").unwrap(), Value::Null);

    // should exist
    assert!(run_state.get_local_scope_mut().local_exists(&identifier));

    // should have the correct value
    assert_eq!(
        *run_state
            .get_local_scope_mut()
            .get_local(&identifier)
            .unwrap(),
        value
    );

    assert_eq!(
        run_state.eval("(set x 5) (+ x x)").unwrap(),
        Value::Integer(5 + 5)
    );
}

#[test]
fn if_tests() {
    assert!(eval("(if)").is_err());
    assert!(eval("(if true)").is_err());
    assert!(eval("(if false)").is_err());
    assert!(eval("(if 5)").is_err());
    assert!(eval("(if 5 5)").is_err());

    assert_eq!(eval("(if false 5)").unwrap(), Value::Null);
    assert_eq!(eval("(if true 5)").unwrap(), Value::Integer(5));

    assert_eq!(eval("(if true 5 6)").unwrap(), Value::Integer(5));
    assert_eq!(eval("(if false 5 6)").unwrap(), Value::Integer(6));
    assert_eq!(eval("(if true 5 else 6)").unwrap(), Value::Integer(5));
    assert_eq!(eval("(if false 5 else 6)").unwrap(), Value::Integer(6));

    assert_eq!(
        eval("(if false 5 elif false 6 else 7)").unwrap(),
        Value::Integer(7)
    );
    assert_eq!(
        eval("(if false 5 elif true 6 else 7)").unwrap(),
        Value::Integer(6)
    );
    assert_eq!(
        eval("(if true 5 elif false 6 else 7)").unwrap(),
        Value::Integer(5)
    );
    assert_eq!(
        eval("(if true 5 elif true 6 else 7)").unwrap(),
        Value::Integer(5)
    );

    assert_eq!(
        eval("(if false 5 elif false 6 elif false 7 else 8)").unwrap(),
        Value::Integer(8)
    );
    assert_eq!(
        eval("(if false 5 elif false 6 elif true 7 else 8)").unwrap(),
        Value::Integer(7)
    );
    assert_eq!(
        eval("(if false 5 elif true 6 elif false 7 else 8)").unwrap(),
        Value::Integer(6)
    );
    assert_eq!(
        eval("(if false 5 elif true 6 elif true 7 else 8)").unwrap(),
        Value::Integer(6)
    );
    assert_eq!(
        eval("(if true 5 elif false 6 elif false 7 else 8)").unwrap(),
        Value::Integer(5)
    );
    assert_eq!(
        eval("(if true 5 elif false 6 elif true 7 else 8)").unwrap(),
        Value::Integer(5)
    );
    assert_eq!(
        eval("(if true 5 elif true 6 elif false 7 else 8)").unwrap(),
        Value::Integer(5)
    );
    assert_eq!(
        eval("(if true 5 elif true 6 elif true 7 else 8)").unwrap(),
        Value::Integer(5)
    );
}

#[test]
fn times_tests() {
    let source = r#"
        (set x 0)
        (times 5 (set x (+ x 1)))
        x
    "#;

    assert_eq!(eval(source).unwrap(), Value::Integer(5));
}
