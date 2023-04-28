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
    pub fn get_local(&self, identifier: &String) -> Option<Variable> {
        Some(self.locals.get(identifier)?.clone())
    }

    pub fn get_local_mut(&mut self, identifier: &String) -> Option<&mut Variable> {
        self.locals.get_mut(identifier)
    }

    pub fn set_local(&mut self, identifier: &str, variable: Variable) {
        // ignore what was already there and just update value
        self.locals.insert(identifier.to_string(), variable);
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
        add_std_lib(&mut output);
        output
    }

    pub fn find_local(&self, identifier: &String) -> Option<Variable> {
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

    pub fn expose_function(&mut self, name: &str, function: NativeFunction) {
        self.get_global_scope_mut()
            .set_local(name, Variable::new(Value::NativeFunction(function)));
    }

    pub fn expose_macro(&mut self, name: &str, r#macro: NativeMacro) {
        self.get_global_scope_mut()
            .set_local(name, Variable::new(Value::NativeMacro(r#macro)));
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
