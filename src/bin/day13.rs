extern crate rusty_the_reindeer;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = severity(contents.trim());

    println!("Part 1: {}", part1);
}

fn severity(_contents: &str) -> usize {
    0
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(24, severity(input));
    }
}
