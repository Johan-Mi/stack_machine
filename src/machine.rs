use super::instruction::Instruction;
use super::value::Value;

pub struct Machine<'a> {
    program: &'a [Instruction],
    stack: Vec<Value>,
    pc: usize,
}

impl<'a> Machine<'a> {
    pub fn new(program: &'a [Instruction]) -> Self {
        Self {
            program,
            stack: Vec::new(),
            pc: 0,
        }
    }

    pub fn step(&mut self) -> Result<(), ()> {
        match self.program.get(self.pc) {
            None => Err(()),
            _ => todo!(),
        }
    }
}
