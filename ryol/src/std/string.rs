use crate::prelude::*;
use ::std::fmt::Write;

pub fn add_string_lib(run_state: &mut RunState) -> Result<(), Error> {
    // could also be called "string" as it is also a conversion function
    run_state.expose_function("format", std_string_format)?;

    Ok(())
}

pub fn format_string(args: &Vec<Value>) -> Result<String, Error> {
    let mut output = String::new();

    for arg in args {
        match write!(output, "{}", arg) {
            Ok(()) => {}
            Err(error) => {
                return Err(Error::new(
                    format!("could not create error message: {}", error),
                    None,
                ));
            }
        }
    }

    Ok(output)
}

fn std_string_format(args: Vec<Value>) -> Result<Value, Error> {
    Ok(Value::String(format_string(&args)?))
}
