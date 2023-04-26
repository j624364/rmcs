use crate::tokeniser::Token;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    message: String,
    token: Option<Token>,
}

impl Error {
    pub fn new(message: String, token: Option<Token>) -> Self {
        Self { message, token }
    }

    pub fn set_token(&mut self, token: Token) {
        self.token = Some(token)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token_output = match &self.token {
            Some(token) => {
                format!(" {}", token)
            }
            None => String::new(),
        };

        write!(f, "Runtime Error: \"{}\"{}", self.message, token_output)
    }
}
