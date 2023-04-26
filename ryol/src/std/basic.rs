use crate::error::Error;
use crate::node::Node;
use crate::run_state::RunState;
use crate::tokeniser::TokenType;
use crate::value::Value;
use crate::variable::Variable;

pub fn add_basic_lib(run_state: &mut RunState) {
    let scope = run_state.get_global_scope_mut();

    // variables
    scope.set_local("set", Variable::new(Value::NativeMacro(std_basic_set)));
}

fn get_identifier(node: &Node) -> Result<&String, Error> {
    match node.get_token() {
        Some(token) => match token.get_token_type() {
            TokenType::Identifier(identifier) => Ok(identifier),
            _ => Err(Error::new(
                "must be an identifier token type".to_string(),
                Some(token.clone()),
            )),
        },
        None => Err(Error::new("node must have a token".to_string(), None)),
    }
}

fn std_basic_set(run_state: &mut RunState, node: &Node) -> Result<Value, Error> {
    let args = node.get_children();

    // can probably get rid of this
    if args.len() < 2 {
        return Err(Error::new(
            "requires at least two arguments".to_string(),
            None,
        ));
    }

    // get every pair of itentifier/value pairs
    for i in (0..args.len()).filter(|i| i % 2 == 0) {
        // should always exist because of the iterator
        let identifier = get_identifier(args.get(i).unwrap())?;

        match args.get(i + 1) {
            Some(value_node) => {
                let value = value_node.evaluate(run_state)?;

                // can not put outside the loop due to multiple borrows
                let scope = run_state.get_local_scope_mut();

                // update variable if already exists, otherwise create
                // todo?: could move this into set_local
                match scope.get_local_mut(identifier) {
                    Some(variable) => {
                        if variable.is_const() {
                            let token = value_node.get_token().clone();

                            return Err(Error::new(
                                format!("variable: \"{}\" is const", identifier),
                                token,
                            ));
                        }

                        variable.set(value);
                    }
                    None => {
                        scope.set_local(identifier.as_str(), Variable::new(value));
                    }
                }
            }
            None => {
                return Err(Error::new(
                    format!(
                        "identifier: \"{}\" requires corresponding value",
                        identifier
                    ),
                    node.get_token().clone(),
                ));
            }
        }
    }

    Ok(Value::default())
}

#[cfg(test)]
mod tests {
    use crate::eval;
    use crate::value::Value;
    use crate::run_state::RunState;

    #[test]
    fn variable_set_tests() {
        // syntax
        assert!(eval("(set)", None).is_err());
        assert!(eval("(set x)", None).is_err());
        assert!(eval("(set x 1 y)", None).is_err());
        assert!(eval("(set x 1)", None).is_ok());
        assert!(eval("set x 1 y 2", None).is_ok());
        assert!(eval("(set x 1 y 2)", None).is_ok());

        let mut run_state = RunState::new();
        let identifier = "x".to_string();
        let value = Value::Integer(5);

        // should return null when setting value
        assert_eq!(eval("set x 5", Some(&mut run_state)).unwrap(), Value::Null);

        // should exist
        assert!(run_state.get_local_scope_mut().local_exists(&identifier));

        // should have the correct value
        assert_eq!(
            *run_state
                .get_local_scope_mut()
                .get_local(&identifier)
                .unwrap()
                .get(),
            value
        );

        assert_eq!(
            eval("(set x 5) (+ x x)", Some(&mut run_state)).unwrap(),
            Value::Integer(5 + 5)
        );
    }
}
