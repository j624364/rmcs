use crate::prelude::*;

pub fn add_basic_lib(run_state: &mut RunState) -> Result<(), Error> {
    run_state.expose_macro("set", std_basic_set)?;
    run_state.expose_macro("if", std_basic_if)?;
    run_state.expose_macro("times", std_basic_times)?;

    Ok(())
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

                match scope.set_local(identifier, value) {
                    Ok(()) => {
                    }
                    Err(mut error) => {
                        match node.get_token().clone() {
                            Some(token) => {
                                error.set_token(token);
                                return Err(error);
                            }
                            None => {}
                        }
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

fn std_basic_if(run_state: &mut RunState, node: &Node) -> Result<Value, Error> {
    let args = node.get_children();

    if args.len() < 2 {
        return Err(Error::new(
            "requires at least two arguments".to_string(),
            node.get_token().clone(),
        ));
    }

    #[derive(Debug)]
    enum IfMode {
        Normal,
        Condition,
        Body,
        Skip,
    }

    let mut if_mode = IfMode::Condition;
    for node in args.iter() {
        println!("{:?} : {:?}", if_mode, node.get_token());
        match if_mode {
            IfMode::Normal => {
                match get_identifier(node) {
                    Ok(identifier) => {
                        match identifier.as_str() {
                            "elseif" | "elif" => {
                                if_mode = IfMode::Condition;
                            }
                            "else" => {
                                if_mode = IfMode::Body;
                            }
                            _ => {
                                // must be an expression
                                return node.evaluate(run_state);
                            }
                        }
                    }
                    _ => {
                        // must be the body of the if block
                        return node.evaluate(run_state);
                    }
                }
            }
            IfMode::Condition => {
                if node.evaluate(run_state)? == Value::Boolean(true) {
                    if_mode = IfMode::Body;
                } else {
                    if_mode = IfMode::Skip;
                }
            }
            IfMode::Skip => {
                if_mode = IfMode::Normal;
            }
            IfMode::Body => {
                return node.evaluate(run_state);
            }
        }
    }

    Ok(Value::default())
}

fn std_basic_times(run_state: &mut RunState, node: &Node) -> Result<Value, Error> {
    let args = node.get_children();

    if args.len() != 2 {
        return Err(Error::new(
            "takes two arguments".to_string(),
            node.get_token().clone(),
        ));
    }

    let count_value = args.first().unwrap().evaluate(run_state)?;
    match count_value {
        Value::Integer(count) => {
            for _ in 0..count {
                args.last().unwrap().evaluate(run_state)?;
            }
        }
        _ => {
            return Err(Error::new(
                format!("count must be an integer, recieved: {:?}", count_value),
                node.get_token().clone(),
            ));
        }
    }

    Ok(Value::default())
}
