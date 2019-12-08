use crate::instruction::{Instruction, Mode};

#[derive(Debug, Default)]
/// A machine which is capable of running intcode programs.
/// It takes a program and a vector of input values, as input.
/// Once done, it returns its memory and a vector of output values (if any).
pub struct Machine {
    memory: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
    cur_i: usize,
    input_i: usize,
    halted: bool,
}

impl Machine {
    pub fn new(program: String, input: Vec<i64>) -> Self {
        let memory: Vec<i64> = program
            .trim()
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|opcode| {
                opcode.parse().unwrap_or_else(|err| {
                    panic!("Not a number: err={:?} | opcode={:?}", err, opcode);
                })
            })
            .collect();
        Self {
            memory,
            input,
            ..Default::default()
        }
    }

    pub fn is_running(&self) -> bool {
        !self.halted
    }

    pub fn push_input(&mut self, input: i64) {
        self.input.push(input);
    }

    pub fn get_result(&self) -> i64 {
        *self.output.last().expect("Program produced no result")
    }

    /// Get the argument for `instruction`, based on it's mode
    fn get_argument(&self, instruction: &Instruction, arg_position: usize) -> i64 {
        let index = self.cur_i + arg_position + 1;
        match instruction.get_mode(arg_position) {
            Mode::Position => self.memory[self.memory[index] as usize],
            Mode::Immediate => self.memory[index],
        }
    }

    /// Fetch the next instruction
    fn fetch_next_instruction(&self) -> Instruction {
        let opcode = *self.memory.get(self.cur_i).unwrap_or_else(|| {
            panic!("`self.cur_i` is out of range: {}", self.cur_i);
        });
        Instruction::new(opcode)
    }

    /// Runs the program, until it is complete. Returns the resulting memory and output.
    pub fn run(mut self) -> (Vec<i64>, Vec<i64>) {
        while !self.halted {
            self.step()
        }
        (self.memory, self.output)
    }

    /// Runs the program, until an OUT instruction is executed or the program is done.
    pub fn step(&mut self) {
        loop {
            let instruction = self.fetch_next_instruction();
            match instruction.opcode {
                1 => {
                    let left = self.get_argument(&instruction, 0);
                    let right = self.get_argument(&instruction, 1);
                    let dest = self.memory[self.cur_i + 3] as usize;
                    //                println!("ADD {} {} => {}", left, right, dest);
                    self.memory[dest] = left + right;
                    self.cur_i += 4;
                }
                2 => {
                    let left = self.get_argument(&instruction, 0);
                    let right = self.get_argument(&instruction, 1);
                    let dest = self.memory[self.cur_i + 3] as usize;
                    //                println!("MUL {} {} => {}", left, right, dest);
                    self.memory[dest] = left * right;
                    self.cur_i += 4;
                }
                3 => {
                    let dest = self.memory[self.cur_i + 1] as usize;
                    let input = *self.input.get(self.input_i).unwrap_or_else(|| {
                        panic!(
                            "Not enough inputs provided. At least {} inputs expected",
                            self.input_i + 1
                        );
                    });
                    //                println!("IN  {} => {}", input, dest);
                    self.memory[dest] = input;
                    self.input_i += 1;
                    self.cur_i += 2;
                }
                4 => {
                    let arg = self.get_argument(&instruction, 0);
                    //                println!("OUT <= {}", arg);
                    self.output.push(arg);
                    self.cur_i += 2;
                    break;
                }
                5 => {
                    let arg = self.get_argument(&instruction, 0);
                    //                println!("JIF {}", arg);
                    if arg != 0 {
                        self.cur_i = self.get_argument(&instruction, 1) as usize;
                    } else {
                        self.cur_i += 3;
                    }
                }
                6 => {
                    let arg = self.get_argument(&instruction, 0);
                    //                println!("JEQ {}", arg);
                    if arg == 0 {
                        self.cur_i = self.get_argument(&instruction, 1) as usize;
                    } else {
                        self.cur_i += 3;
                    }
                }
                7 => {
                    let left = self.get_argument(&instruction, 0);
                    let right = self.get_argument(&instruction, 1);
                    let dest = self.memory[self.cur_i + 3] as usize;
                    //                println!("LT  {} {} => {}", left, right, dest);
                    self.memory[dest] = i64::from(left < right);
                    self.cur_i += 4;
                }
                8 => {
                    let left = self.get_argument(&instruction, 0);
                    let right = self.get_argument(&instruction, 1);
                    let dest = self.memory[self.cur_i + 3] as usize;
                    //                println!("EQ  {} {} => {}", left, right, dest);
                    self.memory[dest] = i64::from(left == right);
                    self.cur_i += 4;
                }
                99 => {
                    self.halted = true;
                    break;
                }
                _ => unreachable!(),
            }
        }
    }
}

pub fn run_program(program: String, input: &[i64]) -> (Vec<i64>, Vec<i64>) {
    let machine = Machine::new(program, Vec::from(input));
    machine.run()
}
