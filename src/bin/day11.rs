extern crate rusty_the_reindeer;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = steps(contents.trim());

    println!("Part 1: {}", part1);
}

fn steps(_contents: &str) -> u64 {
    0
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(3, steps("ne,ne,ne"));
        assert_eq!(0, steps("ne,ne,sw,sw"));
        assert_eq!(2, steps("ne,ne,s,s"));
        assert_eq!(3, steps("se,sw,se,sw,sw"));
    }
}
