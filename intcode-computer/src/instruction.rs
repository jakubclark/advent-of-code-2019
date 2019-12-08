#[derive(Debug, Clone)]
pub enum Mode {
    Position,
    Immediate,
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: i64,
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
