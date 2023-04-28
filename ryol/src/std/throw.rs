use crate::prelude::*;
use ::std::fmt::Write;

pub fn add_throw_lib(run_state: &mut RunState) -> Result<(), Error> {
    run_state.expose_function("throw", std_throw)?;

    Ok(())
}

fn std_throw(args: Vec<Value>) -> Result<Value, Error> {
    // todo: eventually chuck straight into format function
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

    Err(Error::new(output, None))
}
