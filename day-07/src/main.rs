use intcode_computer::run_program;
use std::fs::read_to_string;

fn run_permutation(program: &str, inputs: &[i64; 5]) -> i64 {
    let program = String::from(program);

    let mut prev = 0;
    let mut prev_output = vec![];
    for i in inputs {
        let (_, output) = run_program(program.clone(), &[*i, prev]);
        prev = output[0];
        prev_output = output;
    }
    *prev_output
        .get(0)
        .unwrap_or_else(|| panic!("No result was returned by the program"))
}

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

fn main() {
    let program = read_to_string("input.txt").expect("Failed to read input.txt");
    let mut seq = [0, 1, 2, 3, 4];
    let max = get_max_output(&program, &mut seq);
    println!("max_output = {:?}", max);
}

#[cfg(test)]
mod tests {
    use crate::get_max_output;

    #[test]
    fn test1() {
        let program = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let max = get_max_output(&program, &mut [4, 3, 2, 1, 0]);
        assert_eq!(max, 43210);
    }

    #[test]
    fn test2() {
        let program = String::from(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        let max = get_max_output(&program, &mut [0, 1, 2, 3, 4]);
        assert_eq!(max, 54312);
    }

    #[test]
    fn test3() {
        let program = String::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        let max = get_max_output(&program, &mut [1, 0, 4, 3, 2]);
        assert_eq!(max, 65210);
    }
}
