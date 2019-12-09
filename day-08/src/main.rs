use std::fs::read_to_string;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn part1(layers: &[Vec<u32>]) {
    layers
        .iter()
        .min_by_key(|&layer| layer.iter().filter(|&&n| n == 0).count())
        .and_then(|layer| {
            let num_ones = layer.iter().filter(|&&n| n == 1).count();
            let num_twos = layer.iter().filter(|&&n| n == 2).count();
            println!("Solution for part 1 = {}", num_ones * num_twos);
            Some(())
        })
        .expect("Failed to find layer");
}

fn part2(layers: &[Vec<u32>]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = layers
                .iter()
                .map(|layer| layer[y * WIDTH + x])
                .find(|&n| n != 2)
                .unwrap_or(2);
            if pixel == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let img: Vec<Vec<u32>> = read_to_string("input.txt")
        .expect("Failed to open input.txt")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Not a number"))
        .collect::<Vec<u32>>()
        .chunks(WIDTH * HEIGHT)
        .map(Vec::from)
        .collect();
    part1(&img);
    part2(&img);
}
