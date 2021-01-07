mod instruction;
mod machine;
mod value;
use instruction::Instruction;
use machine::Machine;

fn main() {
    use Instruction::*;
    let program = [Nop, Pop];

    let mut machine = Machine::new(&program);

    loop {
        if let Err(err) = machine.step() {
            eprintln!("Error: {}", err);
            break;
        }
    }
}
