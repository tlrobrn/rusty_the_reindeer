extern crate rusty_the_reindeer;

use std::str::FromStr;
use rusty_the_reindeer::KnotHash;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = once(contents.trim());
    let part2 = hash(contents.trim());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn once(contents: &str) -> u64 {
    let input: Vec<u8> = contents
        .split(',')
        .map(|s| u8::from_str(s).unwrap())
        .collect();
    KnotHash::default().round(&input).head()
}

fn hash(contents: &str) -> String {
    KnotHash::new(contents).dense()
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            52070,
            once("46,41,212,83,1,255,157,65,139,52,39,254,2,86,0,204")
        );
    }

    #[test]
    fn part2() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", hash(""));
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", hash("AoC 2017"));
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", hash("1,2,3"));
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", hash("1,2,4"));
    }
}
