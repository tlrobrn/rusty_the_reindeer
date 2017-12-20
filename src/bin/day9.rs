extern crate rusty_the_reindeer;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let (part1, part2) = score(&contents);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn score(contents: &str) -> (u64, u64) {
    let (_, _, _, score, count) = contents.chars().fold((false, false, 0, 0, 0), |(garbage, skip, depth, score, count), c| {
        if skip {
            (garbage, false, depth, score, count)
        } else {
            match c {
                '!' if garbage => (garbage, true, depth, score, count),
                '>' if garbage => (false, skip, depth, score, count),
                '{' if !garbage => (false, false, depth + 1, score, count),
                '}' if !garbage && depth > 0 => (garbage, skip, depth - 1, score + depth, count),
                '<' if !garbage => (true, skip, depth, score, count),
                _ if garbage => (garbage, skip, depth, score, count + 1),
                _ => (garbage, skip, depth, score, count)
            }
        }
    });

    (score, count)
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(1, score("{}").0);
        assert_eq!(6, score("{{{}}}").0);
        assert_eq!(5, score("{{},{}}").0);
        assert_eq!(16, score("{{{},{},{{}}}}").0);
        assert_eq!(1, score("{<a>,<a>,<a>,<a>}").0);
        assert_eq!(9, score("{{<ab>},{<ab>},{<ab>},{<ab>}}").0);
        assert_eq!(9, score("{{<!!>},{<!!>},{<!!>},{<!!>}}").0);
        assert_eq!(3, score("{{<a!>},{<a!>},{<a!>},{<ab>}}").0);
    }

    #[test]
    fn part2() {
        assert_eq!(0, score("<>").1);
        assert_eq!(17, score("<random characters>").1);
        assert_eq!(3, score("<<<<>").1);
        assert_eq!(2, score("<{!>}>").1);
        assert_eq!(0, score("<!!>").1);
        assert_eq!(0, score("<!!!>>").1);
        assert_eq!(10, score("<{o\"i!a,<{i<a>").1);
    }
}
