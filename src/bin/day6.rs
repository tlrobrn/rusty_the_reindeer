extern crate rusty_the_reindeer;

use std::collections::{BinaryHeap, HashSet};
use std::iter::FromIterator;
use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = reallocate(&contents);

    println!("Part 1: {}", part1);
}

fn reallocate(contents: &str) -> usize {
    let mut banks: Vec<usize> = contents.trim()
        .split_whitespace()
        .map(|word| usize::from_str(word).unwrap())
        .collect();

    let mut seen: HashSet<Vec<usize>> = HashSet::new();
    seen.insert(banks.to_vec());

    redistribute(&mut banks, &mut seen, 0)
}


fn redistribute(banks: &mut[usize], seen: &mut HashSet<Vec<usize>>, steps: usize) -> usize {
    let length = banks.len();
    let sorted_banks: BinaryHeap<(usize, usize, usize)> = BinaryHeap::from_iter(
        banks.iter().enumerate().map(|(i, &v)| (v, length - i, i))
    );

    let &(blocks, _, index) = sorted_banks.peek().unwrap();
    banks[index] = 0;

    let mut index = index + 1;
    for _ in 0..blocks {
        banks[index % length] += 1;
        index += 1;
    }

    if seen.contains(banks) {
        steps + 1
    } else {
        seen.insert(banks.to_vec());
        redistribute(banks, seen, steps + 1)
    }
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(5, reallocate("0 2 7 0"));
    }
}
