use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Opcode {
    Select,
    From,
    Where,
    OrderBy,

    Insert,
    Update,
    Delete,
}

pub struct Definition {
    pub name: String,
    pub operand_widths: Vec<i16>,
}

lazy_static! {
    static ref DEFINITIONS: HashMap<Opcode, Definition> = {
        let mut m = HashMap::new();
        m.insert(
            Opcode::Select,
            Definition {
                name: String::from("select"),
                operand_widths: vec![2],
            },
        );
        m
    };
}

pub struct Lexer {}

pub struct Parser {}
