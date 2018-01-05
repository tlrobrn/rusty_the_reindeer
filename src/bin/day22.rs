extern crate rusty_the_reindeer;

use std::collections::HashSet;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = count_infected(&contents, 10_000);
    println!("Part 1: {}", part1);
}

fn count_infected(contents: &str, bursts: usize) -> usize {
    let (mut infected, mut virus) = parse_contents(contents);

    (0..bursts).fold(0, |total, _| {
        if infected.contains(&virus.position) {
            virus.direction = virus.direction.turn_right();
            infected.remove(&virus.position);
            virus.move_forward();
            total
        } else {
            virus.direction = virus.direction.turn_left();
            infected.insert(virus.position);
            virus.move_forward();
            total + 1
        }
    })
}

fn parse_contents(contents: &str) -> (HashSet<Coordinate>, Virus) {
    let mut infected = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, node) in line.chars().enumerate() {
            if node == '#' {
                infected.insert((x as i64, -(y as i64)));
            }
        }
    }

    let center_y = -((contents.lines().count() / 2) as i64);
    let center_x = (contents.lines().next().unwrap().len() / 2) as i64;

    (
        infected,
        Virus {
            position: (center_x, center_y),
            direction: Direction::North,
        },
    )
}

type Coordinate = (i64, i64);

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn_right(&self) -> Self {
        use Direction::*;
        match *self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }

    pub fn turn_left(&self) -> Self {
        use Direction::*;
        match *self {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }
}

struct Virus {
    position: Coordinate,
    direction: Direction,
}

impl Virus {
    pub fn move_forward(&mut self) {
        use Direction::*;
        let (x, y) = self.position;

        match self.direction {
            North => self.position = (x, y + 1),
            South => self.position = (x, y - 1),
            East => self.position = (x + 1, y),
            West => self.position = (x - 1, y),
        }
    }
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "..#
#..
...
";
        assert_eq!(5, count_infected(input, 7));
        assert_eq!(41, count_infected(input, 70));
        assert_eq!(5587, count_infected(input, 10_000));
    }
}
