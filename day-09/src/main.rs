use intcode_computer::run_program;
use std::fs::read_to_string;

fn main() {
    let program = read_to_string("input.txt").expect("Failed to open input.txt");
    let (_, output) = run_program(program, &[1]);
    println!("{:?}", output);
}
