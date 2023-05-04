use ryol::prelude::*;

#[test]
fn format_basic_tests() {
    assert_eq!(
        eval("(format ())").unwrap(),
        Value::String("()".to_string())
    );
    assert_eq!(
        eval("(format true)").unwrap(),
        Value::String("true".to_string())
    );
    assert_eq!(
        eval("(format false)").unwrap(),
        Value::String("false".to_string())
    );
    assert_eq!(eval("(format 1)").unwrap(), Value::String("1".to_string()));
    assert_eq!(eval("(format 2)").unwrap(), Value::String("2".to_string()));
    assert_eq!(
        eval("(format 1.5)").unwrap(),
        Value::String("1.5".to_string())
    );
    assert_eq!(
        eval("(format 1.0)").unwrap(),
        Value::String("1.0".to_string())
    );
    assert_eq!(
        eval("(format 2.0)").unwrap(),
        Value::String("2.0".to_string())
    );
    assert_eq!(
        eval("(format \"asdf\")").unwrap(),
        Value::String("asdf".to_string())
    );
    assert_eq!(
        eval("(format \"fdsa\")").unwrap(),
        Value::String("fdsa".to_string())
    );
    assert_eq!(
        eval("(format (list 1 2 3)").unwrap(),
        Value::String("(list 1 2 3)".to_string())
    );

    // todo: check function ptrs are printing properly
}

#[test]
fn format_combined_tests() {
    assert_eq!(
        eval("(format 1 2)").unwrap(),
        Value::String("12".to_string())
    );
    assert_eq!(
        eval("(format 1 \" \" 2)").unwrap(),
        Value::String("1 2".to_string())
    );
    assert_eq!(
        eval("(const msg \"asdf\") (format msg)").unwrap(),
        Value::String("asdf".to_string())
    );
}
