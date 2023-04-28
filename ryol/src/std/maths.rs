use crate::error::Error;
use crate::prelude::*;

pub fn add_maths_lib(run_state: &mut RunState) -> Result<(), Error> {
    // main operators
    run_state.expose_function("+", std_maths_add)?;
    run_state.expose_function("-", std_maths_sub)?;
    run_state.expose_function("*", std_maths_mlt)?;
    run_state.expose_function("/", std_maths_div)?;

    // aliases for the operators
    run_state.expose_function("×", std_maths_mlt)?;
    run_state.expose_function("÷", std_maths_div)?;

    Ok(())
}

pub fn get_non_num_type_error(function_name: &str, arg: &Value) -> Error {
    Error::new(
        format!(
            "non numerical type: {:?} in function \"{}\"",
            arg, function_name
        ),
        None,
    )
}

fn std_maths_add(args: Vec<Value>) -> Result<Value, Error> {
    match args.len() {
        0 => Ok(Value::default()),
        1 => Ok(args.first().unwrap().clone()),
        _ => {
            let mut sum = Value::Integer(0);

            for arg in &args {
                // i love rust so much for this
                sum = match (sum, arg) {
                    (Value::Integer(x), Value::Integer(y)) => Value::Integer(x + y),
                    (Value::Integer(x), Value::Float(y)) => Value::Integer(x + *y as i64),
                    (Value::Float(x), Value::Integer(y)) => Value::Float(x + *y as f64),
                    (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
                    _ => {
                        return Err(get_non_num_type_error("+", arg));
                    }
                };
            }

            Ok(sum)
        }
    }
}

fn std_maths_sub(args: Vec<Value>) -> Result<Value, Error> {
    match args.len() {
        0 => Ok(Value::default()),
        1 => {
            let arg = args.first().unwrap();
            match &arg {
                Value::Integer(x) => Ok(Value::Integer(-x)),
                Value::Float(x) => Ok(Value::Float(-x)),
                _ => Err(get_non_num_type_error("-", arg)),
            }
        }
        _ => {
            let mut sum = args.first().unwrap().clone();

            for arg in &args[1..] {
                // i love rust so much for this
                sum = match (sum, arg) {
                    (Value::Integer(x), Value::Integer(y)) => Value::Integer(x - y),
                    (Value::Integer(x), Value::Float(y)) => Value::Integer(x - *y as i64),
                    (Value::Float(x), Value::Integer(y)) => Value::Float(x - *y as f64),
                    (Value::Float(x), Value::Float(y)) => Value::Float(x - y),
                    _ => {
                        return Err(get_non_num_type_error("-", arg));
                    }
                };
            }

            Ok(sum)
        }
    }
}

fn std_maths_mlt(args: Vec<Value>) -> Result<Value, Error> {
    match args.len() {
        0 => Ok(Value::default()),
        1 => Ok(args.first().unwrap().clone()),
        _ => {
            let mut sum = Value::Integer(1);

            for arg in &args {
                // i love rust so much for this
                sum = match (sum, arg) {
                    (Value::Integer(x), Value::Integer(y)) => Value::Integer(x * y),
                    (Value::Integer(x), Value::Float(y)) => Value::Integer(x * *y as i64),
                    (Value::Float(x), Value::Integer(y)) => Value::Float(x * *y as f64),
                    (Value::Float(x), Value::Float(y)) => Value::Float(x * y),
                    _ => {
                        return Err(get_non_num_type_error("*", arg));
                    }
                };
            }

            Ok(sum)
        }
    }
}

fn std_maths_div(args: Vec<Value>) -> Result<Value, Error> {
    match args.len() {
        0 => Ok(Value::default()),
        1 => {
            let arg = args.first().unwrap();
            match &arg {
                Value::Integer(x) => Ok(Value::Integer(-x)),
                Value::Float(x) => Ok(Value::Float(-x)),
                _ => Err(get_non_num_type_error("-", arg)),
            }
        }
        _ => {
            let mut sum = args.first().unwrap().clone();

            for arg in &args[1..] {
                // i love rust so much for this
                sum = match (sum, arg) {
                    (Value::Integer(x), Value::Integer(y)) => Value::Integer(x / y),
                    (Value::Integer(x), Value::Float(y)) => Value::Integer(x / *y as i64),
                    (Value::Float(x), Value::Integer(y)) => Value::Float(x / *y as f64),
                    (Value::Float(x), Value::Float(y)) => Value::Float(x / y),
                    _ => {
                        return Err(get_non_num_type_error("/", arg));
                    }
                };
            }

            Ok(sum)
        }
    }
}
