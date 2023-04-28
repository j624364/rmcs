use crate::prelude::*;

pub fn add_list_lib(run_state: &mut RunState) {
    run_state.expose_function("list", std_list_list);
}

fn std_list_list(args: Vec<Value>) -> Result<Value, Error> {
    Ok(Value::List(args))
}
