#[derive(Debug, Clone)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for Mode {
    fn from(i: i64) -> Self {
        match i {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Opcode {
    ADD,
    MUL,
    IN,
    OUT,
    JIF,
    JEQ,
    LT,
    EQ,
    RB,
    BRK,
}

impl From<i64> for Opcode {
    fn from(i: i64) -> Self {
        match i {
            1 => Opcode::ADD,
            2 => Opcode::MUL,
            3 => Opcode::IN,
            4 => Opcode::OUT,
            5 => Opcode::JIF,
            6 => Opcode::JEQ,
            7 => Opcode::LT,
            8 => Opcode::EQ,
            9 => Opcode::RB,
            99 => Opcode::BRK,
            _ => unreachable!("Unexpected opcode: {}", i),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    argument_modes: Vec<Mode>,
}

impl Instruction {
    pub fn new(mut full_opcode: i64) -> Self {
        let opcode = full_opcode % 100;
        full_opcode /= 100;

        let mut argument_modes = vec![];

        while full_opcode > 0 {
            let mode = (full_opcode % 10).into();
            argument_modes.push(mode);
            full_opcode /= 10;
        }

        let opcode = opcode.into();

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
