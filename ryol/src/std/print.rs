use crate::prelude::*;

// dont really know how to test these rn so todo: figure that out later

pub fn add_print_lib(run_state: &mut RunState) {
    let scope = run_state.get_global_scope_mut();

    scope.set_local(
        "print",
        Variable::new(Value::NativeFunction(std_print_print)),
    );
    scope.set_local(
        "println",
        Variable::new(Value::NativeFunction(std_print_println)),
    );
}

pub fn std_print_print(args: Vec<Value>) -> Result<Value, Error> {
    for arg in args {
        print!("{}", arg);
    }

    Ok(Value::default())
}

pub fn std_print_println(args: Vec<Value>) -> Result<Value, Error> {
    std_print_print(args)?;
    println!();
    Ok(Value::default())
}
