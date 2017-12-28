extern crate rusty_the_reindeer;

use std::collections::VecDeque;
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = execute(contents.trim());
    println!("Part 1: {}", part1);

    let part2 = execute_pair(contents.trim());
    println!("Part 2: {}", part2);
}

fn execute(contents: &str) -> i64 {
    let instructions: Vec<_> = contents
        .lines()
        .map(Instruction::from_str)
        .map(Result::unwrap)
        .collect();
    let mut process = Process::part1(&instructions);
    while let Some(m) = process.run() {
        match m {
            Message::Sent(lp) => process.send(lp),
            Message::Received(lp) => return process.final_message().unwrap_or(lp),
        }
    }
    0
}

fn execute_pair(contents: &str) -> usize {
    let instructions: Vec<_> = contents
        .lines()
        .map(Instruction::from_str)
        .map(Result::unwrap)
        .collect();
    let mut programs: Vec<_> = (0..2).map(|id| Process::part2(&instructions, id)).collect();
    let mut active = 0;
    let mut count = 0;

    loop {
        let inactive = (active + 1) % 2;
        match programs[active].run() {
            Some(Message::Sent(m)) => {
                programs[inactive].send(m);
                count += active;
            }
            Some(_) => (),
            None => {
                if programs[inactive].is_blocked() {
                    return count;
                }
                active = inactive;
            }
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Snd(Value),
    Set(Value, Value),
    Add(Value, Value),
    Mul(Value, Value),
    Mod(Value, Value),
    Rcv(Value),
    Jgz(Value, Value),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.trim().split_whitespace();
        let command = tokens.next().unwrap();
        let register = Value::from_str(tokens.next().unwrap()).unwrap();
        let value = Value::from_str(tokens.next().unwrap_or("")).unwrap();

        match command {
            "snd" => Ok(Instruction::Snd(register)),
            "set" => Ok(Instruction::Set(register, value)),
            "add" => Ok(Instruction::Add(register, value)),
            "mul" => Ok(Instruction::Mul(register, value)),
            "mod" => Ok(Instruction::Mod(register, value)),
            "rcv" => Ok(Instruction::Rcv(register)),
            "jgz" => Ok(Instruction::Jgz(register, value)),
            _ => Err(format!("Instruction could not be parsed: {}", s)),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Snd(ref v) => write!(f, "snd {}", v),
            Instruction::Set(ref a, ref b) => write!(f, "set {} {}", a, b),
            Instruction::Add(ref a, ref b) => write!(f, "add {} {}", a, b),
            Instruction::Mul(ref a, ref b) => write!(f, "mul {} {}", a, b),
            Instruction::Mod(ref a, ref b) => write!(f, "mod {} {}", a, b),
            Instruction::Rcv(ref v) => write!(f, "rcv {}", v),
            Instruction::Jgz(ref a, ref b) => write!(f, "jgz {} {}", a, b),
        }
    }
}

#[derive(Debug, Clone)]
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Register(c) => write!(f, "{}", c),
            Value::Literal(l) => write!(f, "{}", l),
            Value::Empty => write!(f, ""),
        }
    }
}

impl Value {
    pub fn as_register(&self) -> Result<char, String> {
        match *self {
            Value::Register(r) => Ok(r),
            _ => Err(format!("Value is not a register: {:?}", self)),
        }
    }
}

#[derive(Debug)]
enum Message {
    Sent(i64),
    Received(i64),
}

#[derive(Default)]
struct Process {
    inbox: VecDeque<i64>,
    registers: HashMap<char, i64>,
    instructions: Vec<Instruction>,
    position: usize,
    store_receive: bool,
}

impl Process {
    pub fn part1(instructions: &[Instruction]) -> Self {
        Self::new(instructions, false, None)
    }
    pub fn part2(instructions: &[Instruction], id: i64) -> Self {
        Self::new(instructions, true, Some(id))
    }

    fn new(instructions: &[Instruction], store_receive: bool, id: Option<i64>) -> Self {
        let mut p = Self {
            inbox: VecDeque::new(),
            registers: HashMap::new(),
            instructions: instructions.to_vec(),
            position: 0,
            store_receive,
        };

        if let Some(id) = id {
            p.registers.insert('p', id);
        }

        p
    }

    pub fn send(&mut self, msg: i64) {
        self.inbox.push_back(msg);
    }

    pub fn final_message(&self) -> Option<i64> {
        self.inbox.back().cloned()
    }

    pub fn is_blocked(&self) -> bool {
        match self.instructions.get(self.position) {
            Some(&Instruction::Rcv(_)) => self.inbox.is_empty(),
            Some(_) => false,
            None => true,
        }
    }

    pub fn run(&mut self) -> Option<Message> {
        loop {
            let instruction = self.instructions.get(self.position).cloned();
            self.position += 1;

            match instruction {
                Some(Instruction::Snd(value)) => return self.reduce(&value).map(Message::Sent),
                Some(Instruction::Set(r, v)) => {
                    let register = r.as_register().unwrap();
                    let value = self.reduce(&v).unwrap();
                    self.registers.insert(register, value);
                }
                Some(Instruction::Add(r, v)) => {
                    let register = r.as_register().unwrap();
                    let value = self.reduce(&v).unwrap();
                    let entry = self.registers.entry(register).or_insert(0);
                    *entry += value;
                }
                Some(Instruction::Mul(r, v)) => {
                    let register = r.as_register().unwrap();
                    let value = self.reduce(&v).unwrap();
                    let entry = self.registers.entry(register).or_insert(0);
                    *entry *= value;
                }
                Some(Instruction::Mod(r, v)) => {
                    let register = r.as_register().unwrap();
                    let value = self.reduce(&v).unwrap();
                    let entry = self.registers.entry(register).or_insert(0);
                    *entry %= value;
                }
                Some(Instruction::Rcv(ref v)) if self.store_receive => match self.inbox.pop_front()
                {
                    None => {
                        self.position -= 1;
                        return None;
                    }
                    Some(msg) => {
                        let register = v.as_register().unwrap();
                        self.registers.insert(register, msg);
                    }
                },
                Some(Instruction::Rcv(v)) => {
                    if self.reduce(&v).unwrap() != 0 {
                        match self.inbox.pop_front() {
                            None => {
                                self.position -= 1;
                                return None;
                            }
                            msg => return msg.map(Message::Received),
                        }
                    }
                }
                Some(Instruction::Jgz(g, v)) => {
                    if self.reduce(&g).unwrap() > 0 {
                        self.position =
                            (self.position as i64 + self.reduce(&v).unwrap() - 1) as usize;
                    }
                }
                None => return None,
            }
        }
    }

    fn reduce(&mut self, value: &Value) -> Option<i64> {
        match *value {
            Value::Register(r) => Some(*self.registers.entry(r).or_insert(0)),
            Value::Literal(l) => Some(l),
            Value::Empty => None,
        }
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

    #[test]
    fn part2() {
        let input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
        assert_eq!(3, execute_pair(input));
    }
}
