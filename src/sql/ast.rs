pub trait Node {
    fn to_string(&self) -> String;
}

pub trait Statement: Node {
    //
}

pub trait Expression: Node {
    //
}

pub struct SelectStatement {
    pub selection: Vec<Box<dyn Expression>>,
    pub from: Option<Box<dyn Expression>>,
    pub joins: Vec<Box<dyn Expression>>,
    pub group_by: Vec<Box<dyn Expression>>,
    pub order_by: Vec<Box<dyn Expression>>,
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for SelectStatement {
    fn to_string(&self) -> String {
        let selection_str = self
            .selection
            .iter()
            .map(|expr| expr.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let from_str = match &self.from {
            None => "".to_string(),
            Some(from) => format!(" from {}", from.to_string()),
        };

        return format!("select {} {}", selection_str, from_str);
    }
}

impl Statement for SelectStatement {}
