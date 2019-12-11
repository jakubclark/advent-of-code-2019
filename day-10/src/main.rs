use std::collections::HashSet;
use std::fs::read_to_string;

type Cell = (usize, usize);

/// Get the approximate angle between `a` and `b`
fn get_angle(a: Cell, b: Cell) -> i64 {
    let x_dist = b.0 as f64 - a.0 as f64;
    let y_dist = a.1 as f64 - b.1 as f64;

    (x_dist.atan2(y_dist) * 57.3 * 100_000.0) as i64
}

/// Count visible asteroids, based on the unique angle between `origin` and each asteroid
fn count_visible_asteroids(asteroids: &[Cell], origin: Cell) -> i64 {
    let cells: HashSet<_> = asteroids
        .iter()
        .filter(|&&a| a != origin)
        .map(|&a| get_angle(origin, a))
        .collect();
    cells.len() as i64
}

fn find_max(asteroids: &[Cell]) -> (Cell, i64) {
    asteroids
        .iter()
        .map(|&a| (a, count_visible_asteroids(asteroids, a)))
        .max_by_key(|x| x.1)
        .expect("Failed to find asteroid with max visible asteroids")
}

fn get_cells(input: String) -> Vec<Cell> {
    let mut cells = Vec::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                cells.push((x, y));
            }
        })
    });
    cells
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to open input.txt");
    let cells = get_cells(input);

    //    let max_x = set.iter().max_by_key(|&&entry| entry.0).expect("foo").0;
    //    let max_y = set.iter().max_by_key(|&&entry| entry.1).expect("foo").1;

    let (_, count) = find_max(&cells);

    println!("Solution for part 1 = {}", count);
}

#[cfg(test)]
mod tests {
    use crate::{find_max, get_cells};

    #[test]
    fn test1() {
        let input = String::from(".#..#\n.....\n#####\n....#\n...##");
        let cells = get_cells(input);
        let (destination, count) = find_max(&cells);
        assert_eq!(destination, (3, 4));
        assert_eq!(count, 8);
    }

    #[test]
    fn test2() {
        let input = String::from("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####");
        let cells = get_cells(input);
        let (destination, count) = find_max(&cells);
        assert_eq!(destination, (5, 8));
        assert_eq!(count, 33);
    }

    #[test]
    fn test3() {
        let input = String::from("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.");
        let cells = get_cells(input);
        let (destination, count) = find_max(&cells);
        assert_eq!(destination, (1, 2));
        assert_eq!(count, 35);
    }

    #[test]
    fn test4() {
        let input = String::from(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##");
        let cells = get_cells(input);
        let (destination, count) = find_max(&cells);
        assert_eq!(destination, (11, 13));
        assert_eq!(count, 210);
    }
}
