use crate::{instruction::Instruction, value::Value};
use itertools::Itertools;
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
    I: Iterator<Item = Value>,
{
    program: I,
    stack: Vec<Value>,
}

impl<I> Machine<I>
where
    I: Iterator<Item = Value>,
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
        self.exec_value(&v).map(ControlFlow::Continue)
    }

    pub fn exec_value(&mut self, value: &Value) -> Result<(), Error> {
        println!("Running: {value}");
        let ret = match value {
            Value::Instruction(inst) => self.exec_instruction(*inst),
            v => {
                self.push(v.clone());
                Ok(())
            }
        };
        println!("Stack:   [{}]", self.stack.iter().format(" "));
        ret
    }

    pub fn exec_instruction(&mut self, inst: Instruction) -> Result<(), Error> {
        match inst {
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
                        self.exec_value(&i)?;
                    }
                    Ok(())
                } else {
                    todo!();
                }
            }
        }
    }
}
