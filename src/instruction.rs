use super::value::Value;

pub enum Instruction {
    Nop,
    Pop,
    Push { value: Value },
    Dup,
    Print,
}
