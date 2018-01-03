extern crate rusty_the_reindeer;

use std::str::FromStr;
use std::ops::AddAssign;
use std::collections::HashMap;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = pokey_little_puppy(&contents);
    println!("Part 1: {}", part1);
    let part2 = survive(&contents);
    println!("Part 2: {}", part2);
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

fn survive(contents: &str) -> usize {
    let mut particles: Vec<_> = contents
        .lines()
        .map(Particle::from_str)
        .map(Result::unwrap)
        .collect();

    for _ in 0..1000 {
        let mut counter: HashMap<Point, usize> = HashMap::new();
        for particle in &mut particles {
            particle.tick();
            let entry = counter.entry(particle.position).or_insert(0);
            *entry += 1;
        }
        particles.retain(|particle| counter[&particle.position] == 1);
    }

    particles.len()
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Point {
    pub fn manhatten_distance(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

#[derive(Clone)]
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

impl Particle {
    pub fn tick(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
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

    #[test]
    fn part2() {
        let input = "p=<-6,0,0>, v=< 3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=<0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=<0,0,0>";
        assert_eq!(1, survive(input));
    }
}
