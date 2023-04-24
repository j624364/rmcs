use crate::error::Error;
use crate::value::Value;

pub type NativeFunction = fn(Vec<Value>) -> Result<Value, Error>;
