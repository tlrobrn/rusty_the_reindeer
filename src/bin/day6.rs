extern crate rusty_the_reindeer;

use std::collections::{BinaryHeap, HashMap};
use std::iter::FromIterator;
use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let (part1, part2) = reallocate(&contents);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn reallocate(contents: &str) -> (usize, usize) {
    let mut banks: Vec<usize> = contents.trim()
        .split_whitespace()
        .map(|word| usize::from_str(word).unwrap())
        .collect();

    let mut seen: HashMap<Vec<usize>, usize> = HashMap::new();
    seen.insert(banks.to_vec(), 0);

    redistribute(&mut banks, &mut seen, 1)
}

fn redistribute(banks: &mut[usize], seen: &mut HashMap<Vec<usize>, usize>, step: usize) -> (usize, usize) {
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

    if let Some(&original_step) = seen.get(banks) {
        (step, step - original_step)
    } else {
        seen.insert(banks.to_vec(), step);
        redistribute(banks, seen, step + 1)
    }
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn parts1and2() {
        assert_eq!((5, 4), reallocate("0 2 7 0"));
    }
}
