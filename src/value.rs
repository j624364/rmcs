use crate::function::NativeFunction;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Value {
    #[default]
    Null,

    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),

    List(Vec<Value>),
    NativeFunction(NativeFunction),
}
