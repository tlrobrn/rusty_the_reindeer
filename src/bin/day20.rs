extern crate rusty_the_reindeer;

use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = pokey_little_puppy(&contents);
    println!("Part 1: {}", part1);
}

fn pokey_little_puppy(contents: &str) -> usize {
    contents
        .lines()
        .map(Particle::from_str)
        .map(Result::unwrap)
        .enumerate()
        .min_by_key(|&(_, ref particle)| {
            (
                particle.acceleration.manhatten_distance(),
                particle.velocity.manhatten_distance(),
                particle.position.manhatten_distance(),
            )
        })
        .map(|(id, _)| id)
        .unwrap()
}

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s[3..]
            .trim_right_matches('>')
            .split(',')
            .map(str::trim)
            .map(i64::from_str)
            .map(Result::unwrap);

        Ok(Self {
            x: values.next().unwrap(),
            y: values.next().unwrap(),
            z: values.next().unwrap(),
        })
    }
}

impl Point {
    pub fn manhatten_distance(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

struct Particle {
    position: Point,
    velocity: Point,
    acceleration: Point,
}

impl FromStr for Particle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(", ").map(Point::from_str).map(Result::unwrap);
        Ok(Self {
            position: points.next().unwrap(),
            velocity: points.next().unwrap(),
            acceleration: points.next().unwrap(),
        })
    }
}

#[cfg(test)]
mod day20_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";
        assert_eq!(0, pokey_little_puppy(input));
    }
}
