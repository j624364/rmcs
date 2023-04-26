pub mod error;
pub mod function;
pub mod node;
pub mod parser;
pub mod run_state;
pub mod std;
pub mod tokeniser;
pub mod value;
pub mod variable;

use ::std::fmt;
use run_state::RunState;
use value::Value;

#[derive(Debug)]
pub enum EvalError {
    TokeniserError(tokeniser::TokeniserError),
    ParserError(parser::ParserError),
    RuntimeError(error::Error),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::TokeniserError(tokeniser_error) => {
                write!(f, "Tokeniser Error: {}", tokeniser_error)
            }
            EvalError::ParserError(parser_error) => write!(f, "Parser Error: {}", parser_error),
            EvalError::RuntimeError(runtime_error) => write!(f, "Runtime Error: {}", runtime_error),
        }
    }
}

pub fn eval(source: &str) -> Result<Value, EvalError> {
    let mut run_state = RunState::new();
    run_state.eval(source)
}

pub mod prelude {
    pub use crate::error::Error;
    pub use crate::eval;
    pub use crate::function::{NativeFunction, NativeMacro};
    pub use crate::node::Node;
    pub use crate::run_state::RunState;
    pub use crate::value::Value;
    pub use crate::variable::Variable;
}
