use crate::instruction::Instruction;
use derive_more::Display;
use itertools::Itertools;

#[derive(Clone, Display)]
pub enum Value {
    Int(i32),
    #[display(fmt = "[{}]", "_0.iter().format(\" \")")]
    List(Vec<Value>),
    Instruction(Instruction),
}
