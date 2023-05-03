#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum TokenType {
    // identifiers + literals
    Identifier,
    Number,

    // operators
    Equal,
    Lower,
    LowerEqual,
    Greater,
    GreaterEqual,
    Star,

    // delimiters
    SemiColon,
    Dot,
    Comma,
    LeftParen,
    RightParen,

    // keywords
    Select,
    From,
    Where,
    Is,
    Not,
    Null,
    OrderBy,

    Insert,
    Update,
    Delete,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

fn lookup_identifier(identifier: &str) -> TokenType {
    return match identifier.to_lowercase().as_str() {
        "select" => TokenType::Select,
        "from" => TokenType::From,
        "where" => TokenType::Where,
        "insert" => TokenType::Insert,
        "update" => TokenType::Update,
        "delete" => TokenType::Delete,
        "is" => TokenType::Is,
        "not" => TokenType::Not,
        "null" => TokenType::Null,
        _ => TokenType::Identifier,
    };
}

pub struct Lexer {
    input: String,
    position: usize,      // points to current char
    read_position: usize, // current read position (after current_char)
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            current_char: None,
        };

        lexer.read_char();

        return lexer;
    }

    pub fn tokenize_str(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            match self.next_token() {
                None => {
                    return Err(format!(
                        "Invalid token found: {}",
                        self.current_char.unwrap()
                    ))
                }
                Some(token) => tokens.push(token),
            }
        }

        match self.next_token() {
            None => return Err(format!("Invalid token found: {:?}", self.current_char)),
            Some(token) => tokens.push(token),
        }

        return Ok(tokens);
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.current_char.is_none() {
            return None;
        }

        let mut has_char_consumed = false;
        let current_char = self.current_char.unwrap();

        let token = match current_char {
            ';' => Some(Token {
                token_type: TokenType::SemiColon,
                literal: current_char.to_string(),
            }),
            ',' => Some(Token {
                token_type: TokenType::Comma,
                literal: current_char.to_string(),
            }),
            '.' => Some(Token {
                token_type: TokenType::Dot,
                literal: current_char.to_string(),
            }),
            '=' => Some(Token {
                token_type: TokenType::Equal,
                literal: current_char.to_string(),
            }),
            '*' => Some(Token {
                token_type: TokenType::Star,
                literal: current_char.to_string(),
            }),
            '(' => Some(Token {
                token_type: TokenType::LeftParen,
                literal: current_char.to_string(),
            }),
            ')' => Some(Token {
                token_type: TokenType::RightParen,
                literal: current_char.to_string(),
            }),
            '>' => {
                let next_char = self.peek_char();
                if next_char.is_none() {
                    Some(Token {
                        token_type: TokenType::Greater,
                        literal: current_char.to_string(),
                    })
                } else if next_char.unwrap() == '=' {
                    Some(Token {
                        token_type: TokenType::GreaterEqual,
                        literal: ">=".to_string(),
                    })
                } else {
                    Some(Token {
                        token_type: TokenType::Greater,
                        literal: current_char.to_string(),
                    })
                }
            }
            '<' => {
                let next_char = self.peek_char();
                if next_char.is_none() {
                    Some(Token {
                        token_type: TokenType::Lower,
                        literal: current_char.to_string(),
                    })
                } else if next_char.unwrap() == '=' {
                    Some(Token {
                        token_type: TokenType::LowerEqual,
                        literal: "<=".to_string(),
                    })
                } else {
                    Some(Token {
                        token_type: TokenType::Lower,
                        literal: current_char.to_string(),
                    })
                }
            }
            _ => {
                if current_char.is_alphabetic() {
                    has_char_consumed = true;
                    let literal = self.read_identifier();

                    Some(Token {
                        token_type: lookup_identifier(literal.as_str()),
                        literal,
                    })
                } else if current_char.is_numeric() {
                    has_char_consumed = true;
                    Some(Token {
                        token_type: TokenType::Number,
                        literal: self.read_number(),
                    })
                } else {
                    None
                }
            }
        };

        if !has_char_consumed {
            self.read_char();
        }

        return token;
    }

    fn read_identifier(&mut self) -> String {
        let beginning_pos = self.position;

        while let Some(current_char) = self.current_char {
            if current_char.is_alphanumeric() || current_char == '_' {
                self.read_char();
            } else {
                break;
            }
        }

        return self
            .input
            .chars()
            .skip(beginning_pos)
            .take(self.position - beginning_pos)
            .collect();
    }

    fn read_number(&mut self) -> String {
        let beginning_pos = self.position;

        while let Some(current_char) = self.current_char {
            if current_char.is_numeric() {
                self.read_char();
            } else {
                break;
            }
        }

        return self
            .input
            .chars()
            .skip(beginning_pos)
            .take(self.position - beginning_pos)
            .collect();
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_some() {
            let character = self.current_char.unwrap();
            if character.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        return if self.is_at_end() {
            None
        } else {
            self.input.chars().nth(self.read_position)
        };
    }

    fn read_char(&mut self) {
        if self.is_at_end() {
            self.current_char = None;
        } else {
            self.current_char = self.input.chars().nth(self.read_position);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn is_at_end(&self) -> bool {
        return self.read_position >= self.input.len();
    }
}

#[cfg(test)]
mod tests {
    use crate::sql::lexer::{Lexer, TokenType};

    #[test]
    fn simple_select_of_number() {
        let input = "select 1;";
        let mut lexer = Lexer::new(input.to_string());

        let token = lexer.next_token();
        assert!(token.is_some());
        assert_eq!(TokenType::Select, token.unwrap().token_type);

        let token = lexer.next_token();
        assert!(token.is_some());
        assert_eq!(TokenType::Number, token.unwrap().token_type);

        let token = lexer.next_token();
        assert!(token.is_some());
        assert_eq!(TokenType::SemiColon, token.unwrap().token_type);
    }

    #[test]
    fn simple_select_of_column_from_table() {
        let input = "select foo from my_table;";
        let mut lexer = Lexer::new(input.to_string());

        let token = lexer.next_token();
        assert!(token.is_some());
        assert_eq!(TokenType::Select, token.unwrap().token_type);

        let token = lexer.next_token();
        assert!(token.is_some());
        let token = token.unwrap();
        assert_eq!(TokenType::Identifier, token.token_type);
        assert_eq!("foo", token.literal.as_str());

        let token = lexer.next_token();
        assert!(token.is_some());
        assert_eq!(TokenType::From, token.unwrap().token_type);

        let token = lexer.next_token();
        assert!(token.is_some());
        let token = token.unwrap();
        assert_eq!(TokenType::Identifier, token.token_type);
        assert_eq!("my_table", token.literal.as_str());
    }

    #[test]
    fn simple_select_of_column_from_table_to_token_list() {
        let input = "select foo from my_table;";
        let mut lexer = Lexer::new(input.to_string());

        match lexer.tokenize_str() {
            Ok(r) => assert_eq!(5, r.len(), "{:?}", r),
            Err(err) => assert!(false, "{}", err),
        }
    }

    #[test]
    fn simple_select_all_from_table_to_token_list() {
        let input = "select * from my_table;";
        let mut lexer = Lexer::new(input.to_string());

        match lexer.tokenize_str() {
            Ok(r) => assert_eq!(5, r.len()),
            Err(err) => assert!(false, "{}", err),
        }
    }
}
