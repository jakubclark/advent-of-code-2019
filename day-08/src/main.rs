use std::fs::read_to_string;

type Layer = Vec<i64>;

fn get_layers(img: String, width: usize, height: usize) -> Vec<Layer> {
    let nums: Vec<i64> = img
        .chars()
        .map(|c| {
            c.to_string().parse().unwrap_or_else(|err| {
                panic!("Not a number. err={:?} | c={:?}", err, c);
            })
        })
        .collect();

    nums.chunks(width * height)
        .map(Vec::from)
        .collect()
}

fn part1(layers: &[Layer]) {
    let idx = layers
        .iter()
        .enumerate()
        .map(|(idx, layer)| {
            let count = layer.iter().filter(|&&n| n == 0).count();
            (idx, count)
        })
        .min_by_key(|(_, c)| *c)
        .map(|(i, _)| i)
        .expect("Could not get min index");

    let num_ones = layers[idx].iter().filter(|&&n| n == 1).count();
    let num_twos = layers[idx].iter().filter(|&&n| n == 2).count();

    println!("Solution for part 1 = {}", num_ones * num_twos);
}

fn main() {
    let img = read_to_string("input.txt")
        .expect("Failed to open input.txt")
        .trim()
        .to_owned();

    let layers = get_layers(img, 25, 6);
    part1(&layers);
}
