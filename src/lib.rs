use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::env;

pub fn get_input() -> Option<String> {
    env::args()
        .last()
        .and_then(|filename| read_file(&filename).ok())
}

fn read_file(filepath: &str) -> io::Result<String> {
    let file = File::open(filepath)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

pub struct KnotHash {
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
        let mut dense_hash = [0; 16];

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
