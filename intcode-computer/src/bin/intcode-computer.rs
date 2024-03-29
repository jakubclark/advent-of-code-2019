use intcode_computer::run_program;
use std::fs::read_to_string;

fn main() {
    let program = read_to_string("input2.txt").expect("Failed to open file");
    let (_, output) = run_program(program, 15);
    println!("{:?}", output);
}
