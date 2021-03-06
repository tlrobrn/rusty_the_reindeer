extern crate rusty_the_reindeer;

use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = severity(contents.trim());
    let part2 = delay(contents.trim());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn severity(contents: &str) -> usize {
    let mut firewall = Firewall::parse(contents);
    traverse_firewall(&mut firewall)
}

fn delay(contents: &str) -> usize {
    let firewall = Firewall::parse(contents);
    let mut delay = 1;
    loop {
        let safe = firewall.range().all(|layer| {
            let time = layer + delay;
            firewall
                .scanner(layer)
                .map_or(true, |scanner| scanner.look_ahead(time) != 0)
        });

        if safe {
            return delay;
        }
        delay += 1;
    }
}

fn traverse_firewall(firewall: &mut Firewall) -> usize {
    firewall.range().fold(0, |mut total, layer| {
        if let Some(scanner) = firewall.scanner(layer) {
            if scanner.position == 0 {
                total += scanner.range * layer;
            }
        }
        firewall.tick();

        total
    })
}

struct Scanner {
    pub position: usize,
    pub range: usize,
    step: isize,
}

impl Scanner {
    pub fn new(range: usize) -> Self {
        Self {
            position: 0,
            range: range,
            step: 1,
        }
    }

    pub fn advance(&mut self) {
        self.position = (self.position as isize + self.step) as usize;
        if self.position == self.range - 1 || self.position == 0 {
            self.step = -self.step;
        }
    }

    pub fn look_ahead(&self, steps: usize) -> usize {
        steps % ((self.range - 1) * 2)
    }
}

struct Firewall {
    layers: HashMap<usize, Scanner>,
}

impl Firewall {
    pub fn parse(input: &str) -> Self {
        let mut layers = HashMap::new();

        for line in input.lines() {
            let mut tokens = line.split(": ");
            let depth = tokens.next().and_then(|s| usize::from_str(s).ok()).unwrap();
            let range = tokens.next().and_then(|s| usize::from_str(s).ok()).unwrap();

            layers.insert(depth, Scanner::new(range));
        }

        Self { layers }
    }

    pub fn tick(&mut self) {
        for scanner in self.layers.values_mut() {
            scanner.advance();
        }
    }

    pub fn range(&self) -> std::ops::Range<usize> {
        0..(*self.layers.keys().max().unwrap_or(&0) + 1)
    }

    pub fn scanner(&self, layer: usize) -> Option<&Scanner> {
        self.layers.get(&layer)
    }
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(24, severity(input));
    }

    #[test]
    fn part2() {
        let input = "0: 3
1: 2
4: 4
6: 4";
        assert_eq!(10, delay(input));
    }
}
