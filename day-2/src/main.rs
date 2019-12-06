use std::fs::read_to_string;

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

fn part1() -> Result<(), ()> {
    let input_string = read_to_string("input.txt").expect("");
    let nums = string_to_vec(input_string);
    let res = run_parameterized_program(nums, 12, 2)?;
    println!("{:?}", res);
    Ok(())
}

fn part2() -> Result<(), ()> {
    let input_string = read_to_string("input.txt").expect("");
    let nums = string_to_vec(input_string);

    // Increasing noun, increased result by 331776
    // Increasing verb, increased result by 1
    // Increasing both, increased result by 331777
    // Expected output                      19690720
    // Find the noun and verb using pseudo binary search

    let expected_first_entry: i64 = 19690720;

    let candidate_nouns: Vec<i64> = (0..100).collect();
    let candidate_verbs: Vec<i64> = (0..100).collect();

    let mut noun_i: i64 = 50;
    let mut verb_i: i64 = 0;

    let mut first_entry;
    let mut diff = i64::max_value();

//    let left = 0;
//    let right = 99;
//    let mid = left + (right + 1) / 2;

    while diff.abs() > 331776 {
        let clone = nums.clone();
        println!("noun_i={}", noun_i);
        if diff > 0 {
            // ceil because it turned out to provide the correct solution
            noun_i += ((100 - noun_i) as f64 / 2.0).ceil() as i64;
        } else {
            noun_i -= ((100 - noun_i) as f64 / 2.0).ceil() as i64;
        }

        let noun = *candidate_nouns.get(noun_i as usize).unwrap();

        if let Ok(res) = run_parameterized_program(clone, noun, 0) {
            first_entry = *res.get(0).unwrap();
            diff = expected_first_entry - first_entry;
        }
    }

    // We got the correct noun value
    let noun = *candidate_nouns.get(noun_i as usize).unwrap();

    while verb_i < 100 {
        let verb = *candidate_verbs.get(verb_i as usize).unwrap();

        let clone = nums.clone();

        if let Ok(res) = run_parameterized_program(clone, noun, verb) {
            first_entry = *res.get(0).unwrap();
            diff = expected_first_entry - first_entry;
            if diff == 0 {
                break;
            }
        }

        verb_i += 1;
    }

    println!("noun_i={}", noun_i);
    println!("verb_i={}", verb_i);

    Ok(())
}

fn main() -> Result<(), ()> {
    part1()?;
    part2()?;
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
}
