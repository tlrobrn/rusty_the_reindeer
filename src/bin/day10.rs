extern crate rusty_the_reindeer;

use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = once(contents.trim());
    let part2 = hash(contents.trim());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn once(contents: &str) -> u64 {
    let input: Vec<u8> = contents.split(',').map(|s| u8::from_str(s).unwrap()).collect();
    KnotHash::default().round(&input).head()
}

fn hash(contents: &str) -> String {
    KnotHash::new(contents).dense()
}

struct KnotHash {
    list: [u8; 256],
    position: usize,
    skip_size: usize,
}

impl Default for KnotHash {
    fn default() -> Self {
        let mut list: [u8; 256] = [0; 256];

        for (x, value) in list.iter_mut().enumerate().skip(1) {
            *value = x as u8;
        }

        KnotHash {
            list,
            position: 0,
            skip_size: 0,
        }
    }
}

impl KnotHash {
    const SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];

    pub fn new(input: &str) -> Self {
        let input: Vec<u8> = input.as_bytes().iter().chain(Self::SUFFIX.iter()).cloned().collect();

        let mut knot_hash = Self::default();
        for _ in 0..64 {
            knot_hash.round(&input);
        }
        knot_hash
    }

    pub fn round(&mut self, lengths: &[u8]) -> &Self {
        for &length in lengths {
            self.reverse(length as usize);
            self.move_position(length);
            self.advance_skip_size();
        }

        self
    }

    pub fn head(&self) -> u64 {
        (u64::from(self.list[0])) * (u64::from(self.list[1]))
    }

    pub fn dense(&self) -> String {
        let mut bytes = self.list.iter();
        let mut dense_hash = vec![0; 16];

        for block in &mut dense_hash {
            *block = *bytes.next().unwrap();
            for _ in 0..15 {
                *block ^= *bytes.next().unwrap();
            }
        }

        let hexes: Vec<String> = dense_hash.iter().map(|n| format!("{:02x}", n)).collect();
        hexes.join("")
    }

    fn reverse(&mut self, length: usize) {
        let position = self.position;

        let mut values = Vec::new();
        for offset in 0..length {
            let index = (position + offset) % 256;
            values.push(self.list[index]);
        }
        values.reverse();

        values.iter()
            .enumerate()
            .map(|(offset, &value)| ((position + offset) % 256, value))
            .for_each(|(index, value)| self.list[index] = value);
    }

    fn move_position(&mut self, length: u8) {
        self.position += length as usize + self.skip_size;
    }

    fn advance_skip_size(&mut self) {
        self.skip_size += 1;
    }
}


#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(52070, once("46,41,212,83,1,255,157,65,139,52,39,254,2,86,0,204"));
    }

    #[test]
    fn part2() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", hash(""));
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", hash("AoC 2017"));
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", hash("1,2,3"));
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", hash("1,2,4"));
    }
}
