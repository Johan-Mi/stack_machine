use std::fmt::{self, Display};

#[derive(Clone, Copy)]
pub enum Instruction {
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
        f.write_str(match self {
            Self::Nop => "nop",
            Self::Pop => "pop",
            Self::Dup => "dup",
            Self::Print => "print",
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Exec => "exec",
        })
    }
}
