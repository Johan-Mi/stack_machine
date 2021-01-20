use super::instruction::Instruction;
use super::value::Value;
use itertools::Itertools;
use std::borrow::Cow;
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
    pub fn new(program: I) -> Self {
        Self {
            program,
            stack: Vec::new(),
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
        let v = self.program.next().ok_or(MachineError::EndOfProgram)?;
        self.exec_value(&v)
    }

    pub fn exec_value(&mut self, value: &Value) -> Result<(), MachineError> {
        println!("Running: {}", value);
        let ret = match value {
            Value::Instruction(inst) => self.exec_instruction(&inst),
            v => {
                self.push(v.clone());
                Ok(())
            }
        };
        println!(
            "Stack:   [{}]",
            self.stack
                .iter()
                .map(|v| Cow::Owned(v.to_string()))
                .intersperse(Cow::Borrowed(" "))
                .collect::<String>()
        );
        ret
    }

    pub fn exec_instruction(
        &mut self,
        inst: &Instruction,
    ) -> Result<(), MachineError> {
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
                println!("{}", value);
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
                    for i in list.iter() {
                        self.exec_value(i)?;
                    }
                    Ok(())
                } else {
                    todo!();
                }
            }
        }
    }
}
