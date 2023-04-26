use crate::error::Error;
use crate::run_state::RunState;
use crate::tokeniser::{Token, TokenType};
use crate::value::Value;

#[derive(Debug)]
pub struct Node {
    token: Option<Token>,
    children: Vec<Node>,
}

impl Node {
    pub fn new(token: Option<Token>) -> Self {
        Node {
            token,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn get_token(&self) -> &Option<Token> {
        &self.token
    }

    pub fn get_children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn evaluate(&self, run_state: &mut RunState) -> Result<Value, Error> {
        match &self.token {
            Some(token) => {
                if self.children.is_empty() {
                    // just this one node
                    match token.get_token_type() {
                        TokenType::Identifier(identifier) => {
                            match identifier.as_str() {
                                // should probably change how i do this
                                "true" => Ok(Value::Boolean(true)),
                                "false" => Ok(Value::Boolean(false)),
                                _ => match run_state.find_local(identifier) {
                                    Some(local) => match local.get() {
                                        Value::NativeFunction(func) => match func(Vec::new()) {
                                            Ok(res) => Ok(res),
                                            Err(mut error) => {
                                                error.set_token(token.clone());
                                                Err(error)
                                            }
                                        },
                                        Value::NativeMacro(func) => func(run_state, self),
                                        _ => Ok(local.get().clone()),
                                    },
                                    None => Err(Error::new(
                                        format!("could not find identifier: \"{}\"", identifier),
                                        self.token.clone(),
                                    )),
                                },
                            }
                        }
                        TokenType::Integer(integer) => Ok(Value::Integer(*integer)),
                        TokenType::Float(float) => Ok(Value::Float(*float)),
                        TokenType::String(string) => Ok(Value::String(string.clone())),
                        TokenType::LBracket | TokenType::RBracket => {
                            unreachable!()
                        }
                    }
                } else {
                    match token.get_token_type() {
                        TokenType::Identifier(identifier) => {
                            match run_state.find_local(identifier) {
                                Some(local) => match local.get() {
                                    Value::NativeFunction(func) => {
                                        let mut args = Vec::with_capacity(self.children.len());
                                        for child in &self.children {
                                            args.push(child.evaluate(run_state)?);
                                        }

                                        match func(args) {
                                            Ok(res) => Ok(res),
                                            Err(mut error) => {
                                                error.set_token(token.clone());
                                                Err(error)
                                            }
                                        }
                                    }
                                    Value::NativeMacro(func) => func(run_state, self),
                                    _ => Err(Error::new(
                                        "must be a function or macro".to_string(),
                                        self.token.clone(),
                                    )),
                                },
                                None => Err(Error::new(
                                    format!("could not find identifier: \"{}\"", identifier),
                                    self.token.clone(),
                                )),
                            }
                        }
                        _ => Err(Error::new(
                            "must be a function or macro".to_string(),
                            self.token.clone(),
                        )),
                    }
                }
            }
            None => {
                if self.children.is_empty() {
                    // like the rust () for null
                    Ok(Value::Null)
                } else if self.children.len() == 1 {
                    self.children.first().unwrap().evaluate(run_state)
                } else {
                    let mut last_value = Value::default();

                    for child in &self.children {
                        last_value = child.evaluate(run_state)?;
                    }

                    Ok(last_value)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval;
    use crate::variable::Variable;

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
        let scope = run_state.get_global_scope_mut();
        let value = Value::Integer(5);

        scope.set_local("x", Variable::new(value.clone()));
        assert_eq!(run_state.eval("x").unwrap(), value.clone());
        assert_eq!(run_state.eval("(x)").unwrap(), value);
        assert_eq!(run_state.eval("((x))").unwrap(), value);
    }
}
