extern crate rusty_the_reindeer;

use std::str::FromStr;
use std::collections::HashSet;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = count(contents.trim());
    let part2 = count_groups(contents.trim());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn count(contents: &str) -> usize {
    Graph::parse(contents).group(0).len()
}

fn count_groups(contents: &str) -> usize {
    let mut seen = HashSet::new();
    let graph = Graph::parse(contents);
    
    graph.nodes().fold(0, |total, node| {
        if !seen.contains(&node) {
            for &node in &graph.group(node) {
                seen.insert(node);
            }
            total + 1
        }
        else {
            total
        }
    })
}

#[derive(Default)]
struct Graph {
    nodes: Vec<Vec<usize>>,
}

impl Graph {
    pub fn parse(input: &str) -> Self {
        let mut graph = Self::default();
        for line in input.lines() {
            let edges: Vec<usize> = line.split("<->")
                .last()
                .unwrap()
                .split(", ")
                .map(|s| usize::from_str(s.trim()).unwrap())
                .collect();

            graph.nodes.push(edges)
        }

        graph
    }

    pub fn group(&self, node: usize) -> HashSet<usize> {
        let mut mates = HashSet::new();
        mates.insert(node);
        self.build_group(node, &mut mates);
        mates
    }

    pub fn nodes(&self) -> std::ops::Range<usize> {
        0..self.nodes.len()
    }

    fn build_group(&self, node: usize, group: &mut HashSet<usize>) {
        for &mate in &self.nodes[node] {
            if !group.contains(&mate) {
                group.insert(mate);
                self.build_group(mate, group);
            }
        }
    }
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        assert_eq!(6, count(input));
    }

    #[test]
    fn part2() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        assert_eq!(2, count_groups(input));
    }
}
