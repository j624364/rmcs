use crate::tokeniser::Token;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node {
    token: Option<Token>,
    children: Vec<RefCell<Node>>,
}

impl Node {
    pub fn new(token: Option<Token>) -> Self {
        Node {
            token,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: RefCell<Node>) {
        self.children.push(child);
    }

    pub fn get_token(&self) -> &Option<Token> {
        &self.token
    }

    pub fn get_children(&self) -> &Vec<RefCell<Node>> {
        &self.children
    }
}
