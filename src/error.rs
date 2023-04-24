use crate::tokeniser::Token;

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
