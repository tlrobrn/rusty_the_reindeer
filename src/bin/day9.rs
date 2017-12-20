extern crate rusty_the_reindeer;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = score(&contents);

    println!("Part 1: {}", part1);
}

fn score(_contents: &str) -> u64 {
    0
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
