extern crate rusty_the_reindeer;

use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = escape(&contents);

    println!("Part 1: {}", part1);
}

fn escape(contents: &str) -> usize {
    let mut instructions: Vec<i64> = contents
        .lines()
        .map(|line| i64::from_str(line.trim()).unwrap())
        .collect();

    let mut position = 0;
    let mut jumps = 0;

    while position < instructions.len() {
        let instruction = instructions[position];
        instructions[position] = instruction + 1;
        position = (position as i64 + instruction) as usize;
        jumps += 1;
    }
    jumps
}
