use crate::value::Value;

// todo: wrap value with a Rc
#[derive(Debug, Clone)]
pub struct Variable {
    value: Value,
    is_const: bool,
}

impl Variable {
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

    pub fn _set(&mut self, value: Value) {
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
