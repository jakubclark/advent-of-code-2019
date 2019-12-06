use std::fs::read_to_string;

const ERR_MSG: &str = "No entry found";
const EXPECTED_FIRST: i64 = 19690720;
const DELTA: i64 = 331776;

fn string_to_vec(input: String) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|opcode| opcode.parse().expect("Not a number"))
        .collect()
}

fn get_parts(i: usize, nums: &[i64]) -> Result<(i64, i64, usize), ()> {
    let left_index = nums.get(i + 1).ok_or(())?;
    let right_index = nums.get(i + 2).ok_or(())?;

    let left = nums.get(*left_index as usize).ok_or(())?;
    let right = nums.get(*right_index as usize).ok_or(())?;

    let dest = nums.get(i + 3).ok_or(())?;

    Ok((*left, *right, *dest as usize))
}

fn run_program(mut nums: Vec<i64>) -> Result<Vec<i64>, ()> {
    let mut i = 0;

    while let Some(opcode) = nums.get(i) {
        if *opcode == 99 {
            break;
        }

        let (left, right, dest) = get_parts(i, &nums)?;

        let res = match *opcode {
            1 => left + right,
            2 => left * right,
            _ => unreachable!(),
        };
        let entry = nums.get_mut(dest).expect(ERR_MSG);
        *entry = res;
        i += 4;
    }
    Ok(nums)
}

fn run_parameterized_program(mut nums: Vec<i64>, noun: i64, verb: i64) -> Result<Vec<i64>, ()> {
    let second = nums.get_mut(1).expect("");
    *second = noun;
    let third = nums.get_mut(2).expect("");
    *third = verb;
    run_program(nums)
}

fn part1() {
    let input_string = read_to_string("input.txt").expect("Failed to open input.txt");
    let nums = string_to_vec(input_string);
    let res = run_parameterized_program(nums, 12, 2).expect("invalid program input");
    let first = res.get(0).unwrap();
    println!("Solution for part 1 = {}", first);
}

fn compute_noun(nums: &Vec<i64>) -> Result<i64, ()> {
    for noun in 0..100 {
        let clone = nums.clone();

        if let Ok(res) = run_parameterized_program(clone, noun, 0) {
            let first_entry = *res.get(0).unwrap();
            let diff = EXPECTED_FIRST - first_entry;
            if diff.abs() < DELTA {
                // We are close enough to the result here
                return Ok(noun);
            }
        }
    }
    Err(())
}

fn compute_verb(nums: &Vec<i64>, noun: i64) -> Result<i64, ()> {
    // In theory, this should be 0..DELTA, but 100 works in this case.
    for verb in 0..100 {
        let clone = nums.clone();

        if let Ok(res) = run_parameterized_program(clone, noun, verb) {
            let first_entry = *res.get(0).unwrap();
            let diff = EXPECTED_FIRST - first_entry;
            if diff == 0 {
                return Ok(verb);
            }
        }
    }
    Err(())
}

fn part2() {
    let input_string = read_to_string("input.txt").expect("Failed to open input.txt");
    let nums = string_to_vec(input_string);

    // Increasing noun, increases result by 331776
    // Increasing verb, increases result by 1
    // Increasing both, increases result by 331777
    // Expected output                      19690720
    // Just brute force search :)

    let noun = compute_noun(&nums).expect("No solution for noun");
    let verb = compute_verb(&nums, noun).expect("No solution for verb");

    println!("Solution for part 2 = {}", 100 * noun + verb);
}

fn main() -> Result<(), ()> {
    part1();
    part2();
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{run_program, string_to_vec, compute_noun, compute_verb, run_parameterized_program};
    use std::fs::read_to_string;

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
        let result = run_program(nums).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let expected = vec![2, 3, 0, 6, 99];
        let nums = string_to_vec(String::from("2,3,0,3,99"));
        let result = run_program(nums).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test4() {
        let expected = vec![2, 4, 4, 5, 99, 9801];
        let nums = string_to_vec(String::from("2,4,4,5,99,0"));
        let result = run_program(nums).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test5() {
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        let nums = string_to_vec(String::from("1,1,1,4,99,5,6,0,99"));
        let result = run_program(nums).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part1() {
        let input_string = read_to_string("input.txt").expect("Failed to open input.txt");
        let nums = string_to_vec(input_string);
        let res = run_parameterized_program(nums, 12, 2).expect("invalid program input");
        let first = res.get(0).unwrap();
        assert_eq!(*first, 6087827, "first_entry is not correct")
    }

    #[test]
    fn test_part2() {
        let input_string = read_to_string("input.txt").expect("Failed to open input.txt");
        let nums = string_to_vec(input_string);
        let noun = compute_noun(&nums).expect("No solution for noun");
        let verb = compute_verb(&nums, noun).expect("No solution for verb");
        assert_eq!(noun, 53, "noun is not correct");
        assert_eq!(verb, 79, "verb is not correct");
    }
}
