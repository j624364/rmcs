use crate::error::Error;
use crate::node::Node;
use crate::run_state::RunState;
use crate::value::Value;

pub type NativeFunction = fn(Vec<Value>) -> Result<Value, Error>;
pub type NativeMacro = fn(&mut RunState, &Node) -> Result<Value, Error>;
