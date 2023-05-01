use crate::{instruction::Instruction, value::Value};
use std::ops::ControlFlow;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("tried to pop from empty stack")]
    PopEmptyStack,
    #[error("tried to get top of empty stack")]
    TopEmptyStack,
}

pub struct Machine<I>
where
    I: Iterator<Item = Instruction>,
{
    program: I,
    stack: Vec<Value>,
}

impl<I> Machine<I>
where
    I: Iterator<Item = Instruction>,
{
    pub const fn new(program: I) -> Self {
        Self {
            program,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        while self.step()?.is_continue() {}
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

    pub fn step(&mut self) -> Result<ControlFlow<()>, Error> {
        let Some(v) = self.program.next() else {
            return Ok(ControlFlow::Break(()))
        };
        self.exec_instruction(v).map(ControlFlow::Continue)
    }

    pub fn exec_instruction(&mut self, inst: Instruction) -> Result<(), Error> {
        match inst {
            Instruction::Push(val) => {
                self.push(val);
                Ok(())
            }
            Instruction::Nop => Ok(()),
            Instruction::Pop => {
                self.pop()?;
                Ok(())
            }
            Instruction::Dup => {
                let top = self.top()?.clone();
                self.push(top);
                Ok(())
            }
            Instruction::Print => {
                let value = self.pop()?;
                println!("{value}");
                Ok(())
            }
            Instruction::Add => {
                let rhs = self.pop()?;
                let lhs = self.pop()?;
                match (lhs, rhs) {
                    (Value::Int(lhs), Value::Int(rhs)) => {
                        self.push(Value::Int(lhs + rhs));
                        Ok(())
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
                        Ok(())
                    }
                    _ => todo!(),
                }
            }
            Instruction::Exec => {
                let list = self.pop()?;
                if let Value::List(list) = list {
                    for i in list {
                        self.exec_instruction(i)?;
                    }
                    Ok(())
                } else {
                    todo!();
                }
            }
        }
    }
}
