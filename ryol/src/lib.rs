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

pub fn eval(source: &str, run_state_option: Option<&mut RunState>) -> Result<Value, EvalError> {
    let mut default_run_state = RunState::new();

    let mut run_state = run_state_option.unwrap_or(&mut default_run_state);

    match tokeniser::tokenise(source) {
        Ok(tokens) => match parser::parse(tokens) {
            Ok(parent_node) => match parent_node.evaluate(&mut run_state) {
                Ok(value) => Ok(value),
                Err(runtime_error) => Err(EvalError::RuntimeError(runtime_error)),
            },
            Err(parser_error) => Err(EvalError::ParserError(parser_error)),
        },
        Err(tokeniser_error) => Err(EvalError::TokeniserError(tokeniser_error)),
    }
}
