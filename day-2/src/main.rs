use std::fs::read_to_string;
use std::io::Error as IoError;

const ERR_MSG: &str = "No entry found";

fn string_to_vec(input: String) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|opcode| opcode.parse().expect("Not a number"))
        .collect()
}

fn get_parts(i: usize, nums: &[i64]) -> (i64, i64, usize) {
    let left_index = nums.get(i + 1).expect(ERR_MSG);
    let right_index = nums.get(i + 2).expect(ERR_MSG);

    let left = nums.get(*left_index as usize).expect(ERR_MSG);
    let right = nums.get(*right_index as usize).expect(ERR_MSG);

    let dest = nums.get(i + 3).expect(ERR_MSG);

    (*left, *right, *dest as usize)
}

fn run_program(mut nums: Vec<i64>) -> Vec<i64> {
    let mut i = 0;

    while let Some(opcode) = nums.get(i) {
        if *opcode == 99 {
            break;
        }

        let (left, right, dest) = get_parts(i, &nums);

        let res = match *opcode {
            1 => left + right,
            2 => left * right,
            _ => unreachable!(),
        };
        let entry = nums.get_mut(dest).expect(ERR_MSG);
        *entry = res;
        i += 4;
    }
    nums
}

fn part1() -> Result<(), IoError> {
    let input_string = read_to_string("input.txt")?;
    let mut nums = string_to_vec(input_string);

    let second = nums.get_mut(1).expect(ERR_MSG);
    *second = 12;
    let third = nums.get_mut(2).expect(ERR_MSG);
    *third = 2;

    let res = run_program(nums);
    println!("{:?}", res.get(0).unwrap());
    Ok(())
}

fn main() -> Result<(), IoError> {
    part1()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{run_program, string_to_vec};

    #[test]
    fn test1() {
        let expected = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let nums = string_to_vec(String::from("1,9,10,3,2,3,11,0,99,30,40,50"));
        assert_eq!(nums, expected);
    }

    #[test]
    fn test2() {
        let expected = vec![2, 0, 0, 0, 99];
        let nums = string_to_vec(String::from("1,0,0,0,99"));
        let result = run_program(nums);
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let expected = vec![2, 3, 0, 6, 99];
        let nums = string_to_vec(String::from("2,3,0,3,99"));
        let result = run_program(nums);
        assert_eq!(result, expected);
    }

    #[test]
    fn test4() {
        let expected = vec![2, 4, 4, 5, 99, 9801];
        let nums = string_to_vec(String::from("2,4,4,5,99,0"));
        let result = run_program(nums);
        assert_eq!(result, expected);
    }

    #[test]
    fn test5() {
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let nums = string_to_vec(String::from("1,1,1,4,99,5,6,0,99"));
        let result = run_program(nums);
        assert_eq!(result, expected);
    }
}
