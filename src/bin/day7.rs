extern crate rusty_the_reindeer;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = base(&contents);
    let part2 = balanced_weight(&contents);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn base(contents: &str) -> &str {
    let lines: Vec<&str> = contents.lines().collect();

    let descendants: HashSet<&str> = HashSet::from_iter(
        lines
            .iter()
            .filter(|line| line.contains("->"))
            .flat_map(|line| {
                line.split("->")
                    .last()
                    .unwrap()
                    .split(',')
                    .map(|node| node.trim())
            }),
    );

    lines
        .iter()
        .map(|line| line.split_whitespace().next().unwrap())
        .find(|node| !descendants.contains(node))
        .expect("Root node not found")
}

fn balanced_weight(contents: &str) -> u64 {
    let tower = Tower::from_input(contents);
    let root = &tower.nodes[base(contents)];
    let unbalanced_node = find_unbalance(&tower, root);
    let target_weight = tower
        .parent(unbalanced_node.name)
        .descendants
        .iter()
        .find(|&&descendant_name| descendant_name != unbalanced_node.name)
        .and_then(|&sibling_name| tower.weight(sibling_name))
        .unwrap();
    let unbalanced_weight = tower.weight(unbalanced_node.name).unwrap();

    ((unbalanced_node.weight as i64) + ((target_weight as i64) - (unbalanced_weight as i64))) as u64
}

fn find_unbalance<'a>(tower: &'a Tower, node: &'a Node<'a>) -> &'a Node<'a> {
    if node.descendants.is_empty() {
        return node;
    }

    let mut weights: Vec<(&str, u64)> = node.descendants
        .iter()
        .map(|&descendant_name| {
            (
                descendant_name,
                tower
                    .weight(descendant_name)
                    .expect("Unable to get weight for tower"),
            )
        })
        .collect();
    if weights[1..]
        .iter()
        .all(|&(_, weight)| weight == weights[0].1)
    {
        return node;
    }

    weights.sort_by_key(|&(_, weight)| weight);
    if weights[0].1 == weights[1].1 {
        find_unbalance(tower, &tower.nodes[weights.last().unwrap().0])
    } else {
        find_unbalance(tower, &tower.nodes[weights[0].0])
    }
}

struct Node<'a> {
    name: &'a str,
    weight: u64,
    descendants: Vec<&'a str>,
}

struct Tower<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> Tower<'a> {
    fn from_input(contents: &'a str) -> Self {
        let mut nodes: HashMap<&str, Node> = HashMap::new();

        for mut words in contents.lines().map(|line| line.split_whitespace()) {
            let name = words.next().unwrap();

            let weight = words
                .next()
                .map(|w| {
                    u64::from_str(w.trim_left_matches('(').trim_right_matches(')')).unwrap_or(0)
                })
                .expect("Could not parse weight");

            let descendants: Vec<&str> = if let Some("->") = words.next() {
                words.map(|w| w.trim_matches(',')).collect()
            } else {
                Vec::new()
            };

            let node = Node {
                name,
                weight,
                descendants,
            };

            nodes.insert(name, node);
        }

        Self { nodes }
    }

    fn weight(&self, node_name: &'a str) -> Option<u64> {
        self.nodes.get(node_name).map(|node| {
            node.weight + node.descendants.iter().fold(0, |total, descendant_name| {
                total + self.weight(descendant_name).unwrap_or(0)
            })
        })
    }

    fn parent(&self, node_name: &'a str) -> &Node<'a> {
        self.nodes
            .values()
            .find(|&node| node.descendants.iter().any(|&name| name == node_name))
            .expect("could not find parent")
    }
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

    #[test]
    fn part2() {
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
        assert_eq!(60, balanced_weight(input));
    }
}
