pub struct Node {
    pub token: String,
}

pub trait Expression {
    fn get_sub_nodes(&mut self) -> Vec<Node>;
    fn get_sub_expressions(&mut self) -> Vec<Box<dyn Expression>>;
}

pub trait Statement {
    fn get_sub_expressions(&mut self) -> Vec<Box<dyn Expression>>;
}

pub struct SelectStatement {
}

impl Statement for SelectStatement {
    fn get_sub_expressions(&mut self) -> Vec<Box<dyn Expression>> {
        todo!()
    }
}

pub struct Ast {
    pub nodes: Vec<Box<dyn Statement>>,
}