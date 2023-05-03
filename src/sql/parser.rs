use crate::sql::ast::{Ast, Expression, NumberExpr, SelectStatement, Statement};
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

    pub fn create_ast(&mut self) -> Result<Ast, String> {
        let mut nodes = vec![];

        while !self.is_at_end() {
            match self.parse_statement() {
                Err(err) => return Err(format!("Error during parsing: {}", err)),
                Ok(result) => nodes.push(result),
            }
        }

        return Ok(Ast { statements: nodes });
    }

    pub fn parse_statement(&mut self) -> Result<Box<dyn Statement>, String> {
        let token = match self.read_token() {
            None => return Err("No token found".to_string()),
            Some(r) => r,
        };

        return match token.token_type {
            TokenType::Select => match self.parse_select() {
                Ok(r) => Ok(r),
                Err(err) => Err(err.to_string()),
            },
            _ => Err(format!("Could not handle token: {:?}", token.clone())),
        };
    }

    fn parse_select(&mut self) -> Result<Box<SelectStatement>, String> {
        let mut statement = SelectStatement::new();
        while !self.is_at_end() {
            match self.parse_expression() {
                Ok(expr) => statement.selection.push(expr),
                Err(err) => return Err(err.to_string()),
            }

            let next_token = self.peek_token();
            if next_token.is_none() {
                return Err("Select was not finished".to_string());
            }

            match next_token.unwrap().token_type {
                TokenType::From | TokenType::SemiColon => break,
                _ => continue,
            }
        }

        // break if there is semicolon
        if self.peek_token().unwrap().token_type == TokenType::SemiColon {
            self.read_token();
            return Ok(Box::new(statement));
        }

        return Ok(Box::new(statement));
    }

    fn parse_expression(&mut self) -> Result<Box<dyn Expression>, String> {
        return match self.read_token() {
            None => Err("Invalid token found".to_string()),
            Some(token) => match token.token_type {
                TokenType::Number => Ok(Box::new(NumberExpr::new(token.literal.to_string()))),
                _ => Err(format!("Not supported expression: {:?}", token.clone())),
            },
        };
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
                assert_eq!(1, ast.statements.len());

                let select_statement = ast.statements.first().unwrap();
                assert_eq!(input, select_statement.to_string());
            }
            Err(err) => {
                assert!(false, "Parsing failed: {}", err)
            }
        }
    }

    #[test]
    fn simple_select_all_from_table() {
        let input = "select * from table_name;";
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
                assert_eq!(1, ast.statements.len());

                let select_statement = ast.statements.first().unwrap();
                assert_eq!(input, select_statement.to_string());
            }
            Err(err) => {
                assert!(false, "Parsing failed: {}", err)
            }
        }
    }
}
