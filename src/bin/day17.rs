extern crate rusty_the_reindeer;

use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = spin(contents.trim());
    let part2 = spin_zero(contents.trim());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
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

fn spin_zero(contents: &str) -> usize {
    let step = usize::from_str(contents).unwrap();
    (1..50_000_000)
        .fold(
            (0, 0, 0),
            |(last_insert_position, zero_position, value_after_zero), x| {
                let next_insert_position = (last_insert_position + step) % x + 1;
                let value_after_zero = if next_insert_position == zero_position + 1 {
                    x
                } else {
                    value_after_zero
                };
                let zero_position = if next_insert_position == zero_position {
                    zero_position + 1
                } else {
                    zero_position
                };
                (next_insert_position, zero_position, value_after_zero)
            },
        )
        .2
}

#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(1282, spin("335"));
    }

    #[test]
    fn part2() {
        assert_eq!(27650600, spin_zero("335"));
    }
}
