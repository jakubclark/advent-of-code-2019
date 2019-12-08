use intcode_computer::run_program;
use std::fs::read_to_string;

fn part1() {
    let input_string = read_to_string("input.txt").expect("Failed to open input.txt");
    let (_, output) = run_program(input_string, &vec![1]);
    println!("Part 1 Program Output: {:?}", output);
}

fn part2() {
    let input_string = read_to_string("input.txt").expect("Failed to open input.txt");
    let (_, output) = run_program(input_string, &vec![5]);
    println!("Part 2 Program Output: {:?}", output);
}

fn main() {
    part1();
    part2();
}
