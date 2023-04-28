use crate::error::Error;
use crate::function::{NativeFunction, NativeMacro};
use crate::parser;
use crate::std::add_std_lib;
use crate::tokeniser;
use crate::value::Value;
use crate::variable::Variable;
use crate::EvalError;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Scope {
    locals: HashMap<String, Variable>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
        }
    }

    pub fn local_exists(&self, identifier: &String) -> bool {
        self.locals.contains_key(identifier)
    }

    // todo: make into Option<&Variable>
    pub fn get_local(&self, identifier: &String) -> Option<&Value> {
        Some(self.locals.get(identifier)?.get())
    }

    pub fn get_local_mut(&mut self, identifier: &String) -> Option<&mut Variable> {
        self.locals.get_mut(identifier)
    }

    pub fn set_local(&mut self, identifier: &String, value: Value) -> Result<(), Error> {
        if let Some(local) = self.locals.get_mut(identifier) {
            if local.is_const() {
                Err(Error::new(
                    format!("variable: \"{}\" is const", identifier),
                    None,
                ))
            } else {
                local.set(value);
                Ok(())
            }
        } else {
            self.locals
                .insert(identifier.to_string(), Variable::new(value));
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct RunState {
    scopes: VecDeque<Scope>,
}

impl RunState {
    pub fn new_empty() -> Self {
        let mut output = Self {
            scopes: VecDeque::with_capacity(512),
        };

        // make sure there is one global scope
        output.scopes.push_back(Scope::new());

        output
    }

    pub fn new() -> Self {
        let mut output = Self::new_empty();

        // should never have an error
        add_std_lib(&mut output).unwrap();

        output
    }

    pub fn find_local(&self, identifier: &String) -> Option<&Value> {
        for scope in self.scopes.iter().rev() {
            if scope.local_exists(identifier) {
                return scope.get_local(identifier);
            }
        }

        return None;
    }

    pub fn get_global_scope_mut(&mut self) -> &mut Scope {
        self.scopes.front_mut().unwrap()
    }

    pub fn get_local_scope_mut(&mut self) -> &mut Scope {
        self.scopes.back_mut().unwrap()
    }

    pub fn expose(&mut self, name: &str, value: Value) -> Result<(), Error> {
        self.get_global_scope_mut()
            .set_local(&name.to_string(), value)
    }

    pub fn expose_function(&mut self, name: &str, function: NativeFunction) -> Result<(), Error> {
        self.expose(name, Value::NativeFunction(function))
    }

    pub fn expose_macro(&mut self, name: &str, r#macro: NativeMacro) -> Result<(), Error> {
        self.expose(name, Value::NativeMacro(r#macro))
    }

    pub fn eval(&mut self, source: &str) -> Result<Value, EvalError> {
        match tokeniser::tokenise(source) {
            Ok(tokens) => match parser::parse(tokens) {
                Ok(parent_node) => match parent_node.evaluate(self) {
                    Ok(value) => Ok(value),
                    Err(runtime_error) => Err(EvalError::RuntimeError(runtime_error)),
                },
                Err(parser_error) => Err(EvalError::ParserError(parser_error)),
            },
            Err(tokeniser_error) => Err(EvalError::TokeniserError(tokeniser_error)),
        }
    }
}
