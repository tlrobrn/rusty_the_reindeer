extern crate rusty_the_reindeer;

use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = count(contents.trim());

    println!("Part 1: {}", part1);
}

fn count(contents: &str) -> usize {
    let factors = [16_807, 48_271];
    let seeds = contents.lines().map(|line| {
        line.split_whitespace()
            .last()
            .and_then(|word| usize::from_str(word).ok())
            .unwrap()
    });
    let mut generators: Vec<Generator> = factors
        .iter()
        .zip(seeds)
        .map(|(&factor, seed)| Generator::new(factor, seed))
        .collect();
    (0..40_000_001).fold(0, |total, _| {
        let mut values = generators
            .iter_mut()
            .map(|g| g.next_value())
            .map(lowest_16_bits);
        if values.next() == values.next() {
            total + 1
        } else {
            total
        }
    })
}

fn lowest_16_bits(n: usize) -> u16 {
    let mask = (2u32.pow(16) - 1) as usize;
    (n & mask) as u16
}

struct Generator {
    factor: usize,
    value: usize,
}

impl Generator {
    const DIVISOR: usize = 2_147_483_647;

    pub fn new(factor: usize, seed: usize) -> Self {
        Generator {
            factor,
            value: seed,
        }
    }

    pub fn next_value(&mut self) -> usize {
        self.value = (self.value * self.factor) % Self::DIVISOR;
        self.value
    }
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "65\n8921";
        assert_eq!(588, count(input));
    }
}
