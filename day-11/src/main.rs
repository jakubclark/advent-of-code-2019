use crate::Color::{Black, White};
use crate::Direction::{Down, Left, Right, Up};
use intcode_computer::Machine;
use std::collections::BTreeMap;
use std::fs::read_to_string;

type Point = (i64, i64);

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
enum Color {
    Black,
    White,
}

impl Color {
    pub fn to_num(&self) -> i64 {
        match self {
            Black => 0,
            White => 1,
        }
    }
}

impl From<i64> for Color {
    fn from(i: i64) -> Self {
        match i {
            0 => Black,
            1 => White,
            _ => unreachable!("Invalid color value: {}", i),
        }
    }
}

#[derive(Debug)]
struct Robot {
    x: i64,
    y: i64,
    direction: Direction,
    // Keeps a history of colors, for each cell
    cells: BTreeMap<Point, Vec<Color>>,
    machine: Machine,
}

impl Robot {
    pub fn new(program: String, starting_color: Color) -> Self {
        let machine = Machine::new(program, vec![starting_color.to_num()]);
        Self {
            x: 0,
            y: 0,
            direction: Up,
            cells: BTreeMap::new(),
            machine,
        }
    }

    pub fn count_painted_cells(&self) -> usize {
        self.cells.len() - 1
    }

    pub fn show_result(&self) {
        let min_x = self.cells.keys().map(|k| k.0).min().expect("No min x");
        let max_x = self.cells.keys().map(|k| k.0).max().expect("No max x");
        let min_y = self.cells.keys().map(|k| k.1).min().expect("No min y");
        let max_y = self.cells.keys().map(|k| k.1).max().expect("No max y");

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c = self
                    .cells
                    .get(&(x, y))
                    .map(|colors| {
                        let color = colors.last().expect("No last color");
                        match color {
                            Black => ' ',
                            White => '\u{2593}',
                        }
                    })
                    .unwrap_or(' ');
                print!("{}", c);
            }
            println!();
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
        // Get the color to paint
        self.machine.step();
        let color_to_paint = (*self.machine.get_output().last().expect("No last entry")).into();

        // Get the turn direction
        self.machine.step();
        let turn_amount = self.machine.get_output().last().expect("No last entry");

        // Paint the current panel
        if let Some(colors) = self.cells.get_mut(&(self.x, self.y)) {
            colors.push(color_to_paint);
        } else {
            self.cells
                .insert((self.x, self.y), vec![Black, color_to_paint]);
        }

        // Turn the robot
        match *turn_amount {
            0 => self.turn_left(),
            1 => self.turn_right(),
            _ => unreachable!("Unexpected turn code"),
        }

        // Move forward by 1
        self.make_move();

        // Get the current cell color
        let input = match self.cells.get(&(self.x, self.y)) {
            Some(colors) => colors.last().expect("No last entry").to_num(),
            None => Black.to_num(),
        };

        self.machine.push_input(input);
    }
}

fn main() {
    let program = read_to_string("input.txt").expect("Failed to open input.txt");
    let mut robot1 = Robot::new(program.clone(), Black);
    robot1.run();
    robot1.show_result();
    println!("Solution for part 1: {}", robot1.count_painted_cells());

    let mut robot2 = Robot::new(program, White);
    robot2.run();
    println!("Solution for part 2: ");
    robot2.show_result();
}
