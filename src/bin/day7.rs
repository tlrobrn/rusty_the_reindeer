extern crate rusty_the_reindeer;

use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = base(&contents);

    println!("Part 1: {}", part1);
}

fn base(contents: &str) -> &str {
    let lines: Vec<&str> = contents.lines().collect();

    let descendants: HashSet<&str> = HashSet::from_iter(
        lines
            .iter()
            .filter(|line| line.contains("->"))
            .flat_map(|line| line.split("->").last().unwrap().split(',').map(|node| node.trim()))
    );

    lines.iter()
        .map(|line| line.split_whitespace().next().unwrap())
        .find(|node| !descendants.contains(node))
        .expect("Root node not found")
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
";
        assert_eq!("tknk", base(input));
    }
}
