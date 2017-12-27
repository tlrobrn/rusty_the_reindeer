extern crate rusty_the_reindeer;

use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = execute(contents.trim());

    println!("Part 1: {}", part1);
}

fn execute(contents: &str) -> i64 {
    let instructions: Vec<_> = contents.lines().map(parse).collect();
    let mut registers: HashMap<char, i64> = HashMap::new();
    let mut position = 0;
    let mut last_played = None;

    while position < instructions.len() {
        match instructions[position].execute(&mut registers, last_played) {
            (None, Some(jump)) => position = advance(position, jump),
            (note_played, Some(jump)) => {
                last_played = note_played;
                position = advance(position, jump);
            }
            (Some(frequency), None) => return frequency,
            _ => position = advance(position, 1),
        }
    }
    last_played.unwrap()
}

fn advance(position: usize, jump: i64) -> usize {
    ((position as i64) + jump) as usize
}

enum Instruction<'a> {
    Snd(&'a str),
    Set(&'a str, &'a str),
    Add(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Mod(&'a str, &'a str),
    Rcv(&'a str),
    Jgz(&'a str, &'a str),
    Unknown,
}

fn as_register(s: &str) -> char {
    s.chars().next().unwrap_or('\0')
}

impl<'a> Instruction<'a> {
    fn execute(
        &self,
        registers: &mut HashMap<char, i64>,
        last_played: Option<i64>,
    ) -> (Option<i64>, Option<i64>) {
        use Instruction::*;

        match *self {
            Snd(register) => {
                let register = as_register(register);
                let entry = registers.entry(register).or_insert(0);
                (Some(*entry), Some(1))
            }
            Set(register, y) => {
                let register = as_register(register);
                let y = value(registers, y);
                registers.insert(register, y);
                (None, Some(1))
            }
            Add(register, y) => {
                let register = as_register(register);
                let y = value(registers, y);
                let entry = registers.entry(register).or_insert(0);
                *entry += y;
                (None, Some(1))
            }
            Mul(register, y) => {
                let register = as_register(register);
                let y = value(registers, y);
                let entry = registers.entry(register).or_insert(0);
                *entry *= y;
                (None, Some(1))
            }
            Mod(register, y) => {
                let register = as_register(register);
                let y = value(registers, y);
                let entry = registers.entry(register).or_insert(0);
                *entry %= y;
                (None, Some(1))
            }
            Rcv(y) => {
                if value(registers, y) != 0 {
                    (last_played, None)
                } else {
                    (None, Some(1))
                }
            }
            Jgz(guard, jump) => {
                if value(registers, guard) != 0 {
                    (None, Some(value(registers, jump)))
                } else {
                    (None, Some(1))
                }
            }
            _ => (None, Some(1)),
        }
    }
}

fn value(registers: &HashMap<char, i64>, v: &str) -> i64 {
    let n = i64::from_str(v);
    if n.is_ok() {
        n.unwrap()
    } else {
        *registers.get(&v.chars().next().unwrap()).unwrap_or(&0)
    }
}

fn parse(line: &str) -> Instruction {
    let mut tokens = line.trim().split_whitespace();
    let command = tokens.next().unwrap();
    let register = tokens.next().unwrap();
    let value = tokens.next().unwrap_or("");

    match command {
        "snd" => Instruction::Snd(register),
        "set" => Instruction::Set(register, value),
        "add" => Instruction::Add(register, value),
        "mul" => Instruction::Mul(register, value),
        "mod" => Instruction::Mod(register, value),
        "rcv" => Instruction::Rcv(register),
        "jgz" => Instruction::Jgz(register, value),
        _ => Instruction::Unknown,
    }
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        assert_eq!(4, execute(input));
    }
}
