use crate::instruction::Instruction;
use itertools::Itertools;
use std::fmt::{self, Display};

#[derive(Clone)]
pub enum Value {
    Int(i32),
    List(Vec<Instruction>),
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => i.fmt(f),
            Self::List(list) => write!(f, "[{}]", list.iter().format(" ")),
        }
    }
}
