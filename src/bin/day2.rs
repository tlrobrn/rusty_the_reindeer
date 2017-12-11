extern crate rusty_the_reindeer;

use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = checksum(&contents);
    let part2 = checksum2(&contents);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn checksum(contents: &str) -> u64 {
    contents
        .lines()
        .map(min_max)
        .fold(0, |total, mm| total + (mm.max - mm.min))
}

fn checksum2(contents: &str) -> u64 {
    contents
        .lines()
        .map(divided_result)
        .sum()
}

#[derive(Debug)]
struct MinMax {
    min: u64,
    max: u64,
}

impl Default for MinMax {
    fn default() -> Self {
        MinMax {
            min: u64::max_value(),
            max: u64::min_value(),
        }
    }
}

fn min_max(line: &str) -> MinMax {
    line.split_whitespace().fold(MinMax::default(), |mm, word| {
        let number = u64::from_str(word).unwrap();
        MinMax {
            min: number.min(mm.min),
            max: number.max(mm.max),
        }
    })
}

fn divided_result(line: &str) -> u64 {
    let numbers: Vec<u64> = line
        .split_whitespace()
        .map(|word| u64::from_str(word).unwrap())
        .collect();

    for number in &numbers {
        if let Some(n) = numbers.iter().find(|&x| x != number && (number % x == 0 || x % number == 0)) {
            return (number / n).max(n / number);
        }
    }

    1
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(18, checksum("5 1 9 5\n7 5 3\n2 4 6 8"));
    }

    #[test]
    fn part2() {
        assert_eq!(9, checksum2("5 9 2 8\n 9 4 7 3\n3 8 6 5"));
    }
}
