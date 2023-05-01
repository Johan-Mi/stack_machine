use crate::value::Value;
use std::fmt::{self, Display};

#[derive(Clone)]
pub enum Instruction {
    Push(Value),
    Nop,
    Pop,
    Dup,
    Print,
    Add,
    Sub,
    Exec,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Push(val) => val.fmt(f),
            Self::Nop => f.write_str("nop"),
            Self::Pop => f.write_str("pop"),
            Self::Dup => f.write_str("dup"),
            Self::Print => f.write_str("print"),
            Self::Add => f.write_str("add"),
            Self::Sub => f.write_str("sub"),
            Self::Exec => f.write_str("exec"),
        }
    }
}
