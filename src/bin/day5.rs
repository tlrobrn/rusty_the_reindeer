extern crate rusty_the_reindeer;

use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = escape(&contents, i64::max_value());
    let part2 = escape(&contents, 2);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn escape(contents: &str, cutoff: i64) -> usize {
    let mut offsets: Vec<i64> = contents
        .lines()
        .map(|line| i64::from_str(line.trim()).unwrap())
        .collect();

    let mut position = 0;
    let mut jumps = 0;

    while position < offsets.len() {
        let offset = offsets[position];
        let delta = if offset <= cutoff { 1 } else { -1 };

        offsets[position] = offset + delta;
        position = (position as i64 + offset) as usize;
        jumps += 1;
    }
    jumps
}
