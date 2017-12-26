extern crate rusty_the_reindeer;

use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = spin(contents.trim());

    println!("Part 1: {}", part1);
}

fn spin(contents: &str) -> usize {
    let step = usize::from_str(contents).unwrap();
    let mut state = vec![0];
    let mut position = 0;

    for x in 1..2018 {
        position = (position + step) % state.len() + 1;
        state.insert(position, x);
    }

    state[position + 1]
}

#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(1282, spin("335"));
    }
}
