use super::instruction::Instruction;
use super::value::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MachineError {
    #[error("program counter has reached the end of the program")]
    EndOfProgram,
    #[error("tried to pop from empty stack")]
    PopEmptyStack,
    #[error("tried to get top of empty stack")]
    TopEmptyStack,
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

    pub fn run(&mut self) -> Result<(), MachineError> {
        loop {
            match self.step() {
                Ok(()) => (),
                Err(MachineError::EndOfProgram) => break Ok(()),
                Err(err) => break Err(err),
            }
        }
    }

    fn pop(&mut self) -> Result<Value, MachineError> {
        self.stack.pop().ok_or(MachineError::PopEmptyStack)
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn top(&self) -> Result<&Value, MachineError> {
        self.stack.last().ok_or(MachineError::TopEmptyStack)
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
            Some(Instruction::Push { value }) => {
                self.push(value.clone());
                self.pc += 1;
                Ok(())
            }
            Some(Instruction::Dup) => {
                self.pc += 1;
                let top = self.top()?.clone();
                self.push(top);
                Ok(())
            }
            Some(Instruction::Print) => {
                self.pc += 1;
                let value = self.pop()?;
                println!("{}", value);
                Ok(())
            }
            Some(Instruction::Add) => {
                self.pc += 1;
                let rhs = self.pop()?;
                let lhs = self.pop()?;
                match (lhs, rhs) {
                    (Value::Int(lhs), Value::Int(rhs)) => {
                        self.push(Value::Int(lhs + rhs));
                        Ok(())
                    }
                }
            }
        }
    }
}
