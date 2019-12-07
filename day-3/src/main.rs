use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::BufRead, BufReader, Error as IoError};

type Point = (i32, i32);

/// Convert String to a mapping of Point-to-distance
fn traverse(path: String) -> HashMap<Point, i32> {
    let (mut cur_x, mut cur_y) = (0, 0);
    let mut distance = 0;

    let mut positions_map = HashMap::new();

    path.split(',')
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|op| {
            let dir: &str = &op[0..1];

            let num_steps: i64 = op[1..].parse().expect("Not a number");

            match dir {
                "U" => {
                    for _ in 0..num_steps {
                        cur_y += 1;
                        distance += 1;
                        positions_map.insert((cur_x, cur_y), distance);
                    }
                }
                "D" => {
                    for _ in 0..num_steps {
                        cur_y -= 1;
                        distance += 1;
                        positions_map.insert((cur_x, cur_y), distance);
                    }
                }
                "R" => {
                    for _ in 0..num_steps {
                        cur_x += 1;
                        distance += 1;
                        positions_map.insert((cur_x, cur_y), distance);
                    }
                }
                "L" => {
                    for _ in 0..num_steps {
                        cur_x -= 1;
                        distance += 1;
                        positions_map.insert((cur_x, cur_y), distance);
                    }
                }
                _ => unreachable!(),
            }
        });

    positions_map
}

fn compute_min_manhattan(first: &HashMap<Point, i32>, second: &HashMap<Point, i32>) -> i32 {
    let first: HashSet<Point> = first.keys().cloned().collect();
    let second: HashSet<Point> = second.keys().cloned().collect();
    first
        .intersection(&second)
        .map(|coords| coords.0.abs() + coords.1.abs())
        .min()
        .expect("No collisions")
}

fn compute_min_steps_sum(first: &HashMap<Point, i32>, second: &HashMap<Point, i32>) -> i32 {
    let first_set: HashSet<Point> = first.keys().cloned().collect();
    let second_set: HashSet<Point> = second.keys().cloned().collect();
    first_set
        .intersection(&second_set)
        .map(|k| (first.get(k).unwrap(), second.get(k).unwrap()))
        .map(|(n1, n2)| *n1 + *n2)
        .min()
        .expect("No min distance")
}

fn part1() -> Result<(), IoError> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let all_positions: Vec<HashMap<Point, i32>> =
        reader.lines().map(Result::unwrap).map(traverse).collect();

    let first = all_positions.get(0).unwrap();
    let second = all_positions.get(1).unwrap();
    let min = compute_min_manhattan(first, second);
    println!("Solution for part 1 = {}", min);
    Ok(())
}

fn part2() -> Result<(), IoError> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let all_positions: Vec<HashMap<Point, i32>> =
        reader.lines().map(Result::unwrap).map(traverse).collect();

    let first = all_positions.get(0).unwrap();
    let second = all_positions.get(1).unwrap();
    let min = compute_min_steps_sum(first, second);
    println!("Solution for part 2 = {}", min);
    Ok(())
}

fn main() -> Result<(), IoError> {
    part1()?;
    part2()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{compute_min_manhattan, compute_min_steps_sum, traverse};

    fn manhattan(s1: String, s2: String, expected: i32) {
        let points1 = traverse(s1);
        let points2 = traverse(s2);
        let min = compute_min_manhattan(&points1, &points2);
        assert_eq!(
            min, expected,
            "Expected min_dist to be '{}', got '{}' instead",
            expected, min
        );
    }

    fn min_steps(s1: String, s2: String, expected: i32) {
        let m1 = traverse(s1);
        let m2 = traverse(s2);
        let min_steps_sum = compute_min_steps_sum(&m1, &m2);
        assert_eq!(
            min_steps_sum, expected,
            "Expected min_steps_sum to be '{}', got '{}' instead",
            expected, min_steps_sum
        )
    }

    #[test]
    fn test1() {
        let s1 = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let s2 = String::from("U62,R66,U55,R34,D71,R55,D58,R83");
        manhattan(s1, s2, 159);
    }

    #[test]
    fn test2() {
        let s1 = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let s2 = String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        manhattan(s1, s2, 135);
    }

    #[test]
    fn test3() {
        let s1 = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let s2 = String::from("U62,R66,U55,R34,D71,R55,D58,R83");
        min_steps(s1, s2, 610);
    }

    #[test]
    fn test4() {
        let s1 = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let s2 = String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        min_steps(s1, s2, 410);
    }
}
