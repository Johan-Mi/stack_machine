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
        use Instruction::*;
        write!(
            f,
            "{}",
            match self {
                Nop => "nop",
                Pop => "pop",
                Dup => "dup",
                Print => "print",
                Add => "add",
                Sub => "sub",
                Exec => "exec",
            }
        )
    }
}
