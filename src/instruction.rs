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
            Instruction::Nop => "nop",
            Instruction::Pop => "pop",
            Instruction::Dup => "dup",
            Instruction::Print => "print",
            Instruction::Add => "add",
            Instruction::Sub => "sub",
            Instruction::Exec => "exec",
        })
    }
}
