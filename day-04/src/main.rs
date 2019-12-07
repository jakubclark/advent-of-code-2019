use std::collections::HashMap;

fn num_to_digits(num: u32) -> Vec<u32> {
    let s = num.to_string();
    s.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn always_increasing(digits: &[u32]) -> bool {
    let mut prev = u32::min_value();
    digits.iter().all(|n| {
        if *n < prev {
            return false;
        }
        prev = *n;
        true
    })
}

fn at_least_once(digits: &[u32]) -> bool {
    let mut count_map = HashMap::new();
    for digit in digits {
        let count = count_map.entry(digit).or_insert(0);
        *count += 1;
    }
    count_map.values().any(|n| *n == 2)
}

fn get_possibilities(start: u32, finish: u32) -> Vec<u32> {
    println!("start={} | finish={}", start, finish);
    (start..=finish)
        .filter(|n| {
            println!("{}", n);
            let digits = num_to_digits(*n);
            always_increasing(&digits) && at_least_once(&digits)
        })
        .collect()
}

fn main() {
    let possibilities: Vec<u32> = get_possibilities(254_032, 789_860);
    println!("Solution for part 2 = {}", possibilities.len());
}

#[cfg(test)]
mod tests {
    use crate::get_possibilities;

    #[test]
    fn test_part2() {
        let possibilities: Vec<u32> = get_possibilities(254_032, 789_860);
        assert_eq!(possibilities.len(), 670);
    }
}
