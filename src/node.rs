use crate::error::Error;
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

    pub fn evaluate(&mut self) -> Result<Value, Error> {
        match &self.token {
            Some(token) => {
                if self.children.is_empty() {
                    // just this one node
                    match token.get_token_type() {
                        TokenType::Identifier(_identifier) => {
                            // will require variables and scopes
                            todo!()
                        }
                        TokenType::Integer(integer) => {
                            Ok(Value::Integer(*integer))
                        }
                        TokenType::Float(float) => {
                            Ok(Value::Float(*float))
                        }
                        TokenType::String(string) => {
                            Ok(Value::String(string.clone()))
                        }
                        TokenType::LBracket | TokenType::RBracket => {
                            unreachable!()
                        }
                    }
                }
                else {
                    // will require functions
                    todo!()
                }
            },
            None => {
                if self.children.is_empty() {
                    // like the rust () for null
                    Ok(Value::Null)
                } else {
                    for child in &self.children {
                        child.borrow_mut().evaluate()?;
                    }

                    Ok(Value::Null)
                }
            }
        }
    }
}
