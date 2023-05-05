use crate::prelude::*;

pub fn get_identifier(node: &Node) -> Result<&String, Error> {
    match node.get_token() {
        Some(token) => match token.get_token_type() {
            TokenType::Identifier(identifier) => Ok(identifier),
            _ => Err(Error::new(
                "must be an identifier token type".to_string(),
                Some(token.clone()),
            )),
        },
        None => Err(Error::new("node must have a token".to_string(), None)),
    }
}
