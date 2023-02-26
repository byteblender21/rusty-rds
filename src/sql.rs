use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum TokenType {
    // identifiers + literals
    Identifier,
    Number,

    // operators
    SemiColon,

    // keywords
    Select,
    From,
    Where,
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

// pub struct Definition {
//     pub name: String,
//     pub operand_widths: Vec<i16>,
// }

// lazy_static! {
//     static ref DEFINITIONS: HashMap<TokenType, Definition> = {
//         let mut m = HashMap::new();
//         m.insert(
//             TokenType::Select,
//             Definition {
//                 name: String::from("select"),
//                 operand_widths: vec![2],
//             },
//         );
//         m
//     };
// }

fn lookup_identifier(identifier: &str) -> TokenType {
    return match identifier.to_lowercase().as_str() {
        "select" => TokenType::Select,
        "from" => TokenType::From,
        "where" => TokenType::Where,
        "insert" => TokenType::Insert,
        "update" => TokenType::Update,
        "delete" => TokenType::Delete,
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
            if current_char.is_alphanumeric() {
                self.read_char();
            } else {
                break;
            }
        }

        return self
            .input
            .chars()
            .skip(beginning_pos)
            .take(self.position)
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
            .take(self.position)
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

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = self.input.chars().nth(self.read_position as usize);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}

pub struct Parser {}

#[cfg(test)]
mod tests {
    use crate::sql::{Lexer, TokenType};

    #[test]
    fn it_works() {
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
}
