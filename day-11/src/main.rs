use std::fs::read_to_string;
use intcode_computer::{run_program, Machine};
use std::collections::HashMap;
use crate::Color::{Black, White};
use crate::Direction::{Up, Right, Down, Left};

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Color {
    Black,
    White,
}

type Point = (i64, i64);

struct Robot {
    x: i64,
    y: i64,
    direction: Direction,
    cells: HashMap<Point, Color>,
    machine: Machine,
}

impl Robot {
    pub fn new(program: String) -> Self {
        let machine = Machine::new(program, vec![1]);
        Self {
            x: 0,
            y: 0,
            direction: Up,
            cells: HashMap::new(),
            machine,
        }
    }

    pub fn run(&mut self) {
        while self.machine.is_running() {
            self.step();
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        };
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn make_move(&mut self) {
        match self.direction {
            Up => self.y -= 1,
            Right => self.x += 1,
            Down => self.y += 1,
            Left => self.x -= 1,
        }
    }

    fn step(&mut self) {
        self.machine.step();
        self.machine.step();
        let out = self.machine.get_output();
        let len = out.len();

        let turn_amount = out.get(len - 1).expect("No last entry");
        let color_to_paint = out.get(len - 2).expect("No second-to-last entry");

        println!("turn_amount={:?} | color_to_paint={:?}", turn_amount, color_to_paint);

        let color_to_paint = match color_to_paint {
            0 => Black,
            1 => White,
            _ => unreachable!("foobar"),
        };

        if let Some(color) = self.cells.get_mut(&(self.x, self.y)) {
            *color = color_to_paint
        } else {
            self.cells.insert((self.x, self.y), color_to_paint);
        }


        match *turn_amount {
            0 => self.turn_left(),
            1 => self.turn_right(),
            _ => unreachable!("Unexpected turn code"),
        }

        self.make_move();
    }
}

fn main() {
    let program = read_to_string("input.txt").expect("Failed to open input.txt");
    let mut robot = Robot::new(program);
    robot.run();
}
