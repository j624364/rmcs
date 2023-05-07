use crate::value::Value;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
struct VariableInstance {
    value: Value,
    is_const: bool,
}

impl VariableInstance {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            is_const: false,
        }
    }

    pub fn _new_const(value: Value) -> Self {
        Self {
            value,
            is_const: true,
        }
    }

    pub fn set(&mut self, value: Value) {
        assert!(!self.is_const);
        self.value = value;
    }

    pub fn get(&self) -> &Value {
        &self.value
    }

    pub fn is_const(&self) -> bool {
        self.is_const
    }
}

// todo: wrap value with a Rc
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    inner: Rc<RefCell<VariableInstance>>
}

impl Variable {
    pub fn new(value: Value) -> Self {
        Self {
            inner: Rc::new(RefCell::new(VariableInstance::new(value)))
        }
    }

    pub fn _new_const(value: Value) -> Self {
        // todo: make const
        Self {
            inner: Rc::new(RefCell::new(VariableInstance::new(value)))
        }
    }

    pub fn set(&self, value: Value) {
        self.inner.borrow_mut().set(value)
    }

    pub fn get(&self) -> Value {
        self.inner.borrow().get().clone()
    }

    pub fn is_const(&self) -> bool {
        self.inner.borrow().is_const()
    }
}
