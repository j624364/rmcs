use crate::prelude::*;

pub fn add_basic_lib(run_state: &mut RunState) {
    let scope = run_state.get_global_scope_mut();

    // variables
    scope.set_local("set", Variable::new(Value::NativeMacro(std_basic_set)));
    scope.set_local("if", Variable::new(Value::NativeMacro(std_basic_if)));
    scope.set_local("times", Variable::new(Value::NativeMacro(std_basic_times)));
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
