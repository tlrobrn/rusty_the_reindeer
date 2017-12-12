extern crate rusty_the_reindeer;

use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = step_counter(contents.trim());

    println!("Part 1: {}", part1);
}

fn step_counter(contents: &str) -> i64 {
    let number = i64::from_str(contents).unwrap();
    if number == 1 { return 0; }

    let ring = ring_of(number);
    let mut coords = starting_coords_of(ring);

    for n in start_of(ring)..end_of(ring) {
        if n == number {
            let (x, y) = coords;
            return x.abs() + y.abs();
        }

        coords = match coords {
            (x, y) if x == ring && y == ring  => (ring - 1, ring),
            (x, y) if x == -ring && y == ring => (-ring, ring - 1),
            (x, y) if x == -ring && y == -ring => (-ring + 1, -ring),
            (x, y) if x == ring && y == -ring => (ring, -ring + 1),
            (x, y) if x == ring => (x, y + 1),
            (x, y) if y == ring => (x - 1, y),
            (x, y) if x == -ring => (x, y - 1),
            (x, y) if y == -ring => (x + 1, y),
            _ => panic!("Invalid ring coordinates"),
        }
    }

    0
}

fn ring_of(number: i64) -> i64 {
    let float = number as f64;
    ((float.sqrt() - 1.0) / 2.0).ceil() as i64
}

fn start_of(ring: i64) -> i64 {
    if ring == 0 {
        1
    } else {
        (width_of(ring - 1)).pow(2) + 1
    }
}

fn end_of(ring: i64) -> i64 {
    start_of(ring + 1)
}

fn width_of(ring: i64) -> i64 {
    ring * 2 + 1
}

fn starting_coords_of(ring: i64) -> (i64, i64) {
    (ring, -ring + 1)
}


#[cfg(test)]
mod day2_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(0, step_counter("1"));
        assert_eq!(3, step_counter("12"));
        assert_eq!(2, step_counter("23"));
        assert_eq!(31, step_counter("1024"));
    }
}
