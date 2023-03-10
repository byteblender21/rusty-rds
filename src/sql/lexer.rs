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

#[derive(Eq, PartialEq)]
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

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.current_char.is_none() {
            return None;
        }

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
                    let literal = self.read_identifier();

                    Some(Token {
                        token_type: lookup_identifier(literal.as_str()),
                        literal,
                    })
                } else if current_char.is_numeric() {
                    Some(Token {
                        token_type: TokenType::Number,
                        literal: self.read_number(),
                    })
                } else {
                    None
                }
            }
        };

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
        return if self.read_position >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.read_position)
        };
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = self.input.chars().nth(self.read_position);
        }

        self.position = self.read_position;
        self.read_position += 1;
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
}
