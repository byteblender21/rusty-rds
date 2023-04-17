use std::slice::Iter;

pub struct Node {
    pub token: String,
}

pub trait Expression {
    fn get_sub_nodes(&mut self) -> Iter<Node>;
    fn get_sub_expressions(&mut self) -> Iter<Box<dyn Expression>>;
}

pub trait Statement {
    fn get_sub_expressions(&mut self) -> Iter<Box<dyn Expression>>;
}

pub struct SelectStatement {
    pub selection_expressions: Vec<Box<dyn Expression>>,
    pub from_expression: Option<Box<dyn Expression>>,
    pub join_expressions: Vec<Box<dyn Expression>>,
}

impl SelectStatement {
    pub fn new() -> SelectStatement {
        return SelectStatement {
            selection_expressions: vec![],
            from_expression: None,
            join_expressions: vec![],
        }
    }
}

impl Statement for SelectStatement {
    fn get_sub_expressions(&mut self) -> Iter<Box<dyn Expression>> {
        return self.selection_expressions.iter()
    }
}

pub struct Ast {
    pub nodes: Vec<Box<dyn Statement>>,
}