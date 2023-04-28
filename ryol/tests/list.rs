use ryol::prelude::*;

#[test]
fn variable_list_tests() {
    assert_eq!(eval("(list)").unwrap(), Value::List(Vec::new()));

    assert_eq!(
        eval("(list 5)").unwrap(),
        Value::List(vec![Value::Integer(5)])
    );

    assert_eq!(
        eval("(list 5 1)").unwrap(),
        Value::List(vec![Value::Integer(5), Value::Integer(1)])
    );

    assert_eq!(
        eval("(list 5 \"asdf\")").unwrap(),
        Value::List(vec![Value::Integer(5), Value::String("asdf".to_string())])
    );
}
