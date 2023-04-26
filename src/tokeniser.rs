use std::fmt;

#[derive(Debug)]
pub struct TokeniserError {
    message: String,
    line_no: usize,
    col_no: usize,
}

impl TokeniserError {
    pub fn new(tokeniser_state: &TokeniserState, message: String) -> Self {
        Self {
            message,
            line_no: tokeniser_state.line_no,
            col_no: tokeniser_state.col_no,
        }
    }

    pub fn get_message(&self) -> &String {
        &self.message
    }

    pub fn get_line_no(&self) -> usize {
        self.line_no
    }

    pub fn get_col_no(&self) -> usize {
        self.col_no
    }
}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} at {}:{}",
            self.token_type, self.line_no, self.col_no
        )
    }
}

// todo: make private
#[derive(Debug, Clone)]
pub struct TokeniserState {
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

pub fn tokenise(source: &str) -> Result<Vec<Token>, TokeniserError> {
    let mut tokeniser_state = TokeniserState::new(source);

    enum TokeniserMode {
        Normal,
        String,
        StringEscape,
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
                    tokeniser_state.push_token(
                        TokenType::String(tokeniser_state.current_token.clone()),
                        tokeniser_state.current_token.len(),
                    );
                    tokeniser_mode = TokeniserMode::Normal;
                }
                '\\' => {
                    tokeniser_mode = TokeniserMode::StringEscape;
                }
                _ => {
                    tokeniser_state.current_token.push(c);
                }
            },
            TokeniserMode::StringEscape => {
                // mostly just the c ones
                // todo: add custom numbers
                let escape_char_res: Result<char, TokeniserError> = match c {
                    'a' => Ok(0x07 as char),
                    'b' => Ok(0x08 as char),
                    'e' => Ok(0x1b as char),
                    'f' => Ok(0x0c as char),
                    'n' => Ok(0x0a as char),
                    'r' => Ok(0x0d as char),
                    't' => Ok(0x09 as char),
                    'v' => Ok(0x0b as char),
                    '\\' => Ok('\\'),
                    '\'' => Ok('\''),
                    '"' => Ok('"'),
                    _ => Err(TokeniserError::new(
                        &tokeniser_state,
                        format!("Invalid escape escape character: \\{}", c),
                    )),
                };

                match escape_char_res {
                    Ok(escape_char) => {
                        tokeniser_state.current_token.push(escape_char);
                        tokeniser_mode = TokeniserMode::String;
                    }

                    Err(error) => {
                        return Err(error);
                    }
                }
            }
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

    tokeniser_state.try_push_token();

    Ok(tokeniser_state.get_tokens())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn panic_source(source: &str, message: &str) {
        println!("{}, panics", source);

        println!("generated tokens:");
        for token in tokenise(source).unwrap() {
            println!("\t{:?}", token);
        }

        panic!("{}", message)
    }

    fn match_tokens(source: &str, token_types: Vec<TokenType>) {
        let tokens = tokenise(source).unwrap();

        if tokens.len() != token_types.len() {
            panic_source(
                source,
                format!(
                    "incorrect number of tokens generated ({} vs {})",
                    source.len(),
                    token_types.len()
                )
                .as_str(),
            )
        }

        for (current_token, matched_token_type) in tokens.iter().zip(token_types.iter()) {
            if current_token.token_type != *matched_token_type {
                panic_source(
                    source,
                    format!(
                        "tokens do not match ({:?} vs {:?})",
                        current_token.token_type, *matched_token_type
                    )
                    .as_str(),
                )
            }
        }
    }

    #[test]
    fn literals_tests() {
        match_tokens("1", vec![TokenType::Integer(1)]);
        match_tokens("-1", vec![TokenType::Integer(-1)]);
        match_tokens("1.04", vec![TokenType::Float(1.04)]);
        match_tokens("\"Hello\"", vec![TokenType::String("Hello".to_string())]);
    }

    #[test]
    fn string_escape_tests() {
        // this was fun to do
        match_tokens("\"asdf\"", vec![TokenType::String("asdf".to_string())]);
        match_tokens("\"\\n\"", vec![TokenType::String("\n".to_string())]);
        match_tokens("\"\\r\"", vec![TokenType::String("\r".to_string())]);
        match_tokens("\"\\t\"", vec![TokenType::String("\t".to_string())]);
        match_tokens("\"\\\\\"", vec![TokenType::String("\\".to_string())]);
        match_tokens("\"\\\"\"", vec![TokenType::String("\"".to_string())]);
        match_tokens("\"\\\'\"", vec![TokenType::String("\'".to_string())]);

        assert!(tokenise("\"\\x\"").is_err());
    }

    #[test]
    fn basic_expr_tests() {
        match_tokens(
            "(+ 1 2)",
            vec![
                TokenType::LBracket,
                TokenType::Identifier("+".to_string()),
                TokenType::Integer(1),
                TokenType::Integer(2),
                TokenType::RBracket,
            ],
        );

        match_tokens(
            "(+ 1 2)",
            vec![
                TokenType::LBracket,
                TokenType::Identifier("+".to_string()),
                TokenType::Integer(1),
                TokenType::Integer(2),
                TokenType::RBracket,
            ],
        );

        match_tokens(
            "(println \"Hello, World!\")",
            vec![
                TokenType::LBracket,
                TokenType::Identifier("println".to_string()),
                TokenType::String("Hello, World!".to_string()),
                TokenType::RBracket,
            ],
        );
    }
}
