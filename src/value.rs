use super::instruction::Instruction;
use super::list::List;
use derive_more::Display;

#[derive(Clone, Display)]
pub enum Value {
    Int(i32),
    List(Box<List>),
    Instruction(Instruction),
}
