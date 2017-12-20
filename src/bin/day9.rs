extern crate rusty_the_reindeer;

use std::str::Chars;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = score(&contents);

    println!("Part 1: {}", part1);
}

fn score(contents: &str) -> u64 {
    let mut chars = contents.chars();
    score_loop(&mut chars, 0)
}

fn score_loop(chars: &mut Chars, score: u64) -> u64 {
    match chars.next() {
        None => score,
        Some('{') => score_group(chars, score, 0),
        Some('<') => {
            collect_garbage(chars);
            score_loop(chars, score)
        },
        _ => score_loop(chars, score),
    }
}

fn score_group(chars: &mut Chars, score: u64, depth: u64) -> u64 {
    match chars.next() {
        None => score,
        Some('}') if depth > 0 => score_group(chars, score + depth + 1, depth - 1),
        Some('}') => score_loop(chars, score + 1),
        Some('{') => score_group(chars, score, depth + 1),
        Some('<') => {
            collect_garbage(chars);
            score_group(chars, score, depth)
        },
        _ => score_group(chars, score, depth),
    }
}

fn collect_garbage(chars: &mut Chars) {
    match chars.next() {
        None | Some('>') => (),
        Some('!') => {
            chars.next();
            collect_garbage(chars)
        },
        _ => collect_garbage(chars),
    }
}


#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(1, score("{}"));
        assert_eq!(6, score("{{{}}}"));
        assert_eq!(5, score("{{},{}}"));
        assert_eq!(16, score("{{{},{},{{}}}}"));
        assert_eq!(1, score("{<a>,<a>,<a>,<a>}"));
        assert_eq!(9, score("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
        assert_eq!(9, score("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
        assert_eq!(3, score("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
    }
}
