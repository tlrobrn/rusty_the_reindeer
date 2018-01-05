extern crate rusty_the_reindeer;

use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = execute_debug(contents.trim());
    println!("Part 1: {}", part1);
    let part2 = execute(contents.trim());
    println!("Part 2: {}", part2);
}

fn execute_debug(contents: &str) -> usize {
    let instructions: Vec<_> = contents
        .lines()
        .map(Instruction::from_str)
        .map(Result::unwrap)
        .collect();

    process(&instructions)
        .filter(|instruction| match *instruction {
            Instruction::Mul(_, _) => true,
            _ => false,
        })
        .count()
}

fn process(instructions: &[Instruction]) -> Process {
    Process {
        registers: Registers::new(),
        instructions,
        position: 0,
    }
}

fn execute(contents: &str) -> i64 {
    let instructions: Vec<_> = contents
        .lines()
        .map(Instruction::from_str)
        .map(Result::unwrap)
        .collect();

    let mut context = Registers::new();
    context.insert('a', 1);
    let mut process = process_with_context(&instructions, context);
    while let Some(_) = process.next() {}
    process.registers.get(&'h').cloned().unwrap_or(0)
}

fn process_with_context(instructions: &[Instruction], registers: Registers) -> Process {
    Process {
        registers,
        instructions,
        position: 0
    }
}

struct Process<'a> {
    registers: Registers,
    instructions: &'a [Instruction],
    position: usize,
}

impl<'a> Iterator for Process<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions.get(self.position).cloned();
        self.advance();
        instruction
    }
}

impl<'a> Process<'a> {
    fn advance(&mut self) {
        use Instruction::*;
        match self.instructions.get(self.position) {
            None => {}
            Some(&Set(register, ref value)) => {
                let value = value.resolve(&self.registers).unwrap_or(0);
                self.registers.insert(register, value);
                self.position += 1;
            }
            Some(&Sub(register, ref value)) => {
                let value = value.resolve(&self.registers).unwrap_or(0);
                let entry = self.registers.entry(register).or_insert(0);
                *entry -= value;
                self.position += 1;
            }
            Some(&Mul(register, ref value)) => {
                let value = value.resolve(&self.registers).unwrap_or(0);
                let entry = self.registers.entry(register).or_insert(0);
                *entry *= value;
                self.position += 1;
            }
            Some(&Jnz(ref guard, ref offset)) => {
                let guard = guard.resolve(&self.registers).unwrap_or(0);
                if guard != 0 {
                    let offset = offset.resolve(&self.registers).unwrap_or(0);
                    self.position = ((self.position as i64) + offset) as usize;
                } else {
                    self.position += 1;
                }
            }
        }
    }
}

type Registers = HashMap<char, i64>;

#[derive(Clone, PartialEq)]
enum Instruction {
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.trim().split_whitespace();
        let command = tokens.next().unwrap();

        match command {
            "set" => {
                let register = tokens.next().unwrap().trim().chars().next().unwrap();
                let value = Value::from_str(tokens.next().unwrap()).unwrap();
                Ok(Instruction::Set(register, value))
            }
            "sub" => {
                let register = tokens.next().unwrap().trim().chars().next().unwrap();
                let value = Value::from_str(tokens.next().unwrap()).unwrap();
                Ok(Instruction::Sub(register, value))
            }
            "mul" => {
                let register = tokens.next().unwrap().trim().chars().next().unwrap();
                let value = Value::from_str(tokens.next().unwrap()).unwrap();
                Ok(Instruction::Mul(register, value))
            }
            "jnz" => {
                let register = Value::from_str(tokens.next().unwrap()).unwrap();
                let value = Value::from_str(tokens.next().unwrap()).unwrap();
                Ok(Instruction::Jnz(register, value))
            }
            _ => Err(format!("Instruction could not be parsed: {}", s)),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Value {
    Register(char),
    Literal(i64),
    Empty,
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match i64::from_str(s.trim()) {
            Ok(n) => Ok(Value::Literal(n)),
            _ => match s.chars().next() {
                Some(c) => Ok(Value::Register(c)),
                None => Ok(Value::Empty),
            },
        }
    }
}

impl Value {
    fn resolve(&self, context: &Registers) -> Option<i64> {
        match *self {
            Value::Register(r) => context.get(&r).cloned(),
            Value::Literal(l) => Some(l),
            Value::Empty => None,
        }
    }
}

#[cfg(test)]
mod day23_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "set b 67
set c b
jnz a 2
jnz 1 5
mul b 100
sub b -100000
set c b
sub c -17000
set f 1
set d 2
set e 2
set g d
mul g e
sub g b
jnz g 2
set f 0
sub e -1
set g e
sub g b
jnz g -8
sub d -1
set g d
sub g b
jnz g -13
jnz f 2
sub h -1
set g b
sub g c
jnz g 2
jnz 1 3
sub b -17
jnz 1 -23";
        assert_eq!(4225, execute_debug(input));
    }
}
