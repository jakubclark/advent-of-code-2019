#[derive(Debug, Clone)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Debug)]
struct Instruction {
    opcode: i64,
    argument_modes: Vec<Mode>,
}

impl Instruction {
    pub fn new(mut full_opcode: i64) -> Self {
        let opcode = full_opcode % 100;
        full_opcode /= 100;

        let mut argument_modes = vec![];

        while full_opcode > 0 {
            let mode = match full_opcode % 10 {
                0 => Mode::Position,
                1 => Mode::Immediate,
                _ => unreachable!(),
            };
            argument_modes.push(mode);
            full_opcode /= 10;
        }
        Self {
            opcode,
            argument_modes,
        }
    }

    pub fn get_mode(&self, index: usize) -> Mode {
        self.argument_modes
            .get(index)
            .map(Clone::clone)
            .unwrap_or_else(|| Mode::Position)
    }
}

fn get_argument(
    memory: &[i64],
    instruction: &Instruction,
    current_index: usize,
    arg_position: usize,
) -> i64 {
    let index = current_index + arg_position + 1;
    let mode = instruction.get_mode(arg_position);
    match mode {
        Mode::Position => memory[memory[index] as usize],
        Mode::Immediate => memory[index],
    }
}

pub fn run_program(program: String, input: &[i64]) -> (Vec<i64>, Vec<i64>) {
    let mut memory: Vec<i64> = program
        .trim()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|opcode| opcode.parse().expect("Not a number"))
        .collect();

    let mut i = 0;
    let mut output = vec![];
    let mut input_i = 0;

    while let Some(opcode) = memory.get(i) {
        let instruction = Instruction::new(*opcode);
        match instruction.opcode {
            1 => {
                let left = get_argument(&memory, &instruction, i, 0);
                let right = get_argument(&memory, &instruction, i, 1);
                let dest = memory[i + 3] as usize;
                println!("ADD {} {} => {}", left, right, dest);
                memory[dest] = left + right;
                i += 4;
            }
            2 => {
                let left = get_argument(&memory, &instruction, i, 0);
                let right = get_argument(&memory, &instruction, i, 1);
                let dest = memory[i + 3] as usize;
                println!("MUL {} {} => {}", left, right, dest);
                memory[dest] = left * right;
                i += 4;
            }
            3 => {
                let dest = memory[i + 1] as usize;
                let input = *input.get(input_i).unwrap_or_else(|| {
                    panic!(
                        "Not enough inputs provided. At least {} inputs expected",
                        input_i + 1
                    );
                });
                println!("IN  {} => {}", input, dest);
                memory[dest] = input;
                input_i += 1;
                i += 2;
            }
            4 => {
                let arg = get_argument(&memory, &instruction, i, 0);
                println!("OUT <= {}", arg);
                output.push(arg);
                i += 2;
            }
            5 => {
                let arg = get_argument(&memory, &instruction, i, 0);
                println!("JIF {}", arg);
                if arg != 0 {
                    i = get_argument(&memory, &instruction, i, 1) as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                let arg = get_argument(&memory, &instruction, i, 0);
                println!("JEQ {}", arg);
                if arg == 0 {
                    i = get_argument(&memory, &instruction, i, 1) as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                let left = get_argument(&memory, &instruction, i, 0);
                let right = get_argument(&memory, &instruction, i, 1);
                let dest = memory[i + 3] as usize;
                println!("LT  {} {} => {}", left, right, dest);
                memory[dest] = i64::from(left < right);
                i += 4;
            }
            8 => {
                let left = get_argument(&memory, &instruction, i, 0);
                let right = get_argument(&memory, &instruction, i, 1);
                let dest = memory[i + 3] as usize;
                println!("EQ  {} {} => {}", left, right, dest);
                memory[dest] = i64::from(left == right);
                i += 4;
            }
            99 => break,
            _ => unreachable!(),
        }
    }
    (memory, output)
}

#[cfg(test)]
mod tests {
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
}
