extern crate rusty_the_reindeer;

use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = count(contents.trim());
    let part2 = picky_count(contents.trim());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn count(contents: &str) -> usize {
    let mut generators = seed_generators(contents);
    (0..40_000_001).fold(0, |total, _| {
        let mut values = generators
            .iter_mut()
            .map(|g| g.next().unwrap())
            .map(lowest_16_bits);
        if values.next() == values.next() {
            total + 1
        } else {
            total
        }
    })
}

fn picky_count(contents: &str) -> usize {
    let mut seeds = contents.lines().map(|line| {
        line.split_whitespace()
            .last()
            .and_then(|word| usize::from_str(word).ok())
            .unwrap()
    });

    let mut generator_a = Generator::new(16_807, seeds.next().unwrap()).filter(|x| x % 4 == 0);
    let mut generator_b = Generator::new(48_271, seeds.next().unwrap()).filter(|x| x % 8 == 0);

    (0..5_000_001).fold(0, |total, _| {
        let a = lowest_16_bits(generator_a.next().unwrap());
        let b = lowest_16_bits(generator_b.next().unwrap());
        if a == b {
            total + 1
        } else {
            total
        }
    })
}

fn seed_generators(contents: &str) -> Vec<Generator> {
    let factors = [16_807, 48_271];
    let seeds = contents.lines().map(|line| {
        line.split_whitespace()
            .last()
            .and_then(|word| usize::from_str(word).ok())
            .unwrap()
    });

    factors
        .iter()
        .zip(seeds)
        .map(|(&factor, seed)| Generator::new(factor, seed))
        .collect()
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
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * self.factor) % Self::DIVISOR;
        Some(self.value)
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

    #[test]
    fn part2() {
        let input = "65\n8921";
        assert_eq!(309, picky_count(input));
    }
}
