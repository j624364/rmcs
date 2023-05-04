use crate::function::{NativeFunction, NativeMacro};
use std::cmp;
use std::fmt::{self, Write};

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

fn compare_list(x_list: &Vec<Value>, y_list: &Vec<Value>) -> bool {
    if x_list.len() != y_list.len() {
        return false;
    }

    for (x, y) in x_list.iter().zip(y_list.iter()) {
        if x != y {
            return false;
        }
    }

    true
}

impl cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Boolean(x), Value::Boolean(y)) => x == y,
            (Value::Integer(x), Value::Integer(y)) => x == y,
            (Value::Float(x), Value::Float(y)) => x == y,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::List(x_list), Value::List(y_list)) => compare_list(x_list, y_list),
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
                Value::List(list) => format!("{:?}", list),
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

pub fn list_to_string(list: &Vec<Value>) -> Result<String, fmt::Error> {
    let mut output = String::new();

    write!(&mut output, "(list ")?;
    for (i, element) in list.iter().enumerate() {
        write!(&mut output, "{}", element)?;

        if i + 1 < list.len() {
            write!(&mut output, " ")?;
        }
    }
    write!(&mut output, ")")?;

    Ok(output)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Value::Null => "()".to_string(),
            Value::Boolean(boolean) => format!("{}", boolean),
            Value::Integer(integer) => format!("{}", integer),
            Value::Float(float) => {
                // ensure always has 1 dp of precision
                if *float % 1.0 == 0.0 {
                    format!("{:.1}", float)
                } else {
                    format!("{}", float)
                }
            }
            Value::String(string) => string.clone(),
            Value::List(list) => list_to_string(list)?,
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
