use super::instruction::Instruction;
use super::list::List;
use derive_more::Display;
use std::rc::Rc;

#[derive(Clone, Display)]
pub enum Value {
    Int(i32),
    List(List),
    Instruction(Instruction),
}
