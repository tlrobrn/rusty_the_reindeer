extern crate rusty_the_reindeer;

use std::str::FromStr;
use std::cell::{Cell, RefCell};

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = hash(&contents);

    println!("Part 1: {}", part1);
}

fn hash(contents: &str) -> u64 {
    KnotHash::new().encrypt(contents.trim()).head()
}

struct KnotHash {
    list: RefCell<[u8; 256]>,
    position: Cell<usize>,
    skip_size: Cell<usize>,
}

impl Default for KnotHash {
    fn default() -> Self {
        let mut list: [u8; 256] = [0; 256];

        for (x, value) in list.iter_mut().enumerate().skip(1) {
            *value = x as u8;
        }

        KnotHash {
            list: RefCell::new(list),
            position: Cell::new(0),
            skip_size: Cell::new(0),
        }
    }
}

impl KnotHash {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn encrypt(&self, lengths: &str) -> &Self {
        for length in lengths.split(',') {
            let length = usize::from_str(length).expect("Invalid length");

            self.reverse(length);
            self.move_position(length);
            self.advance_skip_size();
        }

        self
    }

    pub fn head(&self) -> u64 {
        let list = self.list.borrow();
        (u64::from(list[0])) * (u64::from(list[1]))
    }

    fn reverse(&self, length: usize) {
        let mut list = self.list.borrow_mut();
        let position = self.position.get();
        let mut values = Vec::new();

        for offset in 0..length {
            let index = (position + offset) % 256;
            values.push(list[index]);
        }
        values.reverse();

        values.iter()
            .enumerate()
            .map(|(offset, &value)| ((position + offset) % 256, value))
            .for_each(|(index, value)| list[index] = value);
    }

    fn move_position(&self, length: usize) {
        self.position.set(self.position.get() + length + self.skip_size.get());
    }

    fn advance_skip_size(&self) {
        self.skip_size.set(self.skip_size.get() + 1);
    }
}


#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(52070, hash("46,41,212,83,1,255,157,65,139,52,39,254,2,86,0,204"));
    }
}
