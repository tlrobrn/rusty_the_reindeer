extern crate rusty_the_reindeer;

use rusty_the_reindeer::KnotHash;
use std::collections::{HashMap, HashSet};

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = count(contents.trim());
    let part2 = count_regions(contents.trim());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn count(contents: &str) -> u32 {
    (0..128)
        .map(|x| KnotHash::new(&format!("{}-{}", contents, x)).dense())
        .map(|hash| {
            hash.chars().fold(0, |total, c| {
                total + c.to_digit(16).map_or(0, |n| n.count_ones())
            })
        })
        .sum()
}

fn count_regions(contents: &str) -> u32 {
    let mut rows: Vec<Vec<bool>> = vec![Vec::new(); 128];
    for (i, row) in rows.iter_mut().enumerate() {
        let hash = KnotHash::new(&format!("{}-{}", contents, i)).dense();
        for digit_bits in hash.chars().map(bits) {
            for bit in &digit_bits {
                row.push(*bit);
            }
        }
    }

    let mut seen = HashSet::new();
    let mut total = 0;
    let graph = Graph::new(&rows);
    for row in 0..128 {
        for column in 0..128 {
            let node = (row, column);
            if !seen.contains(&node) && graph.contains(&node) {
                for &node in &graph.group(node) {
                    seen.insert(node);
                }
                total += 1;
            }
        }
    }

    total
}

fn bits(c: char) -> [bool; 4] {
    let n = c.to_digit(16).unwrap_or(0);

    let mut binary = [false; 4];
    for (i, bit) in binary.iter_mut().enumerate() {
        let mask = 2u32.pow(3u32 - i as u32);
        *bit = mask & n == mask;
    }

    binary
}

type Coordinate = (usize, usize);

#[derive(Default)]
struct Graph {
    nodes: HashMap<Coordinate, Vec<Coordinate>>,
}

impl Graph {
    pub fn new(rows: &[Vec<bool>]) -> Self {
        let mut graph = Self::default();

        for (row_index, row) in rows.iter().enumerate() {
            for (column_index, _) in row.iter().enumerate().filter(|&(_, x)| *x) {
                let mut neighbors = Vec::new();
                if row_index > 0 && rows[row_index - 1][column_index] {
                    neighbors.push((row_index - 1, column_index));
                }
                if row_index < 127 && rows[row_index + 1][column_index] {
                    neighbors.push((row_index + 1, column_index));
                }
                if column_index > 0 && rows[row_index][column_index - 1] {
                    neighbors.push((row_index, column_index - 1));
                }
                if column_index < 127 && rows[row_index][column_index + 1] {
                    neighbors.push((row_index, column_index + 1));
                }

                graph.nodes.insert((row_index, column_index), neighbors);
            }
        }

        graph
    }

    pub fn contains(&self, node: &Coordinate) -> bool {
        self.nodes.contains_key(node)
    }

    pub fn group(&self, node: Coordinate) -> HashSet<Coordinate> {
        let mut mates = HashSet::new();
        mates.insert(node);
        self.build_group(node, &mut mates);
        mates
    }

    fn build_group(&self, node: Coordinate, group: &mut HashSet<Coordinate>) {
        for &mate in &self.nodes[&node] {
            if !group.contains(&mate) {
                group.insert(mate);
                self.build_group(mate, group);
            }
        }
    }
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "flqrgnkx";
        assert_eq!(8108, count(input));
    }

    #[test]
    fn part2() {
        let input = "flqrgnkx";
        assert_eq!(1242, count_regions(input));
    }

    #[test]
    fn test_bits() {
        assert_eq!([false, false, false, false], bits('0'));
        assert_eq!([false, false, false, true], bits('1'));
        assert_eq!([true, true, true, false], bits('e'));
        assert_eq!([true, true, true, true], bits('f'));
    }
}
