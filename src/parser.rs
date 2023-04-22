use crate::node::Node;
use crate::tokeniser::{Token, TokenType};
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct ParserError {
    message: String,
    token: Token,
}

struct ParserState {
    index: usize,
    tokens: VecDeque<Token>,
}

impl ParserState {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            index: 0,
            tokens: VecDeque::from(tokens),
        }
    }

    pub fn has_token(&self) -> bool {
        self.index < self.tokens.len()
    }

    pub fn eat_token(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }
}

fn parse_node(parser_state: &mut ParserState) -> Result<RefCell<Node>, ParserError> {
    // deal with node vertex
    let mut node = match parser_state.eat_token() {
        Some(node_token) => {
            if *node_token.get_token_type() == TokenType::LBracket {
                let mut node = Node::new(None);
                node.add_child(parse_node(parser_state)?);
                node
            } else {
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
                    return Ok(RefCell::new(node));
                }
                TokenType::Identifier(_)
                | TokenType::Integer(_)
                | TokenType::Float(_)
                | TokenType::String(_) => node.add_child(RefCell::new(Node::new(Some(token)))),
            },
            None => {
                return Ok(RefCell::new(node));
            }
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<RefCell<Node>, ParserError> {
    let mut parser_state = ParserState::new(tokens);
    parse_node(&mut parser_state)
}
