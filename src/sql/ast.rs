use std::slice::Iter;

pub trait Node {
    fn to_string(&self) -> String;
}

pub trait Statement: Node {
    fn get_sub_expressions(&mut self) -> Iter<Box<dyn Expression>>;
}

pub trait Expression: Node {
    fn get_sub_expressions(&mut self) -> Iter<Box<dyn Expression>>;
}

pub struct SelectStatement {
    pub selection: Vec<Box<dyn Expression>>,
    pub from: Option<Box<dyn Expression>>,
    pub joins: Vec<Box<dyn Expression>>,
    pub group_by: Vec<Box<dyn Expression>>,
    pub order_by: Vec<Box<dyn Expression>>,
}

pub enum NumberType {
    Int,
    Double,
}

pub struct NumberExpr {
    pub number_type: NumberType,
    pub value: f64,
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

        return format!("select {}{};", selection_str, from_str);
    }
}

impl Statement for SelectStatement {
    fn get_sub_expressions(&mut self) -> Iter<Box<dyn Expression>> {
        todo!()
    }
}

impl SelectStatement {
    pub fn new() -> Self {
        return Self {
            selection: vec![],
            from: None,
            joins: vec![],
            group_by: vec![],
            order_by: vec![],
        };
    }
}

impl Node for NumberExpr {
    fn to_string(&self) -> String {
        return match self.number_type {
            NumberType::Int => format!("{:.0}", self.value),
            NumberType::Double => format!("{}", self.value),
        };
    }
}

impl Expression for NumberExpr {
    fn get_sub_expressions(&mut self) -> Iter<Box<dyn Expression>> {
        todo!()
    }
}

impl NumberExpr {
    pub fn new(input: String) -> NumberExpr {
        return match input.contains(".") {
            true => NumberExpr::new_double(input.parse().unwrap()),
            false => NumberExpr::new_int(input.parse().unwrap()),
        };
    }

    pub fn new_int(i: i32) -> NumberExpr {
        return NumberExpr {
            number_type: NumberType::Int,
            value: i.into(),
        };
    }

    pub fn new_double(i: f64) -> NumberExpr {
        return NumberExpr {
            number_type: NumberType::Double,
            value: i,
        };
    }
}

pub struct Ast {
    pub statements: Vec<Box<dyn Statement>>,
}
