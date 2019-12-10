#[derive(Debug, Clone)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Add,
    Mul,
    In,
    Out,
    Jif,
    Jeq,
    Lt,
    Eq,
    Rb,
    Brk,
}

impl From<i64> for Opcode {
    fn from(i: i64) -> Self {
        match i {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::In,
            4 => Opcode::Out,
            5 => Opcode::Jif,
            6 => Opcode::Jeq,
            7 => Opcode::Lt,
            8 => Opcode::Eq,
            9 => Opcode::Rb,
            99 => Opcode::Brk,
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
            let mode = match full_opcode % 10 {
                0 => Mode::Position,
                1 => Mode::Immediate,
                2 => Mode::Relative,
                _ => unreachable!(),
            };
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
