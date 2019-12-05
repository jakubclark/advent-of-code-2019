use std::fs::File;
use std::io::{prelude::BufRead, BufReader, Error as IoError};

fn fuel_for_mass(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}

fn fuel_for_mass_and_fuel(mass: f64) -> f64 {
    let fuel_mass = (mass / 3.0).floor() - 2.0;
    if fuel_mass > 0.0 {
        fuel_mass + fuel_for_mass_and_fuel(fuel_mass)
    } else {
        0.0
    }
}

fn calculate_fuel<F>(buf: BufReader<File>, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    let required_fuel: f64 = buf
        .lines()
        .map(|line| {
            let line = line.expect("Failed to get line");
            let mass = line.parse::<f64>().expect("Failed to convert to f64");
            f(mass)
        })
        .sum();
    required_fuel
}

fn part1() -> Result<(), IoError> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let required_fuel = calculate_fuel(reader, crate::fuel_for_mass);
    println!("Base Required Fuel = {}", required_fuel);
    Ok(())
}

fn part2() -> Result<(), IoError> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let required_fuel = calculate_fuel(reader, crate::fuel_for_mass_and_fuel);
    println!("Total Required Fuel = {}", required_fuel);
    Ok(())
}

fn main() -> Result<(), IoError> {
    part1()?;
    part2()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{fuel_for_mass, fuel_for_mass_and_fuel};

    #[test]
    fn test1() {
        assert_eq!(fuel_for_mass(12.0), 2.0);
    }

    #[test]
    fn test2() {
        assert_eq!(fuel_for_mass(14.0), 2.0);
    }

    #[test]
    fn test3() {
        assert_eq!(fuel_for_mass(1969.0), 654.0);
    }

    #[test]
    fn test4() {
        assert_eq!(fuel_for_mass(100756.0), 33583.0);
    }

    #[test]
    fn test5() {
        assert_eq!(fuel_for_mass_and_fuel(14.0), 2.0);
    }

    #[test]
    fn test6() {
        assert_eq!(fuel_for_mass_and_fuel(1969.0), 966.0);
    }

    #[test]
    fn test7() {
        assert_eq!(fuel_for_mass_and_fuel(100756.0), 50346.0);
    }
}
