use crate::function::NativeFunction;
use std::fmt;
use std::string;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Value {
    #[default]
    Null,

    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),

    List(Vec<Value>),
    NativeFunction(NativeFunction),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Value::Null => "()".to_string(),
            Value::Boolean(boolean) => format!("{}", boolean),
            Value::Integer(integer) => format!("{}", integer),
            Value::Float(float) => format!("{}", float),
            Value::String(string) => string.clone(),
            Value::List(list) => {
                todo!()
            }
            Value::NativeFunction(native_function) => {
                let func_ptr = native_function as *const NativeFunction;

                format!("NativeFunction at {}", func_ptr as u64)
            }
        };

        write!(f, "{}", output)
    }
}
