#![allow(unused_imports)]
use crate::run_program;

#[test]
fn test1() {
    let program = String::from("3,0,4,0,99");
    let (memory, output) = run_program(program, &vec![50]);
    assert_eq!(output, vec![50]);
    assert_eq!(memory, vec![50, 0, 4, 0, 99]);
}

#[test]
fn test2() {
    let program = String::from("1101,100,-1,0,99");
    let (memory, _) = run_program(program, &vec![0]);
    assert_eq!(memory, vec![99, 100, -1, 0, 99]);
}

#[test]
fn test3() {
    let program = String::from("3,9,8,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, &vec![1]);
    assert_eq!(output, vec![0]);
}

// For the following 3 tests:
// "Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:"
// From https://adventofcode.com/2019/day/5#part2
#[test]
fn test4() {
    let program = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
    let (_, output) = run_program(program, &vec![0]);
    assert_eq!(output, vec![0]);
}

#[test]
fn test5() {
    let program = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
    let (_, output) = run_program(program, &vec![50]);
    assert_eq!(output, vec![1]);
}

#[test]
fn test6() {
    let program = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
    let (_, output) = run_program(program, &vec![-50]);
    assert_eq!(output, vec![1]);
}

// The following 6 tests, test if the input is equal to 8
#[test]
fn test_is_equal_to_8_version1() {
    let program = String::from("3,9,8,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, &vec![-50]);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_is_equal_to_8_version2() {
    let program = String::from("3,9,8,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, &vec![8]);
    assert_eq!(output, vec![1]);
}

#[test]
fn test_is_equal_to_8_version3() {
    let program = String::from("3,9,8,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, &vec![50]);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_is_equal_to_8_version4() {
    let program = String::from("3,3,1108,-1,8,3,4,3,99");
    let (_, output) = run_program(program, &vec![-50]);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_is_equal_to_8_version5() {
    let program = String::from("3,3,1108,-1,8,3,4,3,99");
    let (_, output) = run_program(program, &vec![8]);
    assert_eq!(output, vec![1]);
}

#[test]
fn test_is_equal_to_8_version6() {
    let program = String::from("3,3,1108,-1,8,3,4,3,99");
    let (_, output) = run_program(program, &vec![50]);
    assert_eq!(output, vec![0]);
}

// The following 6 tests, test if the input is less than to 8
#[test]
fn test_is_less_than_8_version1() {
    let program = String::from("3,9,7,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, &vec![-50]);
    assert_eq!(output, vec![1]);
}

#[test]
fn test_is_less_than_8_version2() {
    let program = String::from("3,9,7,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, &vec![8]);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_is_less_than_8_version3() {
    let program = String::from("3,9,7,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, &vec![50]);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_is_lass_than_8_version4() {
    let program = String::from("3,3,1107,-1,8,3,4,3,99");
    let (_, output) = run_program(program, &vec![-50]);
    assert_eq!(output, vec![1]);
}

#[test]
fn test_is_lass_than_8_version5() {
    let program = String::from("3,3,1107,-1,8,3,4,3,99");
    let (_, output) = run_program(program, &vec![8]);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_is_lass_than_8_version6() {
    let program = String::from("3,3,1107,-1,8,3,4,3,99");
    let (_, output) = run_program(program, &vec![50]);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_complex1() {
    let program = String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
    let (_, output) = run_program(program, &vec![-50]);
    assert_eq!(output, vec![999]);
}

#[test]
fn test_complex2() {
    let program = String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
    let (_, output) = run_program(program, &vec![8]);
    assert_eq!(output, vec![1000]);
}

#[test]
fn test_complex3() {
    let program = String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
    let (_, output) = run_program(program, &vec![50]);
    assert!(output[0] > 1000);
}
