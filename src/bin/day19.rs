extern crate rusty_the_reindeer;

use std::str::FromStr;
use std::iter::FromIterator;
use std::collections::HashMap;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = trace(&contents);
    println!("Part 1: {}", part1);
    let part2 = count(&contents);
    println!("Part 2: {}", part2);
}

fn trace(contents: &str) -> String {
    String::from_iter(
        Path::from_str(contents)
            .unwrap()
            .trace()
            .filter(|c| c.is_alphanumeric()),
    )
}

fn count(contents: &str) -> usize {
    Path::from_str(contents).unwrap().trace().count() + 1
}

type Coordinates = (usize, usize);

struct Node {
    coordinates: Coordinates,
    data: char,
}

impl Node {
    pub fn north(&self) -> Option<Coordinates> {
        if self.coordinates.1 > 0 {
            Some((self.coordinates.0, self.coordinates.1 - 1))
        } else {
            None
        }
    }

    pub fn south(&self) -> Option<Coordinates> {
        Some((self.coordinates.0, self.coordinates.1 + 1))
    }

    pub fn east(&self) -> Option<Coordinates> {
        Some((self.coordinates.0 + 1, self.coordinates.1))
    }

    pub fn west(&self) -> Option<Coordinates> {
        if self.coordinates.0 > 0 {
            Some((self.coordinates.0 - 1, self.coordinates.1))
        } else {
            None
        }
    }
}

struct Path {
    nodes: HashMap<Coordinates, Node>,
}

impl Path {
    pub fn trace(&self) -> Trace {
        Trace::new(self)
    }

    fn start(&self) -> &Node {
        self.nodes
            .values()
            .find(|node| node.coordinates.1 == 0)
            .unwrap()
    }
}

impl FromStr for Path {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes = s.lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
            .filter(|&(_coordinates, data)| !data.is_whitespace())
            .map(|(coordinates, data)| (coordinates, Node { coordinates, data }));

        Ok(Self {
            nodes: HashMap::from_iter(nodes),
        })
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

struct Trace<'a> {
    path: &'a Path,
    current_node: &'a Node,
    direction: Direction,
    attempts: u8,
}

impl<'a> Trace<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self {
            path,
            current_node: path.start(),
            direction: Direction::South,
            attempts: 0,
        }
    }
}

impl<'a> Iterator for Trace<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.attempts > 3 {
            None
        } else {
            match self.direction {
                Direction::North => {
                    let coordinates = self.current_node.north().unwrap();
                    if self.attempts == 2 {
                        self.direction = Direction::East;
                        self.attempts += 1;
                        self.next()
                    } else {
                        match self.path.nodes.get(&coordinates) {
                            Some(node) => {
                                self.attempts = 0;
                                self.current_node = node;
                                Some(node.data)
                            }
                            None => {
                                self.direction = Direction::East;
                                self.attempts += 1;
                                self.next()
                            }
                        }
                    }
                }
                Direction::South => {
                    let coordinates = self.current_node.south().unwrap();
                    if self.attempts == 2 {
                        self.direction = Direction::West;
                        self.attempts += 1;
                        self.next()
                    } else {
                        match self.path.nodes.get(&coordinates) {
                            Some(node) => {
                                self.attempts = 0;
                                self.current_node = node;
                                Some(node.data)
                            }
                            None => {
                                self.direction = Direction::West;
                                self.attempts += 1;
                                self.next()
                            }
                        }
                    }
                }
                Direction::East => {
                    let coordinates = self.current_node.east().unwrap();
                    if self.attempts == 2 {
                        self.direction = Direction::South;
                        self.attempts += 1;
                        self.next()
                    } else {
                        match self.path.nodes.get(&coordinates) {
                            Some(node) => {
                                self.attempts = 0;
                                self.current_node = node;
                                Some(node.data)
                            }
                            None => {
                                self.direction = Direction::South;
                                self.attempts += 1;
                                self.next()
                            }
                        }
                    }
                }
                Direction::West => {
                    let coordinates = self.current_node.west().unwrap();
                    if self.attempts == 2 {
                        self.direction = Direction::North;
                        self.attempts += 1;
                        self.next()
                    } else {
                        match self.path.nodes.get(&coordinates) {
                            Some(node) => {
                                self.attempts = 0;
                                self.current_node = node;
                                Some(node.data)
                            }
                            None => {
                                self.direction = Direction::North;
                                self.attempts += 1;
                                self.next()
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
";
        assert_eq!("ABCDEF", trace(input));
    }

    #[test]
    fn part2() {
        let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
";
        assert_eq!(38, count(input));
    }
}
