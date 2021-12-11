use rum::{load, machine};
use std::env;

fn main() {
    let input = env::args().nth(1);
    let instructions = load::load(input.as_deref());
    machine::machine(instructions);
}
