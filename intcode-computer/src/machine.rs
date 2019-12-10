use crate::instruction::{Instruction, Mode, Opcode::*};

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
    relative_base: usize,
    halted: bool,
}

impl Machine {
    pub fn new(program: String, input: Vec<i64>) -> Self {
        let mut memory: Vec<i64> = program
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
        memory.append(&mut vec![0; 1000]);
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
            Mode::Relative => self.memory[index + self.relative_base],
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
                Add => {
                    let left = self.get_argument(&instruction, 0);
                    let right = self.get_argument(&instruction, 1);
                    let dest = self.memory[self.cur_i + 3] as usize;
                    self.memory[dest] = left + right;
                    self.cur_i += 4;
                }
                Mul => {
                    let left = self.get_argument(&instruction, 0);
                    let right = self.get_argument(&instruction, 1);
                    let dest = self.memory[self.cur_i + 3] as usize;
                    self.memory[dest] = left * right;
                    self.cur_i += 4;
                }
                In => {
                    let dest = self.memory[self.cur_i + 1] as usize;
                    let input = *self.input.get(self.input_i).unwrap_or_else(|| {
                        panic!(
                            "Not enough inputs provided. At least {} inputs expected",
                            self.input_i + 1
                        );
                    });
                    self.memory[dest] = input;
                    self.input_i += 1;
                    self.cur_i += 2;
                }
                Out => {
                    let arg = self.get_argument(&instruction, 0);
                    self.output.push(arg);
                    self.cur_i += 2;
                    break;
                }
                Jif => {
                    let arg = self.get_argument(&instruction, 0);
                    if arg != 0 {
                        self.cur_i = self.get_argument(&instruction, 1) as usize;
                    } else {
                        self.cur_i += 3;
                    }
                }
                Jeq => {
                    let arg = self.get_argument(&instruction, 0);
                    if arg == 0 {
                        self.cur_i = self.get_argument(&instruction, 1) as usize;
                    } else {
                        self.cur_i += 3;
                    }
                }
                Lt => {
                    let left = self.get_argument(&instruction, 0);
                    let right = self.get_argument(&instruction, 1);
                    let dest = self.memory[self.cur_i + 3] as usize;
                    self.memory[dest] = i64::from(left < right);
                    self.cur_i += 4;
                }
                Eq => {
                    let left = self.get_argument(&instruction, 0);
                    let right = self.get_argument(&instruction, 1);
                    let dest = self.memory[self.cur_i + 3] as usize;
                    self.memory[dest] = i64::from(left == right);
                    self.cur_i += 4;
                }
                Rb => {
                    self.relative_base += self.get_argument(&instruction, 0) as usize;
                    self.cur_i += 2;
                }
                Brk => {
                    self.halted = true;
                    break;
                }
            }
        }
    }
}

pub fn run_program(program: String, input: &[i64]) -> (Vec<i64>, Vec<i64>) {
    let machine = Machine::new(program, Vec::from(input));
    machine.run()
}
