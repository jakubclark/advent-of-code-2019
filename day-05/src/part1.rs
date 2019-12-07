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

pub fn run_program(program: String, input: i64) -> (Vec<i64>, Vec<i64>) {
    let mut memory: Vec<i64> = program
        .trim()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|opcode| opcode.parse().expect("Not a number"))
        .collect();

    let mut i = 0;
    let mut output = vec![];

    while let Some(opcode) = memory.get(i) {
        let instruction = Instruction::new(*opcode);
        match instruction.opcode {
            1 => {
                let left = get_argument(&memory, &instruction, i, 0);
                let right = get_argument(&memory, &instruction, i, 1);
                let dest = memory[i + 3];
                println!("ADD {} {} => {}", left, right, dest);
                memory[dest as usize] = left + right;
                i += 4;
            }
            2 => {
                let left = get_argument(&memory, &instruction, i, 0);
                let right = get_argument(&memory, &instruction, i, 1);
                let dest = memory[i + 3];
                println!("MUL {} {} => {}", left, right, dest);
                memory[dest as usize] = left * right;
                i += 4;
            }
            3 => {
                let dest = memory[i + 1] as usize;
                memory[dest] = input;
                println!("IN  {} => {}", input, dest);
                i += 2;
            }
            4 => {
                let arg = get_argument(&memory, &instruction, i, 0);
                println!("OUT <= {}", arg);
                output.push(arg);
                i += 2;
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
        let (memory, output) = run_program(program, 50);
        assert_eq!(output, vec![50]);
        assert_eq!(memory, vec![50, 0, 4, 0, 99]);
    }

    #[test]
    fn test2() {
        let program = String::from("1101,100,-1,0,99");
        let (memory, _) = run_program(program, 0);
        assert_eq!(memory, vec![99, 100, -1, 0, 99]);
    }
}
