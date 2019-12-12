use std::collections::{BTreeMap, HashSet};
use std::fs::read_to_string;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Cell {
    x: usize,
    y: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Get the approximate angle between `a` and `b`
fn get_angle(a: Cell, b: Cell) -> i64 {
    let x_dist = b.x as f64 - a.x as f64;
    let y_dist = a.y as f64 - b.y as f64;

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
                cells.push(Cell::new(x, y));
            }
        })
    });
    cells
}

// Get the vaporize order of `asteroids`, from `origin`
fn vaporize(asteroids: &[Cell], origin: Cell) -> Vec<&Cell> {
    let mut asteroids = asteroids
        .iter()
        .map(|a| {
            let angle = get_angle(origin, *a);
            (angle, a)
        })
        .collect::<Vec<(i64, &Cell)>>();

    asteroids.sort_by(|a, b| a.0.cmp(&b.0));

    let unique_angles = asteroids.iter().map(|a| a.0).collect::<HashSet<i64>>();

    // Maps an angle, to a list of asteroids, at that angle
    let mut map: BTreeMap<i64, Vec<&Cell>> = BTreeMap::new();

    // Group asteroids, by angle
    for uniq in &unique_angles {
        let in_path = asteroids
            .iter()
            .filter_map(|(angle, asteroid)| if angle == uniq { Some(*asteroid) } else { None })
            .collect::<Vec<&Cell>>();
        map.insert(*uniq, in_path);
    }

    let mut angles: Vec<i64> = unique_angles.into_iter().collect();
    angles.sort();

    // Reorder, so that first entry is >= 0
    while let Some(&angle) = angles.get(0) {
        if angle < 0 {
            angles.push(angle);
            angles.remove(0);
        } else {
            break;
        }
    }

    let mut vaporize_order = vec![];

    while !map.is_empty() {
        angles.iter().for_each(|angle| {
            if let Some(asteroids) = map.get_mut(angle) {
                if asteroids.is_empty() {
                    // We've processed all the asteroids at this angle
                    map.remove(angle);
                    return;
                }

                // Vaporize the asteroid!
                let asteroid = asteroids.remove(0);
                vaporize_order.push(asteroid);
            }
        });
    }

    vaporize_order
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to open input.txt");
    let cells = get_cells(input);

    let (origin, count) = find_max(&cells);

    let order = vaporize(&cells, origin);
    let entry = order[199];
    let res = entry.x * 100 + entry.y;

    println!("Solution for part 1 = {}", count);
    println!("Solution for part 2 = {}", res);
}

#[cfg(test)]
mod tests {
    use crate::{find_max, get_cells, vaporize, Cell};

    #[test]
    fn test1() {
        let input = String::from(".#..#\n.....\n#####\n....#\n...##");
        let cells = get_cells(input);
        let (destination, count) = find_max(&cells);
        assert_eq!(destination, Cell::new(3, 4));
        assert_eq!(count, 8);
    }

    #[test]
    fn test2() {
        let input = String::from("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####");
        let cells = get_cells(input);
        let (destination, count) = find_max(&cells);
        assert_eq!(destination, Cell::new(5, 8));
        assert_eq!(count, 33);
    }

    #[test]
    fn test3() {
        let input = String::from("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.");
        let cells = get_cells(input);
        let (destination, count) = find_max(&cells);
        assert_eq!(destination, Cell::new(1, 2));
        assert_eq!(count, 35);
    }

    #[test]
    fn test4() {
        let input = String::from(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##");
        let cells = get_cells(input);
        let (origin, count) = find_max(&cells);
        assert_eq!(origin, Cell::new(11, 13));
        assert_eq!(count, 210);

        let order = vaporize(&cells, origin);
        assert_eq!(*order[199], Cell::new(8, 2));
    }
}
