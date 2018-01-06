extern crate rusty_the_reindeer;

use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::iter::FromIterator;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = diagnostic_checksum(&contents);
    println!("Part 1: {}", part1);
}

fn diagnostic_checksum(contents: &str) -> usize {
    let mut machine = TuringMachine::from_str(contents).expect("Could not parse states");
    let steps: usize = contents
        .lines()
        .nth(1)
        .map(str::trim)
        .map(|line| line.split_whitespace().nth(5).unwrap())
        .map(|n| usize::from_str(n).unwrap())
        .unwrap();

    machine.run(steps);
    machine.checksum()
}

struct State {
    false_value: bool,
    false_movement: i64,
    false_state: char,
    true_value: bool,
    true_movement: i64,
    true_state: char,
}

impl FromStr for State {
    type Err = String;

    fn from_str(chunk: &str) -> Result<Self, Self::Err> {
        let mut lines = chunk.trim().lines().skip(2).map(str::trim);

        let false_value = lines.next().map(|line| line.contains('1')).unwrap();

        let false_movement = lines
            .next()
            .map(|line| if line.contains("left") { -1 } else { 1 })
            .unwrap();

        let false_state = lines
            .next()
            .map(|line| line.trim_right_matches('.'))
            .map(|line| line.chars().last().unwrap())
            .unwrap();

        lines.next();

        let true_value = lines.next().map(|line| line.contains('1')).unwrap();

        let true_movement = lines
            .next()
            .map(|line| if line.contains("left") { -1 } else { 1 })
            .unwrap();

        let true_state = lines
            .next()
            .map(|line| line.trim_right_matches('.'))
            .map(|line| line.chars().last().unwrap())
            .unwrap();

        Ok(Self {
            false_value,
            false_movement,
            false_state,
            true_value,
            true_movement,
            true_state,
        })
    }
}

struct TuringMachine {
    tape: HashSet<i64>,
    cursor: i64,
    states: HashMap<char, State>,
    current_state: char,
}

impl FromStr for TuringMachine {
    type Err = String;

    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        let current_state = contents
            .lines()
            .nth(0)
            .map(str::trim)
            .map(|line| {
                line.split_whitespace()
                    .nth(3)
                    .map(|w| w.chars().nth(0).unwrap())
                    .unwrap()
            })
            .unwrap();

        let states = contents
            .split("In state")
            .skip(1)
            .map(State::from_str)
            .map(Result::unwrap);

        Ok(Self {
            tape: HashSet::new(),
            cursor: 0,
            states: HashMap::from_iter(ALPHABET.chars().zip(states)),
            current_state,
        })
    }
}

impl TuringMachine {
    pub fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step()
        }
    }

    pub fn checksum(&self) -> usize {
        self.tape.len()
    }

    fn step(&mut self) {
        let state = &self.states[&self.current_state];
        if self.tape.contains(&self.cursor) {
            if !state.true_value {
                self.tape.remove(&self.cursor);
            }
            self.cursor += state.true_movement;
            self.current_state = state.true_state;
        } else {
            if state.false_value {
                self.tape.insert(self.cursor);
            }
            self.cursor += state.false_movement;
            self.current_state = state.false_state;
        }
    }
}

#[cfg(test)]
mod day25_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";
        assert_eq!(3, diagnostic_checksum(input));
    }
}
