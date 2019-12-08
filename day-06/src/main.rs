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

/// Find the path from `start` to `dest`
/// Assumes that `src` is further from COM than `dest`
fn path_to_target<'a>(map: &'a Map, src: &'a str, dest: &'a str) -> Vec<&'a str> {
    let mut path = vec![];

    let mut current_body = src;
    while let Some(body) = map.get(current_body) {
        if current_body == dest {
            break;
        }
        path.push(*body);
        current_body = body;
    }
    path
}

fn find_full_path<'a>(map: &'a Map, src: &'a str, dest: &'a str) -> Vec<&'a str> {
    let src_to_center = path_to_target(&map, src, "COM");
    let dest_to_center = path_to_target(&map, dest, "COM");

    let traverse_to = find_intersection(&src_to_center, &dest_to_center).unwrap_or_else(|| {
        panic!(
            "No intersection in the paths `{}`->'COM' and `{}`->'COM'",
            src, dest
        )
    });

    let src_to_inter = path_to_target(&map, "YOU", traverse_to);
    let mut inter_to_dest = path_to_target(&map, "SAN", traverse_to);

    inter_to_dest.reverse();
    // The now first element in `inter_to_dest` is the same as the first element in `src_to_inter`
    inter_to_dest.remove(0);

    let mut final_path = src_to_inter;
    final_path.append(&mut inter_to_dest);
    final_path
}

fn find_intersection<'a>(path1: &[&'a str], path2: &[&'a str]) -> Option<&'a str> {
    for entry1 in path1 {
        for entry2 in path2 {
            if entry1 == entry2 {
                return Some(entry1);
            }
        }
    }
    None
}

fn get_number_of_transfers(map: &Map, src: &str, dest: &str) -> usize {
    // The number of transfer is the number of visited bodies - 1
    find_full_path(map, src, dest).len() - 1
}

fn part1(map: &Map) {
    let count = count_all_orbits(&map);
    println!("Solution for Part 1 = {}", count);
}

fn part2(map: &Map) {
    let number_of_transfers = get_number_of_transfers(&map, "YOU", "SAN");
    println!("Solution for Part 2 = {}", number_of_transfers);
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to open input.txt");
    let map = get_direct_orbits(&input);
    part1(&map);
    part2(&map);
}

#[cfg(test)]
mod tests {
    use crate::{
        count_all_orbits, find_full_path, find_intersection, get_direct_orbits,
        get_number_of_transfers, path_to_target,
    };

    #[test]
    fn test1() {
        let map = get_direct_orbits("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        let count = count_all_orbits(&map);
        assert_eq!(count, 42);
    }

    #[test]
    fn test2() {
        let map = get_direct_orbits(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN",
        );
        let path = find_full_path(&map, "YOU", "SAN");
        assert_eq!(path, vec!["K", "J", "E", "D", "I"]);
    }

    #[test]
    fn test_find_intersection() {
        let map = get_direct_orbits(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN",
        );
        let santa_path = path_to_target(&map, "SAN", "COM");
        let you_path = path_to_target(&map, "YOU", "COM");
        let common_point = find_intersection(&santa_path, &you_path);
        assert_eq!(common_point, Some("D"));
    }

    #[test]
    fn test_path_to_target() {
        let map = get_direct_orbits(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN",
        );
        let santa_path = path_to_target(&map, "SAN", "COM");
        let you_path = path_to_target(&map, "YOU", "COM");
        assert_eq!(santa_path, vec!["I", "D", "C", "B", "COM"]);
        assert_eq!(you_path, vec!["K", "J", "E", "D", "C", "B", "COM"]);
    }

    #[test]
    fn test_path_from_san_to_you() {
        let map = get_direct_orbits(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN",
        );
        let path = find_full_path(&map, "YOU", "SAN");
        assert_eq!(path, vec!["K", "J", "E", "D", "I"])
    }

    #[test]
    fn test_count_num_transfers() {
        let map = get_direct_orbits(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN",
        );
        let num_transfers = get_number_of_transfers(&map, "YOU", "SAN");
        assert_eq!(num_transfers, 4);
    }
}
