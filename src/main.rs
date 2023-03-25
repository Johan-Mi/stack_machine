mod instruction;
mod list;
mod machine;
mod value;
use instruction::Instruction::*;
use machine::Machine;
use value::Value::*;

fn main() {
    let program = vec![
        Instruction(Nop),
        Int(40),
        Int(4),
        Int(2),
        List(make_list![
            Instruction(Dup),
            Instruction(Pop),
            Instruction(Sub),
            Instruction(Add)
        ]),
        Instruction(Exec),
        Instruction(Print),
    ];

    let mut machine = Machine::new(program.into_iter());

    if let Err(err) = machine.run() {
        eprintln!("Error: {err}");
    }
}
