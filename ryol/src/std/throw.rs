use crate::prelude::*;

pub fn add_throw_lib(run_state: &mut RunState) -> Result<(), Error> {
    run_state.expose_function("throw", std_throw)?;

    Ok(())
}

fn std_throw(args: Vec<Value>) -> Result<Value, Error> {
    let output = crate::std::string::format_string(&args)?;
    Err(Error::new(output, None))
}
