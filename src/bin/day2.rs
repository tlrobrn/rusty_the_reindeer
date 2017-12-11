extern crate rusty_the_reindeer;

use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = checksum(&contents);
    println!("Part 1: {}", part1);
}

fn checksum(contents: &str) -> u64 {
    contents
        .lines()
        .map(min_max)
        .fold(0, |total, mm| total + (mm.max - mm.min))
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

#[cfg(test)]
mod day2_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(18, checksum("5 1 9 5\n7 5 3\n2 4 6 8"));
    }
}
