extern crate rusty_the_reindeer;

use std::str::Chars;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let (part1, part2) = score(&contents);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn score(contents: &str) -> (u64, u64) {
    let mut chars = contents.chars();
    score_loop(&mut chars, 0, 0, 0)
}

fn score_loop(chars: &mut Chars, depth: u64, score: u64, count: u64) -> (u64, u64) {
    match chars.next() {
        None => (score, count),
        Some('{') => score_loop(chars, depth + 1, score, count),
        Some('}') if depth > 0 => score_loop(chars, depth - 1, score + depth, count),
        Some('<') => count_garbage(chars, depth, score, count),
        _ => score_loop(chars, depth, score, count),
    }
}

fn count_garbage(chars: &mut Chars, depth: u64, score: u64, count: u64) -> (u64, u64) {
    match chars.next() {
        None => (score, count),
        Some('!') => {
            chars.next();
            count_garbage(chars, depth, score, count)
        },
        Some('>') => score_loop(chars, depth, score, count),
        _ => count_garbage(chars, depth, score, count + 1),
    }
}


#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(1, score("{}").0);
        assert_eq!(6, score("{{{}}}").0);
        assert_eq!(5, score("{{},{}}").0);
        assert_eq!(16, score("{{{},{},{{}}}}").0);
        assert_eq!(1, score("{<a>,<a>,<a>,<a>}").0);
        assert_eq!(9, score("{{<ab>},{<ab>},{<ab>},{<ab>}}").0);
        assert_eq!(9, score("{{<!!>},{<!!>},{<!!>},{<!!>}}").0);
        assert_eq!(3, score("{{<a!>},{<a!>},{<a!>},{<ab>}}").0);
    }

    #[test]
    fn part2() {
        assert_eq!(0, score("<>").1);
        assert_eq!(17, score("<random characters>").1);
        assert_eq!(3, score("<<<<>").1);
        assert_eq!(2, score("<{!>}>").1);
        assert_eq!(0, score("<!!>").1);
        assert_eq!(0, score("<!!!>>").1);
        assert_eq!(10, score("<{o\"i!a,<{i<a>").1);
    }
}
