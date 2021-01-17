use derive_more::Display;

#[derive(Clone, Display)]
pub enum Instruction {
    Nop,
    Pop,
    Dup,
    Print,
    Add,
    Sub,
}
