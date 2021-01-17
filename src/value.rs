use super::instruction::Instruction;
use derive_more::Display;
use std::fmt::{self, Display};
use std::rc::Rc;

#[derive(Clone, Display)]
pub enum Value {
    Int(i32),
    List(List),
    Instruction(Instruction),
}

#[derive(Clone)]
pub enum List {
    Cons { value: Rc<Value>, next: Rc<List> },
    None,
}

impl Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            List::Cons { value, next } => {
                write!(f, "[{}{}]", value, list_to_string(next))
            }
            List::None => write!(f, "[]"),
        }
    }
}

fn list_to_string(this: &List) -> String {
    match this {
        List::Cons { value, next } => {
            format!(" {}{}", value, list_to_string(next))
        }
        List::None => String::from(""),
    }
}
