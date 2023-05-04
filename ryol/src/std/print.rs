use crate::prelude::*;

// dont really know how to test these rn so todo: figure that out later

pub fn add_print_lib(run_state: &mut RunState) -> Result<(), Error> {
    run_state.expose_function("print", std_print_print)?;
    run_state.expose_function("println", std_print_println)?;

    Ok(())
}

pub fn std_print_print(args: Vec<Value>) -> Result<Value, Error> {
    let output = crate::std::string::format_string(&args)?;
    print!("{}", output);

    // this is probably more efficient but i want to be sure
    // that the output is identical to format
    // for arg in args {
    //     print!("{}", arg);
    // }

    Ok(Value::default())
}

pub fn std_print_println(args: Vec<Value>) -> Result<Value, Error> {
    std_print_print(args)?;
    println!();
    Ok(Value::default())
}
