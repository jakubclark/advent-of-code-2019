use std::fs::read_to_string;

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

fn main() {
    let img: Vec<Vec<u32>> = read_to_string("input.txt")
        .expect("Failed to open input.txt")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Not a number"))
        .collect::<Vec<u32>>()
        .chunks(25 * 6)
        .map(Vec::from)
        .collect();
    part1(&img);
}
