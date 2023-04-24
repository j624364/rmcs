use crate::error::Error;
use crate::run_state::RunState;
use crate::tokeniser::{Token, TokenType};
use crate::value::Value;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node {
    token: Option<Token>,
    children: Vec<RefCell<Node>>,
}

impl Node {
    pub fn new(token: Option<Token>) -> Self {
        Node {
            token,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: RefCell<Node>) {
        self.children.push(child);
    }

    pub fn get_token(&self) -> &Option<Token> {
        &self.token
    }

    pub fn get_children(&self) -> &Vec<RefCell<Node>> {
        &self.children
    }

    pub fn evaluate(&mut self, run_state: &mut RunState) -> Result<Value, Error> {
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
                                _ => {
                                    match run_state.find_local(identifier) {
                                        Some(local) => Ok(local.get().clone()),
                                        None => {
                                            // generate an error
                                            return Err(Error::new(
                                                format!(
                                                    "could not find identifier: \"{}\"",
                                                    identifier
                                                ),
                                                self.token.clone(),
                                            ));
                                        }
                                    }
                                }
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
                    // will require functions
                    todo!()
                }
            }
            None => {
                if self.children.is_empty() {
                    // like the rust () for null
                    Ok(Value::Null)
                } else if self.children.len() == 1 {
                    self.children
                        .first()
                        .unwrap()
                        .borrow_mut()
                        .evaluate(run_state)
                } else {
                    let mut last_value = Value::default();

                    for child in &self.children {
                        last_value = child.borrow_mut().evaluate(run_state)?;
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
    use crate::parser;
    use crate::tokeniser;
    use crate::variable::Variable;

    fn eval(source: &str, run_state_option: Option<RunState>) -> Value {
        let mut run_state = run_state_option.unwrap_or(RunState::new());

        parser::parse(tokeniser::tokenise(source).unwrap())
            .unwrap()
            .borrow_mut()
            .evaluate(&mut run_state)
            .unwrap()
    }

    #[test]
    fn basic_literals_tests() {
        assert_eq!(eval("true", None), Value::Boolean(true));
        assert_eq!(eval("false", None), Value::Boolean(false));
        assert_eq!(eval("1", None), Value::Integer(1));
        assert_eq!(eval("1.5", None), Value::Float(1.5));
        assert_eq!(eval("\"asdf\"", None), Value::String("asdf".to_string()));
        // todo: make work later
        // assert_eq!(eval("()", None), Value::Null);
    }

    #[test]
    fn global_variable_test() {
        let mut run_state = RunState::new();
        let scope = run_state.get_global_scope_mut();
        let value = Value::Integer(5);

        scope.set_local("x", Variable::new(value.clone()));
        assert_eq!(eval("x", Some(run_state.clone())), value.clone());
        assert_eq!(eval("(x)", Some(run_state.clone())), value);
        assert_eq!(eval("((x))", Some(run_state)), value);
    }
}
