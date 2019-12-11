use intcode_computer::run_program;
use std::fs::read_to_string;

fn main() {
    let program = read_to_string("input.txt").expect("Failed to open input.txt");
    let (_, output1) = run_program(program.clone(), &[1]);
    println!("Solution for part 1 = {:?}", output1[0]);
    let (_, output2) = run_program(program, &[2]);
    println!("Solution for part 2 = {:?}", output2[0]);
}
