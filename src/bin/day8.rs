extern crate rusty_the_reindeer;

use std::collections::{HashMap};
use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = max_value(&contents);

    println!("Part 1: {}", part1);
}

fn max_value(contents: &str) -> i64 {
    let mut registers = Registers::default();
    for line in contents.lines() {
        registers.execute(&Command::parse(line));
    }

    registers.max_value()
}

enum Instruction<'a> {
    Inc { register: &'a str, value: i64 },
    Dec { register: &'a str, value: i64 },
}

enum Guard<'a> {
    LT { register: &'a str, value: i64 },
    LE { register: &'a str, value: i64 },
    EQ { register: &'a str, value: i64 },
    NE { register: &'a str, value: i64 },
    GE { register: &'a str, value: i64 },
    GT { register: &'a str, value: i64 },
}

struct Command<'a> {
    instruction: Instruction<'a>,
    guard: Option<Guard<'a>>,
}

impl <'a> Command<'a> {
    pub fn parse(line: &'a str) -> Self {
        let parts:Vec<&str> = line.split(" if ").collect();
        Self {
            instruction: Self::parse_instruction(parts[0]),
            guard: Self::parse_guard(parts.get(1)),
        }
    }

    fn parse_instruction(input: &str) -> Instruction {
        let mut parts = input.split_whitespace();
        let register = parts.next().unwrap();
        let instruction = parts.next().unwrap();
        let value = i64::from_str(parts.next().unwrap()).unwrap();

        match instruction {
            "inc" => Instruction::Inc { register, value },
            _ => Instruction::Dec { register, value },
        }
    }

    fn parse_guard(input: Option<&&'a str>) -> Option<Guard<'a>> {
        match input {
            None => None,
            Some(input) => {
                let mut parts = input.split_whitespace();
                let register = parts.next().unwrap();
                let comparison = parts.next().unwrap();
                let value = i64::from_str(parts.next().unwrap()).unwrap();

                match comparison {
                    "<" => Some(Guard::LT { register, value }),
                    "<=" => Some(Guard::LE { register, value }),
                    "==" => Some(Guard::EQ { register, value }),
                    "!=" => Some(Guard::NE { register, value }),
                    ">=" => Some(Guard::GE { register, value }),
                    ">" => Some(Guard::GT { register, value }),
                    _ => None,
                }
            }
        }
    }
}

#[derive(Default)]
struct Registers<'a> {
    registers: HashMap<&'a str, i64>,
}

impl <'a> Registers<'a> {
    pub fn execute(&mut self, command: &Command<'a>) {
        if self.evaluate_guard(&command.guard) {
            self.evaluate_instruction(&command.instruction);
        }
    }

    pub fn max_value(&self) -> i64 {
        *self.registers.values().max().unwrap_or(&0)
    }

    fn evaluate_guard(&mut self, guard: &Option<Guard<'a>>) -> bool {
        match *guard {
            Some(Guard::LT { register, value }) => {
                let entry = self.registers.entry(register).or_insert(0);
                *entry < value
            },
            Some(Guard::LE { register, value }) => {
                let entry = self.registers.entry(register).or_insert(0);
                *entry <= value
            },
            Some(Guard::EQ { register, value }) => {
                let entry = self.registers.entry(register).or_insert(0);
                *entry == value
            },
            Some(Guard::NE { register, value }) => {
                let entry = self.registers.entry(register).or_insert(0);
                *entry != value
            },
            Some(Guard::GE { register, value }) => {
                let entry = self.registers.entry(register).or_insert(0);
                *entry >= value
            },
            Some(Guard::GT { register, value }) => {
                let entry = self.registers.entry(register).or_insert(0);
                *entry > value
            },
            None => true,
        }
    }

    fn evaluate_instruction(&mut self, instruction: &Instruction<'a>) {
        let (register, value) = match *instruction {
            Instruction::Inc { register, value } => (register, value),
            Instruction::Dec { register, value } => (register, -value),
        };

        let entry = self.registers.entry(register).or_insert(0);
        *entry += value;
    }
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

        assert_eq!(1, max_value(input));
    }
}
