extern crate rusty_the_reindeer;

use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = valid_passphrases(&contents);

    println!("Part 1: {}", part1);
}

fn valid_passphrases(contents: &str) -> usize {
    contents.lines().filter(is_valid).count()
}

fn is_valid(line: &&str) -> bool {
    let unique_word_count = HashSet::<&str>::from_iter(line.split_whitespace()).len();
    let word_count = line.split_whitespace().count();

    unique_word_count == word_count
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    #[test]
    fn part1() {
        assert!(is_valid(&"aa bb cc dd ee"));
        assert!(!is_valid(&"aa bb cc dd aa"));
        assert!(is_valid(&"aa bb cc dd aaa"));
    }
}
