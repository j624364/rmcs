use crate::node::Node;
use crate::tokeniser::{Token, TokenType};
use std::collections::VecDeque;
use std::fmt;

#[derive(Debug)]
pub struct ParserError {
    message: String,
    token: Token,
}

impl ParserError {
    pub fn get_message(&self) -> &String {
        &self.message
    }

    pub fn get_token(&self) -> &Token {
        &self.token
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\" at [{}]", self.get_message(), self.get_token())
    }
}

struct ParserState {
    tokens: VecDeque<Token>,
}

impl ParserState {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: VecDeque::from(tokens),
        }
    }

    pub fn eat_token(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }
}

fn parse_node(parser_state: &mut ParserState) -> Result<Node, ParserError> {
    // deal with node vertex
    let mut node = match parser_state.eat_token() {
        Some(node_token) => {
            if *node_token.get_token_type() == TokenType::LBracket {
                let mut node = Node::new(None);
                node.add_child(parse_node(parser_state)?);
                node
            }
            else if *node_token.get_token_type() == TokenType::RBracket {
                Node::new(None)
            }
            else {
                Node::new(Some(node_token))
            }
        }
        None => Node::new(None),
    };

    // now handle children
    loop {
        match parser_state.eat_token() {
            Some(token) => match token.get_token_type() {
                TokenType::LBracket => {
                    node.add_child(parse_node(parser_state)?);
                }
                TokenType::RBracket => {
                    return Ok(node);
                }
                TokenType::Identifier(_)
                | TokenType::Integer(_)
                | TokenType::Float(_)
                | TokenType::String(_) => node.add_child(Node::new(Some(token))),
            },
            None => {
                return Ok(node);
            }
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Node, ParserError> {
    let mut parser_state = ParserState::new(tokens);
    parse_node(&mut parser_state)
}
