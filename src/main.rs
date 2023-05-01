#![forbid(unsafe_code)]

mod instruction;
mod machine;
mod value;
use instruction::Instruction::*;
use machine::Machine;
use value::Value::*;

fn main() {
    let program = vec![
        Nop,
        Push(Int(40)),
        Push(Int(4)),
        Push(Int(2)),
        Push(List(vec![Dup, Pop, Sub, Add])),
        Exec,
        Print,
    ];

    let mut machine = Machine::new(program.into_iter());

    if let Err(err) = machine.run() {
        eprintln!("Error: {err}");
    }
}
