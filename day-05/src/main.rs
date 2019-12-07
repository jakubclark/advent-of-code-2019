use std::fs::read_to_string;

mod part1;

use crate::part1::run_program;

fn main() {
    let input_string = read_to_string("input.txt").expect("Failed to open input.txt");
    let (_, output) = run_program(input_string, 1);
    println!("Part 1 Program Result: {:?}", output);
}
