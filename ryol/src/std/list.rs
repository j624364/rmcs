use crate::error::Error;
use crate::run_state::RunState;
use crate::value::Value;
use crate::variable::Variable;

pub fn add_list_lib(run_state: &mut RunState) {
    let scope = run_state.get_global_scope_mut();

    scope.set_local("list", Variable::new(Value::NativeFunction(std_list_list)));
}

fn std_list_list(args: Vec<Value>) -> Result<Value, Error> {
    Ok(Value::List(args))
}
