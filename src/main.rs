mod instruction;
mod machine;
mod value;
use instruction::Instruction;
use machine::Machine;
use value::Value;

fn main() {
    use Instruction::*;
    let program = [
        Nop,
        Push {
            value: Value::Int(40),
        },
        Push {
            value: Value::Int(2),
        },
        Dup,
        Pop,
        Add,
        Print,
    ];

    let mut machine = Machine::new(&program);

    if let Err(err) = machine.run() {
        eprintln!("Error: {}", err);
    }
}
