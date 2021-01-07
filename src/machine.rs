use super::instruction::Instruction;
use super::value::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MachineError {
    #[error("program counter has reached the end of the program")]
    EndOfProgram,
    #[error("tried to pop from empty stack")]
    PopEmptyStack,
}

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

    fn pop(&mut self) -> Result<Value, MachineError> {
        self.stack.pop().ok_or(MachineError::PopEmptyStack)
    }

    pub fn step(&mut self) -> Result<(), MachineError> {
        match self.program.get(self.pc) {
            None => Err(MachineError::EndOfProgram),
            Some(Instruction::Nop) => {
                self.pc += 1;
                Ok(())
            }
            Some(Instruction::Pop) => {
                self.pc += 1;
                self.pop()?;
                Ok(())
            }
        }
    }
}
