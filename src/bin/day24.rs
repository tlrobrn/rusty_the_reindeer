extern crate rusty_the_reindeer;

use std::fmt;
use std::str::FromStr;
use std::collections::HashSet;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = max_strength(&contents);
    println!("Part 1: {}", part1);
}

fn max_strength(contents: &str) -> usize {
    let graph = Graph::from_str(contents).unwrap();
    graph.strongest_path()
}

struct Graph {
    nodes: Vec<Node>,
}

impl FromStr for Graph {
    type Err = String;

    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        let mut nodes = vec![
            Node {
                id: usize::max_value(),
                accepts: 0,
                outputs: 0,
                edges: vec![],
            },
        ];

        for (id, line) in contents.trim().lines().map(str::trim).enumerate() {
            let pins: Vec<usize> = line.split('/')
                .map(usize::from_str)
                .map(Result::unwrap)
                .collect();
            nodes.push(Node {
                id,
                accepts: pins[0],
                outputs: pins[1],
                edges: vec![],
            });
            nodes.push(Node {
                id,
                accepts: pins[1],
                outputs: pins[0],
                edges: vec![],
            });
        }

        let candidates: Vec<(usize, usize, usize)> = nodes
            .iter()
            .enumerate()
            .skip(1)
            .map(|(index, node)| (index, node.id, node.accepts))
            .collect();

        for node in &mut nodes {
            let mut edges: Vec<_> = candidates
                .iter()
                .filter(|&&(_, ref id, ref pins)| *id != node.id && *pins == node.outputs)
                .map(|&(index, _, _)| index)
                .collect();

            node.edges.append(&mut edges);
        }

        Ok(Self { nodes })
    }
}

impl Graph {
    pub fn strongest_path(&self) -> usize {
        let mut visited = HashSet::new();
        self.search(0, &mut visited, 0)
    }

    fn search(&self, node: usize, visited: &mut HashSet<usize>, strength: usize) -> usize {
        let new_strength = self.nodes[node].strength() + strength;
        let mut max = new_strength;

        visited.insert(self.nodes[node].id);
        for next_node in &self.nodes[node].edges {
            if !visited.contains(&self.nodes[*next_node].id) {
                max = max.max(self.search(*next_node, visited, new_strength));
            }
        }
        visited.remove(&self.nodes[node].id);
        max
    }
}

struct Node {
    id: usize,
    accepts: usize,
    outputs: usize,
    edges: Vec<usize>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]: -> {}/{} ->", self.id, self.accepts, self.outputs)
    }
}

impl Node {
    pub fn strength(&self) -> usize {
        self.accepts + self.outputs
    }
}

#[cfg(test)]
mod day24_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        assert_eq!(31, max_strength(input));
    }
}
