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
            value: Value::Int(42),
        },
        Dup,
        Pop,
        Print,
    ];

    let mut machine = Machine::new(&program);

    loop {
        if let Err(err) = machine.step() {
            eprintln!("Error: {}", err);
            break;
        }
    }
}
