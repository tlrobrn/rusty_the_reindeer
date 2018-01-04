#![feature(iterator_step_by)]
extern crate rusty_the_reindeer;

use std::fmt;
use std::collections::HashMap;
use std::str::FromStr;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

const INITIAL_STATE: &str = ".#./..#/###";

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = enhance(&contents, 5);
    println!("Part 1: {}", part1);
    let part2 = enhance(&contents, 18);
    println!("Part 2: {}", part2);
}

fn enhance(contents: &str, count: usize) -> usize {
    let mut state = Image::from_str(INITIAL_STATE).unwrap();
    let ruleset = parse_ruleset(contents);

    for _ in 0..count {
        state.enhance(&ruleset);
    }

    state.count()
}

fn parse_ruleset(contents: &str) -> HashMap<Image, Image> {
    let mut ruleset = HashMap::new();
    for line in contents.lines() {
        let mut parts = line.split(" => ");
        let base_key = Image::from_str(parts.next().unwrap()).unwrap();
        let outcome = Image::from_str(parts.next().unwrap()).unwrap();

        ruleset.insert(base_key.rotated(), outcome.clone());
        ruleset.insert(base_key.rotated().rotated(), outcome.clone());
        ruleset.insert(base_key.rotated().rotated().rotated(), outcome.clone());

        let flipped = base_key.flipped();
        ruleset.insert(flipped.rotated(), outcome.clone());
        ruleset.insert(flipped.rotated().rotated(), outcome.clone());
        ruleset.insert(flipped.rotated().rotated().rotated(), outcome.clone());
        ruleset.insert(base_key, outcome.clone());
        ruleset.insert(flipped, outcome);
    }

    ruleset
}

#[derive(Debug, Clone)]
struct Image {
    pixels: Vec<Vec<bool>>,
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pixels = vec![];
        for row in s.split('/') {
            let mut pixel_row = vec![];
            for pixel in row.trim().chars() {
                pixel_row.push(pixel == '#');
            }
            pixels.push(pixel_row);
        }

        Ok(Self { pixels })
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rows: Vec<String> = self.pixels
            .iter()
            .map(|row| row.iter().map(|&pixel| if pixel { '#' } else { '.' }))
            .map(String::from_iter)
            .collect();

        write!(f, "{}", rows.join("/"))
    }
}

impl Hash for Image {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl Eq for Image {}

impl Image {
    pub fn size(&self) -> usize {
        self.pixels.len()
    }

    pub fn flipped(&self) -> Self {
        let mut pixels = self.pixels.clone();
        pixels.iter_mut().for_each(|row| row.reverse());
        Self { pixels }
    }

    pub fn rotated(&self) -> Self {
        let mut pixels = self.pixels.clone();
        let size = self.size();

        for row in 0..size {
            for col in 0..size {
                pixels[col][size - (row + 1)] = self.pixels[row][col];
            }
        }
        Self { pixels }
    }

    pub fn split(&self) -> Vec<Vec<Self>> {
        if self.size() % 2 == 0 {
            self.split_by(2)
        } else {
            self.split_by(3)
        }
    }

    pub fn join(sections: &[Vec<Self>]) -> Self {
        let size = sections.len() * sections[0][0].size();
        let mut pixels: Vec<Vec<bool>> = vec![vec![false; size]; size];

        for (i, row) in sections.iter().enumerate() {
            for (j, image) in row.iter().enumerate() {
                for (y, pixel_row) in image.pixels.iter().enumerate() {
                    for (x, &pixel) in pixel_row.iter().enumerate() {
                        pixels[y + (i * image.size())][x + (j * image.size())] = pixel;
                    }
                }
            }
        }

        Self { pixels }
    }

