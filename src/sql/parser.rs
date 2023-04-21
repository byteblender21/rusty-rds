use crate::sql::ast::{Ast, Expression, SelectStatement, Statement};
use crate::sql::lexer::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,      // points to current token
    read_position: usize, // current read position (after current_token)
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        return Parser {
            tokens,
            position: 0,
            read_position: 0,
        };
    }

    pub fn create_ast(&mut self) -> Result<Ast, &'static str> {
        let mut nodes = vec![];

        while !self.is_at_end() {
            match self.parse_statement() {
                None => return Err("Could not parse next node from tokens"),
                Some(result) => nodes.push(result),
            }
        }

        return Ok(Ast { nodes });
    }

    pub fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = match self.read_token() {
            None => return None,
            Some(r) => r,
        };

        return match token.token_type {
            TokenType::Select => match self.parse_select() {
                Ok(r) => Some(r),
                Err(_) => None,
            },
            _ => None,
        };
    }

    fn parse_select(&mut self) -> Result<Box<SelectStatement>, &'static str> {
        let statement = SelectStatement::new();
        while !self.is_at_end() {
            //
        }

        return Ok(Box::new(statement));
    }

    fn parse_expression(&mut self) -> Result<Box<dyn Expression>, &'static str> {
        return Err("<not yet implemented>");
    }

    fn peek_token(&self) -> Option<&Token> {
        return if self.is_at_end() {
            None
        } else {
            self.tokens.get(self.read_position)
        };
    }

    fn read_token(&mut self) -> Option<&Token> {
        let token = if self.is_at_end() {
            None
        } else {
            self.tokens.get(self.read_position)
        };

        self.position = self.read_position;
        self.read_position += 1;

        return token;
    }

    fn is_at_end(&self) -> bool {
        return self.read_position >= self.tokens.len();
    }
}

#[cfg(test)]
mod tests {
    use crate::sql::lexer::Lexer;
    use crate::sql::parser::Parser;

    #[test]
    fn simple_select_of_number() {
        let input = "select 1;";
        let mut lexer = Lexer::new(input.to_string());
        let tokens = match lexer.tokenize_str() {
            Ok(result) => result,
            Err(err) => {
                assert!(false, "Lexing failed: {}", err);
                return;
            }
        };

        let mut parser = Parser::new(tokens);

        match parser.create_ast() {
            Ok(ast) => {
                assert_eq!(1, ast.nodes.len());
            }
            Err(err) => {
                assert!(false, "Parsing failed: {}", err)
            }
        }
    }
}
