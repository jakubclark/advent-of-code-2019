#![allow(unused_imports)]
#![allow(dead_code)]

use crate::run_program;

#[test]
fn test1() {
    let program = String::from("3,0,4,0,99");
    let (memory, output) = run_program(program, 50);
    assert_eq!(output, vec![50]);
    assert_eq!(memory[0..5], [50, 0, 4, 0, 99]);
}

#[test]
fn test2() {
    let program = String::from("1101,100,-1,0,99");
    let (memory, _) = run_program(program, 0);
    assert_eq!(memory[0..5], [99, 100, -1, 0, 99]);
}

#[test]
fn test3() {
    let program = String::from("3,9,8,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, 1);
    assert_eq!(output, vec![0]);
}

// For the following 3 tests:
// "Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:"
// From https://adventofcode.com/2019/day/5#part2
#[test]
fn test4() {
    let program = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
    let (_, output) = run_program(program, 0);
    assert_eq!(output, vec![0]);
}

#[test]
fn test5() {
    let program = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
    let (_, output) = run_program(program, 0);
    assert_eq!(output, vec![0]);
}

#[test]
fn test6() {
    let program = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
    let (_, output) = run_program(program, -50);
    assert_eq!(output, vec![1]);
}

// The following 6 tests, test if the input is equal to 8
#[test]
fn test_is_equal_to_8_version1() {
    let program = String::from("3,9,8,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, -50);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_is_equal_to_8_version2() {
    let program = String::from("3,9,8,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, 8);
    assert_eq!(output, vec![1]);
}

#[test]
fn test_is_equal_to_8_version3() {
    let program = String::from("3,9,8,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, 50);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_is_equal_to_8_version4() {
    let program = String::from("3,3,1108,-1,8,3,4,3,99");
    let (_, output) = run_program(program, -50);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_is_equal_to_8_version5() {
    let program = String::from("3,3,1108,-1,8,3,4,3,99");
    let (_, output) = run_program(program, 8);
    assert_eq!(output, vec![1]);
}

#[test]
fn test_is_equal_to_8_version6() {
    let program = String::from("3,3,1108,-1,8,3,4,3,99");
    let (_, output) = run_program(program, 50);
    assert_eq!(output, vec![0]);
}

// The following 6 tests, test if the input is less than to 8
#[test]
fn test_is_less_than_8_version1() {
    let program = String::from("3,9,7,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, -50);
    assert_eq!(output, vec![1]);
}

#[test]
fn test_is_less_than_8_version2() {
    let program = String::from("3,9,7,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, 8);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_is_less_than_8_version3() {
    let program = String::from("3,9,7,9,10,9,4,9,99,-1,8");
    let (_, output) = run_program(program, 50);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_is_lass_than_8_version4() {
    let program = String::from("3,3,1107,-1,8,3,4,3,99");
    let (_, output) = run_program(program, -50);
    assert_eq!(output, vec![1]);
}

#[test]
fn test_is_lass_than_8_version5() {
    let program = String::from("3,3,1107,-1,8,3,4,3,99");
    let (_, output) = run_program(program, 8);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_is_lass_than_8_version6() {
    let program = String::from("3,3,1107,-1,8,3,4,3,99");
    let (_, output) = run_program(program, 50);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_complex1() {
    let program = String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
    let (_, output) = run_program(program, -50);
    assert_eq!(output, vec![999]);
}

#[test]
fn test_complex2() {
    let program = String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
    let (_, output) = run_program(program, 8);
    assert_eq!(output, vec![1000]);
}

#[test]
fn test_complex3() {
    let program = String::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
    let (_, output) = run_program(program, 50);
    assert!(output[0] > 1000);
}

#[test]
fn test_relative1() {
    let program = String::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    let (_, output) = run_program(program, 0);
    assert_eq!(
        output,
        vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    );
}

#[test]
fn test_large_number1() {
    //    1102,34915192,34915192,7,4,7,99,0
    let program = String::from("1102,34915192,34915192,7,4,7,99,0");
    let (_, output) = run_program(program, 0);
    let n = format!("{}", output[0]);
    assert_eq!(16, n.len());
}

#[test]
fn test_large_number2() {
    let program = String::from("104,1125899906842624,99");
    let (_, output) = run_program(program, 0);
    assert_eq!(output, vec![1_125_899_906_842_624]);
}

#[test]
fn additional_test1() {
    run_test("109,-1,004,1,99", -1);
}

#[test]
fn additional_test2() {
    run_test("109,-1,104,1,99", 1);
}

#[test]
fn additional_test3() {
    run_test("109,-1,204,1,99", 109);
}

#[test]
fn additional_test4() {
    run_test("109,5,004,0,99,-10,-11,-12", 109);
}

#[test]
fn additional_test5() {
    run_test("109,5,104,0,99,-10,-11,-12", 0);
}

#[test]
fn additional_test6() {
    run_test("109,5,204,0,99,-10,-11,-12", -10);
}

#[test]
fn additional_test7() {
    run_test("109,6,204,0,99,-10,-11,-12", -11);
}

#[test]
fn additional_test8() {
    run_test("109,7,204,0,99,-10,-11,-12", -12);
}

fn run_test(program: &str, expected_output: i64) {
    let program = String::from(program);
    let (_, output) = run_program(program, 0);
    assert_eq!(output, vec![expected_output]);
}
