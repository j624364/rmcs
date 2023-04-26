use crate::function::{NativeFunction, NativeMacro};
use std::cmp;
use std::fmt;

#[derive(Clone, Default)]
pub enum Value {
    #[default]
    Null,

    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),

    List(Vec<Value>),

    NativeFunction(NativeFunction),
    NativeMacro(NativeMacro),
}

impl cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Boolean(x), Value::Boolean(y)) => x == y,
            (Value::Integer(x), Value::Integer(y)) => x == y,
            (Value::Float(x), Value::Float(y)) => x == y,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::List(_x), Value::List(_y)) => todo!(),
            (Value::NativeFunction(x), Value::NativeFunction(y)) => x == y,
            (Value::NativeMacro(x), Value::NativeMacro(y)) => {
                std::ptr::eq(x as *const NativeMacro, y as *const NativeMacro)
            }
            _ => false,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Null => "Value::Null".to_string(),
                Value::Boolean(boolean) => format!("Value::Boolean({})", boolean),
                Value::Integer(integer) => format!("Value::Integer({})", integer),
                Value::Float(float) => format!("Value::Float({})", float),
                Value::String(string) => format!("Value::String(\"{}\")", string),
                Value::List(_list) => todo!(),
                Value::NativeFunction(native_function) => format!(
                    "Value::NativeFunction({:#x})",
                    native_function as *const NativeFunction as u64
                ),
                Value::NativeMacro(native_macro) => format!(
                    "Value::NativeMacro({:#x})",
                    native_macro as *const NativeMacro as u64
                ),
            }
        )
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Value::Null => "()".to_string(),
            Value::Boolean(boolean) => format!("{}", boolean),
            Value::Integer(integer) => format!("{}", integer),
            Value::Float(float) => format!("{}", float),
            Value::String(string) => string.clone(),
            Value::List(_list) => {
                todo!()
            }
            Value::NativeFunction(native_function) => {
                let func_ptr = native_function as *const NativeFunction;

                format!("NativeFunction at {:#x}", func_ptr as u64)
            }
            Value::NativeMacro(native_macro) => {
                let func_ptr = native_macro as *const NativeMacro;

                format!("NativeMacro at {:#x}", func_ptr as u64)
            }
        };

        write!(f, "{}", output)
    }
}
