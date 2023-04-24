use crate::variable::Variable;
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

    pub fn get_local(&self, identifier: &String) -> Option<Variable> {
        Some(self.locals.get(identifier)?.clone())
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
    pub fn new() -> Self {
        let mut output = Self {
            scopes: VecDeque::with_capacity(512),
        };

        // make sure there is one global scope
        output.scopes.push_back(Scope::new());

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
}
