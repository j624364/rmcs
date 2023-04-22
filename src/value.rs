#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,

    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),

    List(Vec<Value>),
}
