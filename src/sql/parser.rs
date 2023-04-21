use crate::sql::ast::{Program, SelectStatement, Statement};
use crate::sql::lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        return Self { lexer: l };
    }

    pub fn parse_program(&mut self) -> Program {
        return Program {
            statements: vec![self.parse_statement()],
        };
    }

    fn parse_statement(&self) -> Box<impl Statement> {
        return Box::new(SelectStatement {
            selection: vec![],
            from: None,
            joins: vec![],
            group_by: vec![],
            order_by: vec![],
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::sql::ast::SelectStatement;
    use crate::sql::lexer::Lexer;
    use crate::sql::parser::Parser;

    #[test]
    fn simple_select_of_number() {
        let input = "select foo from my_table;";
        let lexer = Lexer::new(input.to_string());

        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let statements = program.statements;

        assert_eq!(1, statements.len(), "Wrong number of statements");
        let statement = &statements[0];
        assert_eq!(input, statement.to_string());
    }
}