    pub fn enhance(&mut self, ruleset: &HashMap<Self, Self>) {
        let pieces: Vec<_> = self.split()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|image| ruleset[image].clone())
                    .collect::<Vec<Self>>()
            })
            .collect();
        *self = Self::join(&pieces);
    }

    pub fn count(&self) -> usize {
        self.pixels
            .iter()
            .fold(0, |total, row| total + row.iter().filter(|&&p| p).count())
    }

    fn split_by(&self, count: usize) -> Vec<Vec<Self>> {
        let mut images = vec![];
        let mut sections = vec![vec![vec![false; count]; self.size() / count]; self.size()];

        for (i, row) in self.pixels.iter().enumerate() {
            for (j, chunk) in row.chunks(count).enumerate() {
                for (x, &pixel) in chunk.iter().enumerate() {
                    sections[i][j][x] = pixel;
                }
            }
        }

        for row in (0..sections.len()).step_by(count) {
            let mut image_row = vec![];
            for column in 0..(self.size() / count) {
                let mut pixels = vec![vec![false; count]; count];
                for i in 0..count {
                    for j in 0..count {
                        pixels[i][j] = sections[row + i][column][j];
                    }
                }
                image_row.push(Self { pixels });
            }
            images.push(image_row);
        }

        images
    }
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        assert_eq!(12, enhance(input, 2));
    }
}

#[cfg(test)]
mod image_tests {
    use super::*;

    #[test]
    fn to_string_test() {
        let image = Image::from_str(INITIAL_STATE).unwrap();
        assert_eq!(INITIAL_STATE, image.to_string());
    }

    #[test]
    fn flip_test() {
        let image = Image::from_str(INITIAL_STATE).unwrap();
        let flipped_image = Image::from_str(".#./#../###").unwrap();
        assert_eq!(flipped_image, image.flipped());
    }

    #[test]
    fn rotate_test() {
        let image = Image::from_str(INITIAL_STATE).unwrap();
        let rotated_image = Image::from_str("#../#.#/##.").unwrap();
        assert_eq!(rotated_image, image.rotated());
    }

    #[test]
    fn split_multiple_of_2_test() {
        let image = Image::from_str("#..#/..../..../#..#").unwrap();
        let expected = vec![
            vec![
                Image::from_str("#./..").unwrap(),
                Image::from_str(".#/..").unwrap(),
            ],
            vec![
                Image::from_str("../#.").unwrap(),
                Image::from_str("../.#").unwrap(),
            ],
        ];

        assert_eq!(expected, image.split());
    }

    #[test]
    fn split_multiple_of_3_test() {
        let image = Image::from_str("#......../...#...../......#../##......./...##..../......##./###....../...###.../......###").unwrap();
        let expected = vec![
            vec![
                Image::from_str("#../.../...").unwrap(),
                Image::from_str(".../#../...").unwrap(),
                Image::from_str(".../.../#..").unwrap(),
            ],
            vec![
                Image::from_str("##./.../...").unwrap(),
                Image::from_str(".../##./...").unwrap(),
                Image::from_str(".../.../##.").unwrap(),
            ],
            vec![
                Image::from_str("###/.../...").unwrap(),
                Image::from_str(".../###/...").unwrap(),
                Image::from_str(".../.../###").unwrap(),
            ],
        ];

        assert_eq!(expected, image.split());
    }

    #[test]
    fn join_multiple_of_2_test() {
        let split = vec![
            vec![
                Image::from_str("#./..").unwrap(),
                Image::from_str(".#/..").unwrap(),
            ],
            vec![
                Image::from_str("../#.").unwrap(),
                Image::from_str("../.#").unwrap(),
            ],
        ];
        let expected = Image::from_str("#..#/..../..../#..#").unwrap();

        assert_eq!(expected, Image::join(&split));
    }

    #[test]
    fn join_multiple_of_3_test() {
        let split = vec![
            vec![
                Image::from_str("#../.../...").unwrap(),
                Image::from_str(".../#../...").unwrap(),
                Image::from_str(".../.../#..").unwrap(),
            ],
            vec![
                Image::from_str("##./.../...").unwrap(),
                Image::from_str(".../##./...").unwrap(),
                Image::from_str(".../.../##.").unwrap(),
            ],
            vec![
                Image::from_str("###/.../...").unwrap(),
                Image::from_str(".../###/...").unwrap(),
                Image::from_str(".../.../###").unwrap(),
            ],
        ];
        let expected = Image::from_str("#......../...#...../......#../##......./...##..../......##./###....../...###.../......###").unwrap();

        assert_eq!(expected, Image::join(&split));
    }
}
