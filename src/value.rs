use crate::{instruction::Instruction, list::List};
use derive_more::Display;

#[derive(Clone, Display)]
pub enum Value {
    Int(i32),
    List(List),
    Instruction(Instruction),
}
