mod instruction;
mod machine;
mod value;
use machine::*;

fn main() {
    let program = [];
    let machine = Machine::new(&program);

    println!("Hello, world!");
}
