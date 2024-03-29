#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]

mod instruction;
mod machine;
mod value;

use machine::Machine;

fn main() -> Result<(), machine::Error> {
    use instruction::Instruction as I;
    use value::Value as V;
    let program = vec![
        I::Nop,
        I::Push(V::Int(40)),
        I::Push(V::Int(4)),
        I::Push(V::Int(2)),
        I::Push(V::List(vec![I::Dup, I::Pop, I::Sub, I::Add])),
        I::Exec,
        I::Print,
    ];

    let mut machine = Machine::new();

    machine.run(program.into_iter())
}
