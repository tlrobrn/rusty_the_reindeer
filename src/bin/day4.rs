extern crate rusty_the_reindeer;

use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = valid_passphrases(&contents);
    let part2 = valid_passphrases2(&contents);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn valid_passphrases(contents: &str) -> usize {
    contents.lines().filter(is_valid).count()
}

fn valid_passphrases2(contents: &str) -> usize {
    contents.lines().filter(is_valid2).count()
}

fn is_valid(line: &&str) -> bool {
    let unique_word_count = HashSet::<&str>::from_iter(line.split_whitespace()).len();
    let word_count = line.split_whitespace().count();

    unique_word_count == word_count
}

fn is_valid2(line: &&str) -> bool {
    let sorted_chars: Vec<String> = line.split_whitespace()
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            chars
        })
        .map(String::from_iter)
        .collect();

    let unique_word_count = HashSet::<&String>::from_iter(sorted_chars.iter()).len();
    let word_count = sorted_chars.len();

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

    #[test]
    fn part2() {
        assert!(is_valid2(&"abcde fghij"));
        assert!(!is_valid2(&"abcde xyz ecdab"));
        assert!(is_valid2(&"a ab abc abd abf abj"));
        assert!(is_valid2(&"iiii oiii ooii oooi oooo"));
        assert!(!is_valid2(&"oiii ioii iioi iiio"));
    }
}
