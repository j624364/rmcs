use std::io::Error;

#[derive(Debug, Clone)]
pub enum TokenType {
    Identifier(String),
    LBracket,
    RBracket,
    Integer(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    line_no: usize,
    col_no: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line_no: usize, col_no: usize) -> Self {
        Self {
            token_type,
            line_no,
            col_no,
        }
    }
}

#[derive(Debug, Clone)]
struct TokeniserState {
    pub output: Vec<Token>,
    pub current_token: String,
    pub line_no: usize,
    pub col_no: usize,
}

impl TokeniserState {
    pub fn new(source: &str) -> Self {
        let output_capacity = source.len() / 4;

        Self {
            output: Vec::with_capacity(output_capacity),
            current_token: String::with_capacity(128),
            line_no: 1,
            col_no: 1,
        }
    }

    pub fn push_token(&mut self, token_type: TokenType, token_length: usize) {
        let col_no = self.col_no - token_length;

        self.output
            .push(Token::new(token_type, self.line_no, col_no));
        self.current_token.clear();
    }

    pub fn try_push_token(&mut self) {
        if self.current_token.is_empty() {
            return;
        }

        if let Ok(integer) = self.current_token.parse::<i64>() {
            self.push_token(TokenType::Integer(integer), self.current_token.len());
        } else if let Ok(float) = self.current_token.parse::<f64>() {
            self.push_token(TokenType::Float(float), self.current_token.len());
        } else {
            self.push_token(
                TokenType::Identifier(self.current_token.clone()),
                self.current_token.len(),
            );
        }
    }

    pub fn get_tokens(self) -> Vec<Token> {
        self.output
    }
}

pub fn tokenise(source: &str) -> Result<Vec<Token>, Error> {
    let mut tokeniser_state = TokeniserState::new(source);

    enum TokeniserMode {
        Normal,
        String,
        SingleLineComment,
    }

    let mut tokeniser_mode = TokeniserMode::Normal;

    for c in source.chars() {
        match tokeniser_mode {
            TokeniserMode::Normal => match c {
                '(' => {
                    tokeniser_state.try_push_token();
                    tokeniser_state.push_token(TokenType::LBracket, 1);
                }
                ')' => {
                    tokeniser_state.try_push_token();
                    tokeniser_state.push_token(TokenType::RBracket, 1);
                }
                ' ' | '\t' | '\r' | '\n' => {
                    tokeniser_state.try_push_token();
                }
                '"' => {
                    tokeniser_state.try_push_token();
                    tokeniser_mode = TokeniserMode::String;
                }
                ';' => {
                    tokeniser_state.try_push_token();
                    tokeniser_mode = TokeniserMode::SingleLineComment;
                }
                _ => {
                    tokeniser_state.current_token.push(c);
                }
            },
            TokeniserMode::String => match c {
                '"' => {
                    tokeniser_mode = TokeniserMode::Normal;
                }
                '\\' => {}
                _ => {
                    tokeniser_state.current_token.push(c);
                }
            },
            TokeniserMode::SingleLineComment => {
                if c == '\n' {
                    tokeniser_mode = TokeniserMode::Normal;
                }
            }
        }

        tokeniser_state.col_no += 1;

        if c == '\n' {
            tokeniser_state.line_no += 1;
            tokeniser_state.col_no = 1;
        }
    }

    Ok(tokeniser_state.get_tokens())
}
