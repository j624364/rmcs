use crate::error::Error;
use crate::function::NativeFunction;
use crate::run_state::RunState;
use crate::structure::StructureInstance;
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

    pub fn evaluate_native_function(
        &self,
        run_state: &mut RunState,
        native_function: NativeFunction,
        token: &Token,
    ) -> Result<Value, Error> {
        let mut args = Vec::with_capacity(self.children.len());
        for child in &self.children {
            args.push(child.evaluate(run_state)?);
        }

        match native_function(args) {
            Ok(res) => Ok(res),
            Err(mut error) => {
                error.set_token(token.clone());
                Err(error)
            }
        }
    }

    pub fn evaluate_branch_identifier(
        &self,
        run_state: &mut RunState,
        token: &Token,
        identifier: &String,
    ) -> Result<Value, Error> {
        if let Some(structure_template) = run_state.find_structure_template(identifier) {
            Ok(Value::Structure(StructureInstance::from_template(
                structure_template,
            )))
        } else if let Some(local) = run_state.find_local(identifier) {
            match local.clone() {
                Value::NativeFunction(func) => {
                    self.evaluate_native_function(run_state, func, token)
                }
                Value::NativeMacro(func) => func(run_state, self),
                _ => Err(Error::new(
                    "must be a function or macro".to_string(),
                    self.token.clone(),
                )),
            }
        } else {
            Err(Error::new(
                format!("could not find identifier: \"{}\"", identifier),
                self.token.clone(),
            ))
        }
    }

    pub fn evaluate_token_with_children(
        &self,
        run_state: &mut RunState,
        token: &Token,
    ) -> Result<Value, Error> {
        match token.get_token_type() {
            TokenType::Identifier(identifier) => {
                self.evaluate_branch_identifier(run_state, token, identifier)
            }
            _ => Err(Error::new(
                "must be a function or macro".to_string(),
                self.token.clone(),
            )),
        }
    }

    pub fn evaluate_leaf_identifer(
        &self,
        run_state: &mut RunState,
        token: &Token,
        identifier: &String,
    ) -> Result<Value, Error> {
        if let Some(structure_template) = run_state.find_structure_template(identifier) {
            Ok(Value::Structure(StructureInstance::from_template(
                structure_template,
            )))
        } else if let Some(local) = run_state.find_local(identifier) {
            match local {
                Value::NativeFunction(func) => {
                    self.evaluate_native_function(run_state, func, token)
                }
                Value::NativeMacro(func) => func(run_state, self),
                _ => Ok(local.clone()),
            }
        } else {
            Err(Error::new(
                format!("could not find identifier: \"{}\"", identifier),
                self.token.clone(),
            ))
        }
    }

    pub fn evaluate_leaf_node(
        &self,
        run_state: &mut RunState,
        token: &Token,
    ) -> Result<Value, Error> {
        // just this one node
        match token.get_token_type() {
            TokenType::Identifier(identifier) => {
                self.evaluate_leaf_identifer(run_state, token, identifier)
            }
            TokenType::Integer(integer) => Ok(Value::Integer(*integer)),
            TokenType::Float(float) => Ok(Value::Float(*float)),
            TokenType::String(string) => Ok(Value::String(string.clone())),
            TokenType::LBracket | TokenType::RBracket => {
                unreachable!()
            }
        }
    }

    pub fn evaluate(&self, run_state: &mut RunState) -> Result<Value, Error> {
        match (&self.token, !self.children.is_empty()) {
            (Some(token), true) => self.evaluate_token_with_children(run_state, token),
            (Some(token), false) => self.evaluate_leaf_node(run_state, token),
            (None, true) => {
                if self.children.len() == 1 {
                    self.children.first().unwrap().evaluate(run_state)
                } else {
                    let mut last_value = Value::default();

                    for child in &self.children {
                        last_value = child.evaluate(run_state)?;
                    }

                    Ok(last_value)
                }
            }
            (None, false) => {
                // like the rust () for null
                Ok(Value::Null)
            }
        }
    }
}
