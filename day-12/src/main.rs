use std::fs::read_to_string;
use std::hash::Hash;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    pub fn from(nums: &[i64]) -> Self {
        Self {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        }
    }
}

type Velocity = Position;

//#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
//struct Velocity {
//    x: i64,
//    y: i64,
//    z: i64,
//}

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
struct Moon {
    position: Position,
    velocity: Velocity,
}

impl Moon {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn apply_gravity(&mut self, other: Moon) {
        let x_diff = self.position.x - other.position.x;
        if x_diff < 0 {
            self.velocity.x += 1;
        } else if x_diff > 0 {
            self.velocity.x -= 1;
        }

        let y_diff = self.position.y - other.position.y;
        if y_diff < 0 {
            self.velocity.y += 1;
        } else if y_diff > 0 {
            self.velocity.y -= 1;
        }

        let z_diff = self.position.z - other.position.z;
        if z_diff < 0 {
            self.velocity.z += 1;
        } else if z_diff > 0 {
            self.velocity.z -= 1;
        }
    }

    pub fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    pub fn kinetic_energy(&self) -> i64 {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs())
            * (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs())
    }
}

fn to_positions(input: &str) -> Vec<Moon> {
    input
        .lines()
        .map(|line| {
            let line = line.replace('>', "").replace('<', "");
            let coords: Vec<&str> = line.split(',').collect();

            let coords: Vec<i64> = coords
                .into_iter()
                .map(|coord| coord.trim().split('=').collect::<Vec<&str>>())
                .map(|part| part[1].parse::<i64>().expect("Not a number"))
                .collect::<Vec<i64>>();
            Position::from(&coords)
        })
        .map(Moon::new)
        .collect()
}

fn step(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j {
                continue;
            }
            let other = moons[j];
            moons[i].apply_gravity(other);
        }
    }

    for m in moons {
        m.apply_velocity();
    }
}

fn gcd(n: i64, m: i64) -> i64 {
    if n == 0 {
        m.abs()
    } else {
        gcd(m % n, n)
    }
}

fn lcm(n: i64, m: i64) -> i64 {
    n * m / gcd(n, m)
}

fn cmp(a: i64, b: i64) -> i64 {
    if a > b {
        -1
    } else if b > a {
        1
    } else {
        0
    }
}

// Based on https://github.com/jdlambert/advent-of-code-2019/blob/master/day12/src/main.rs
fn cycle(mut pairs: Vec<(i64, i64)>) -> i64 {
    let target = pairs.clone();
    let mut new_pairs = vec![];
    let mut count = 0;
    loop {
        for (i, moon) in pairs.iter().enumerate() {
            let mut ddx = 0;
            for (j, other) in pairs.iter().enumerate() {
                if i != j {
                    ddx += cmp(moon.0, other.0);
                }
            }
            let dx = moon.1 + ddx;
            let x = moon.0 + dx;

            new_pairs.push((x, dx))
        }
        count += 1;
        pairs = std::mem::replace(&mut new_pairs, vec![]);
        if pairs == target {
            return count;
        }
    }
}

fn part1(mut moons: Vec<Moon>) {
    for _ in 0..100 {
        step(&mut moons);
    }

    let total_kinetic_energy: i64 = moons.iter().map(|m| m.kinetic_energy()).sum();
    println!("Solution for part 1: {}", total_kinetic_energy);
}

fn part2(moons: Vec<Moon>) {
    let starting_xs: Vec<(i64, i64)> = moons.iter().map(|m| (m.position.x, m.velocity.x)).collect();
    let starting_ys: Vec<(i64, i64)> = moons.iter().map(|m| (m.position.y, m.velocity.y)).collect();
    let starting_zs: Vec<(i64, i64)> = moons.iter().map(|m| (m.position.z, m.velocity.z)).collect();

    let x = cycle(starting_xs);
    let y = cycle(starting_ys);
    let z = cycle(starting_zs);

    println!("{}", lcm(lcm(x, y), z));
}

fn main() {
    let s = read_to_string("input.txt").expect("Failed to open input.txt");
    let moons = to_positions(&s);

    part1(moons.clone());
    part2(moons);
}
