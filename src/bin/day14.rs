extern crate rusty_the_reindeer;

use rusty_the_reindeer::KnotHash;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = count(contents.trim());

    println!("Part 1: {}", part1);
}

fn count(contents: &str) -> u32 {
    (0..128).map(|x| KnotHash::new(&format!("{}-{}", contents, x)).dense())
        .map(|hash| hash.chars().fold(0, |total, c| total + c.to_digit(16).map_or(0, |n| n.count_ones())))
        .sum()
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "flqrgnkx";
        assert_eq!(8108, count(input));
    }
}
