use crate::{instruction::Instruction, value::Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("tried to pop from empty stack")]
    PopEmptyStack,
    #[error("tried to get top of empty stack")]
    TopEmptyStack,
}

pub struct Machine {
    stack: Vec<Value>,
}

impl Machine {
    pub const fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn run(
        &mut self,
        program: impl Iterator<Item = Instruction>,
    ) -> Result<(), Error> {
        for inst in program {
            self.exec_instruction(inst)?;
        }
        Ok(())
    }

    fn pop(&mut self) -> Result<Value, Error> {
        self.stack.pop().ok_or(Error::PopEmptyStack)
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn top(&self) -> Result<&Value, Error> {
        self.stack.last().ok_or(Error::TopEmptyStack)
    }

    fn exec_instruction(&mut self, inst: Instruction) -> Result<(), Error> {
        match inst {
            Instruction::Push(val) => self.push(val),
            Instruction::Nop => {}
            Instruction::Pop => {
                self.pop()?;
            }
            Instruction::Dup => {
                let top = self.top()?.clone();
                self.push(top);
            }
            Instruction::Print => {
                let value = self.pop()?;
                println!("{value}");
            }
            Instruction::Add => {
                let rhs = self.pop()?;
                let lhs = self.pop()?;
                match (lhs, rhs) {
                    (Value::Int(lhs), Value::Int(rhs)) => {
                        self.push(Value::Int(lhs + rhs));
                    }
                    _ => todo!(),
                }
            }
            Instruction::Sub => {
                let rhs = self.pop()?;
                let lhs = self.pop()?;
                match (lhs, rhs) {
                    (Value::Int(lhs), Value::Int(rhs)) => {
                        self.push(Value::Int(lhs - rhs));
                    }
                    _ => todo!(),
                }
            }
            Instruction::Exec => {
                let Value::List(list) = self.pop()? else { todo!() };
                for i in list {
                    self.exec_instruction(i)?;
                }
            }
        };
        Ok(())
    }
}
