use crate::TileType::{Ball, Block, Empty, HorizontalPaddle, Wall};
use intcode_computer::Machine;
use std::collections::HashMap;
use std::fs::read_to_string;

type Coords = (i64, i64);
type Tiles = HashMap<Coords, TileType>;

trait Display {
    fn display(&self);
}

impl Display for Tiles {
    fn display(&self) {
        let min_x = self.keys().map(|t| t.0).min().expect("No min X");
        let min_y = self.keys().map(|t| t.1).min().expect("No min Y");
        let max_x = self.keys().map(|t| t.0).max().expect("No min X");
        let max_y = self.keys().map(|t| t.1).max().expect("No min X");

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c: char = self.get(&(x, y)).unwrap_or(&Empty).into();
                print!("{}", c);
            }
            println!();
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum TileType {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl Into<TileType> for i64 {
    fn into(self) -> TileType {
        match self {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => HorizontalPaddle,
            4 => Ball,
            _ => unreachable!("Unexpected tile id: {}", self),
        }
    }
}

impl Into<char> for &TileType {
    fn into(self) -> char {
        match self {
            Empty => ' ',
            Wall => '_',
            Block => 'X',
            HorizontalPaddle => '-',
            Ball => 'O',
        }
    }
}

fn get_x_of_tile_type(tiles: &Tiles, tile_type: TileType) -> i64 {
    tiles
        .iter()
        .find(|(_, &t)| t == tile_type)
        .map(|(coords, _)| coords.0)
        .unwrap_or(0)
}

fn part1(program: String) {
    let mut machine = Machine::new(program, 0);
    let mut tiles: Tiles = Tiles::new();
    while machine.is_running() {
        machine.step();
        machine.step();
        machine.step();
        let out = machine.get_output();
        let l = out.len();
        let x = *out.get(l - 3).expect("No output");
        let y = *out.get(l - 2).expect("No output");
        let tile_type = *out.get(l - 1).expect("No output");
        let tile_type = tile_type.into();
        tiles.insert((x, y), tile_type);
    }

    tiles.display();

    let num_blocks = tiles
        .iter()
        .filter(|t| match t.1 {
            Block => true,
            _ => false,
        })
        .count();

    println!("Solution for part 1: {}", num_blocks);
}

fn part2(program: String) {
    let mut machine = Machine::new(program.clone(), 0);

    machine.set_memory(0, 2);

    let mut tiles: Tiles = Tiles::new();
    let mut last_score = i64::min_value();
    while machine.is_running() {
        machine.step();
        machine.step();
        machine.step();

        let out = machine.get_output();
        let l = out.len();
        let x = *out.get(l - 3).expect("No output");
        let y = *out.get(l - 2).expect("No output");
        let third = *out.get(l - 1).expect("No output");

        if x == -1 && y == 0 {
            println!("Score: {}", third);
            last_score = third;
        } else {
            let tile_type = third.into();
            tiles.insert((x, y), tile_type);

            let ball_x = get_x_of_tile_type(&tiles, Ball);
            let paddle_x = get_x_of_tile_type(&tiles, HorizontalPaddle);

            if paddle_x < ball_x {
                machine.push_input(1);
            } else if paddle_x > ball_x {
                machine.push_input(-1);
            } else {
                machine.push_input(0);
            }
        }
    }

    println!("Solution for part 2: {}", last_score);
}

fn main() {
    let program = read_to_string("input.txt").expect("Failed to open input.txt");
    part1(program.clone());
    part2(program);
}
