use std::collections::HashMap;
use std::fs::read_to_string;

type Map<'a> = HashMap<&'a str, &'a str>;

// Maps orbiter-to-orbitee, e.g.: COM)B -> B:COM
fn get_direct_orbits(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.split(')').collect())
        .map(|parts: Vec<&str>| (parts[1], parts[0]))
        .collect()
}

/// Counts both direct and indirect orbits of `map`
fn count_all_orbits(map: &Map) -> i64 {
    map.keys().fold(0, |mut count, mut orbitee| {
        while *orbitee != "COM" {
            count += 1;
            orbitee = &map[*orbitee]
        }
        count
    })
}

fn part1() {
    let input = read_to_string("input.txt").expect("Failed to open input.txt");
    let orbits = get_direct_orbits(&input);
    let count = count_all_orbits(&orbits);
    println!("Solution for Part 1 = {}", count);
}

fn main() {
    part1();
}

#[cfg(tests)]
mod tests {
    use crate::{count_all_orbits, get_direct_orbits};

    #[test]
    fn test1() {
        let orbits = get_direct_orbits("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        let count = count_all_orbits(&orbits);
        assert_eq!(count, 42);
    }
}
