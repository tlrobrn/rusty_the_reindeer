#![feature(slice_rotate)]

extern crate rusty_the_reindeer;
use std::iter::FromIterator;
use std::str::FromStr;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = dance(contents.trim(), 16, 1);
    let part2 = dance(contents.trim(), 16, 1_000_000_000);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn dance(contents: &str, len: usize, count: usize) -> String {
    let instructions: Vec<_> = contents.split(',').collect();
    let mut dancers = characters(len);
    let mut positions = vec![String::from_iter(dancers.iter())];

    for _ in 0..count {
        for instruction in &instructions {
            spin(&mut dancers, instruction);
            exchange(&mut dancers, instruction);
            partner(&mut dancers, instruction);
        }
        let position = String::from_iter(dancers.iter());
        if positions.iter().any(|p| **p == position) {
            return positions[count % positions.len()].clone();
        }
        positions.push(position);
    }

    positions.last().unwrap().clone()
}

fn characters(len: usize) -> Vec<char> {
    LETTERS.chars().take(len).collect()
}

fn spin(dancers: &mut [char], instruction: &str) {
    if instruction.starts_with('s') {
        let count: usize = instruction[1..].parse().expect("Failed to spin");
        let mid = dancers.len() - count;
        dancers.rotate(mid);
    }
}

fn exchange(dancers: &mut [char], instruction: &str) {
    if instruction.starts_with('x') {
        let mut positions = instruction[1..]
            .split('/')
            .map(|s| usize::from_str(s).expect("failed to convert value"));
        let a = positions.next().expect("failed to get position");
        let b = positions.next().expect("failed to get position");
        dancers.swap(a, b);
    }
}

fn partner(dancers: &mut [char], instruction: &str) {
    if instruction.starts_with('p') {
        let message = format!("Could not handle {}\ndancers: {:?}", instruction, dancers);
        let mut positions = instruction[1..].split('/').filter_map(|s| s.chars().next());
        let a = positions.next().expect("failed to get partner");
        let b = positions.next().expect("failed to get partner");
        let a = dancers.iter().position(|&c| c == a).expect(&message);
        let b = dancers.iter().position(|&c| c == b).expect(&message);
        dancers.swap(a, b);
    }
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "s1,x3/4,pe/b";
        assert_eq!("baedc", dance(input, 5, 1));
    }

    #[test]
    fn test_spin() {
        let mut dancers = ['a', 'b', 'c', 'd', 'e'];
        spin(&mut dancers, "s3");
        assert_eq!(['c', 'd', 'e', 'a', 'b'], dancers);
        spin(&mut dancers, "s1");
        assert_eq!(['b', 'c', 'd', 'e', 'a'], dancers);
        spin(&mut dancers, "x3/4");
        assert_eq!(['b', 'c', 'd', 'e', 'a'], dancers);
    }

    #[test]
    fn test_exchange() {
        let mut dancers = ['a', 'b', 'c', 'd', 'e'];
        exchange(&mut dancers, "x3/4");
        assert_eq!(['a', 'b', 'c', 'e', 'd'], dancers);
        exchange(&mut dancers, "x1/3");
        assert_eq!(['a', 'e', 'c', 'b', 'd'], dancers);
        exchange(&mut dancers, "s4");
        assert_eq!(['a', 'e', 'c', 'b', 'd'], dancers);
    }

    #[test]
    fn test_partner() {
        let mut dancers = ['a', 'b', 'c', 'd', 'e'];
        partner(&mut dancers, "pd/e");
        assert_eq!(['a', 'b', 'c', 'e', 'd'], dancers);
        partner(&mut dancers, "pe/b");
        assert_eq!(['a', 'e', 'c', 'b', 'd'], dancers);
        partner(&mut dancers, "s4");
        assert_eq!(['a', 'e', 'c', 'b', 'd'], dancers);
    }
}
