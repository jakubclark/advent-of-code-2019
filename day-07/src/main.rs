use intcode_computer::{run_program, Machine};
use std::fs::read_to_string;

// Based on https://www.nayuki.io/res/next-lexicographical-permutation-algorithm/nextperm.rs
fn permute(seq: &mut [i64; 5]) -> bool {
    if seq.is_empty() {
        return false;
    }
    let mut i = seq.len() - 1;
    while i > 0 && seq[i - 1] >= seq[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }
    let mut j = seq.len() - 1;
    while seq[j] <= seq[i - 1] {
        j -= 1;
    }
    seq.swap(i - 1, j);
    seq[i..].reverse();
    true
}

fn run_permutation(program: &str, inputs: &[i64; 5]) -> i64 {
    let program = String::from(program);

    let mut prev = 0;
    let mut prev_output = vec![];
    for i in inputs {
        let (_, output) = run_program(program.clone(), prev);
        prev = output[0];
        prev_output = output;
    }
    *prev_output
        .get(0)
        .unwrap_or_else(|| panic!("No result was returned by the program"))
}

fn get_max_output(program: &str, mut sequence: &mut [i64; 5]) -> i64 {
    sequence.sort();
    let mut max = i64::min_value();
    while permute(&mut sequence) {
        let res = run_permutation(&program, &sequence);
        if res > max {
            max = res;
        }
    }
    max
}

fn run_permutation_continuous(program: &str, input: &[i64; 5]) -> i64 {
    let mut machines: Vec<_> = input
        .iter()
        .map(|&n| Machine::new(String::from(program), n))
        .collect();

    let mut input = 0;
    while machines[4].is_running() {
        machines.iter_mut().for_each(|machine| {
            machine.push_input(input);
            machine.step();
            input = machine.get_result();
        })
    }

    machines[4].get_result()
}

fn get_max_output_continuous(program: &str, mut sequence: &mut [i64; 5]) -> i64 {
    sequence.sort();
    let mut max = i64::min_value();
    while permute(&mut sequence) {
        let res = run_permutation_continuous(program, &sequence);
        if res > max {
            max = res
        }
    }
    max
}

fn part1(program: &str) {
    let max = get_max_output(program, &mut [0, 1, 2, 3, 4]);
    println!("Solution for part 1 = {}", max);
}

fn part2(program: &str) {
    let max = get_max_output_continuous(program, &mut [5, 6, 7, 8, 9]);
    println!("Solution for part 2 = {}", max);
}

fn main() {
    let program = read_to_string("input.txt").expect("Failed to read input.txt");
    part1(&program);
    part2(&program);
}

//#[cfg(test)]
//mod tests {
//    use crate::{get_max_output, get_max_output_continuous};
//
//    #[test]
//    fn test1() {
//        let program = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
//        let max = get_max_output(&program, &mut [4, 3, 2, 1, 0]);
//        assert_eq!(max, 43210);
//    }
//
//    #[test]
//    fn test2() {
//        let program = String::from(
//            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
//        );
//        let max = get_max_output(&program, &mut [0, 1, 2, 3, 4]);
//        assert_eq!(max, 54312);
//    }
//
//    #[test]
//    fn test3() {
//        let program = String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
//        let max = get_max_output(&program, &mut [1, 0, 4, 3, 2]);
//        assert_eq!(max, 65210);
//    }
//
//    #[test]
//    fn test4() {
//        let program = String::from(
//            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
//        );
//        let max = get_max_output_continuous(&program, &mut [9, 8, 7, 6, 5]);
//        assert_eq!(max, 139629729);
//    }
//
//    #[test]
//    fn test5() {
//        let program = String::from("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
//        let max = get_max_output_continuous(&program, &mut [9, 7, 8, 5, 6]);
//        assert_eq!(max, 18216);
//    }
//}
