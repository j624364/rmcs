use crate::error::Error;
use crate::run_state::RunState;
use crate::value::Value;
use crate::variable::Variable;

pub fn add_list_lib(run_state: &mut RunState) {
    let scope = run_state.get_global_scope_mut();

    scope.set_local("list", Variable::new(Value::NativeFunction(std_list_list)));
}

fn std_list_list(args: Vec<Value>) -> Result<Value, Error> {
    Ok(Value::List(args))
}

#[cfg(test)]
mod tests {
    use crate::eval;
    use crate::value::Value;

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
}
