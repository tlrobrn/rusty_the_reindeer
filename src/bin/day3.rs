extern crate rusty_the_reindeer;

use std::str::FromStr;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let contents = contents.trim();
    let part1 = step_counter(contents);
    let part2 = stress_test(contents);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn step_counter(contents: &str) -> i64 {
    let number = i64::from_str(contents).unwrap();
    if number == 1 {
        return 0;
    }

    let ring = ring_of(number);
    let mut coords = starting_coords_of(ring);

    for n in start_of(ring)..end_of(ring) {
        if n == number {
            let (x, y) = coords;
            return x.abs() + y.abs();
        }

        coords = match coords {
            (x, y) if x == ring && y == ring => (ring - 1, ring),
            (x, y) if x == -ring && y == ring => (-ring, ring - 1),
            (x, y) if x == -ring && y == -ring => (-ring + 1, -ring),
            (x, y) if x == ring && y == -ring => (ring + 1, -ring),
            (x, y) if x == ring => (x, y + 1),
            (x, y) if y == ring => (x - 1, y),
            (x, y) if x == -ring => (x, y - 1),
            (x, y) if y == -ring => (x + 1, y),
            _ => panic!("Invalid ring coordinates"),
        }
    }

    0
}

fn ring_of(number: i64) -> i64 {
    let float = number as f64;
    ((float.sqrt() - 1.0) / 2.0).ceil() as i64
}

fn start_of(ring: i64) -> i64 {
    if ring == 0 {
        1
    } else {
        (width_of(ring - 1)).pow(2) + 1
    }
}

fn end_of(ring: i64) -> i64 {
    start_of(ring + 1)
}

fn width_of(ring: i64) -> i64 {
    ring * 2 + 1
}

fn starting_coords_of(ring: i64) -> (i64, i64) {
    (ring, -ring + 1)
}

fn stress_test(contents: &str) -> u64 {
    use spiral::{neighbors, next_coordinate, Spiral};

    let number = u64::from_str(contents).unwrap();
    let mut spiral = Spiral::default();
    spiral.push(1);

    loop {
        if let Some(&value) = spiral.last() {
            if value > number {
                return value;
            }
        }

        if let Some(coordinate) = next_coordinate(spiral.last_coordinate()) {
            let value = neighbors(coordinate).iter().fold(0, |total, coordinate| {
                total + spiral.get_by_coordinate(coordinate).unwrap_or(&0)
            });

            spiral.push(value);
        }
    }
}

mod spiral {
    use std::collections::HashMap;

    type Coordinate = (i64, i64);

    #[derive(Default)]
    pub struct Spiral<T> {
        data: Vec<T>,
        last_coordinate: Option<Coordinate>,
        coordinate_map: HashMap<Coordinate, usize>,
    }

    impl<T> Spiral<T> {
        pub fn push(&mut self, value: T) -> Coordinate {
            let coordinate = next_coordinate(self.last_coordinate).unwrap();

            self.data.push(value);
            self.coordinate_map.insert(coordinate, self.data.len());
            self.last_coordinate = Some(coordinate);

            coordinate
        }

        pub fn get_by_index(&self, index: usize) -> Option<&T> {
            if index == 0 {
                None
            } else {
                self.data.get(index - 1)
            }
        }

        pub fn get_by_coordinate(&self, coordinate: &Coordinate) -> Option<&T> {
            self.coordinate_map
                .get(coordinate)
                .and_then(|&index| self.get_by_index(index))
        }

        pub fn last_coordinate(&self) -> Option<Coordinate> {
            self.last_coordinate
        }

        pub fn last(&self) -> Option<&T> {
            self.data.last()
        }
    }

    pub fn next_coordinate(current_coordinate: Option<Coordinate>) -> Option<Coordinate> {
        match current_coordinate {
            None => Some((0, 0)),
            Some((x, y)) => {
                let boundary = x.abs().max(y.abs());
                match (x, y) {
                    (0, 0) => Some((1, 0)),
                    (a, b) if a == boundary && b == boundary => Some((boundary - 1, boundary)),
                    (a, b) if a == -boundary && b == boundary => Some((-boundary, boundary - 1)),
                    (a, b) if a == -boundary && b == -boundary => Some((-boundary + 1, -boundary)),
                    (a, b) if a == boundary && b == -boundary => Some((boundary + 1, -boundary)),
                    (a, b) if a == boundary => Some((a, b + 1)),
                    (a, b) if b == boundary => Some((a - 1, b)),
                    (a, b) if a == -boundary => Some((a, b - 1)),
                    (a, b) if b == -boundary => Some((a + 1, b)),
                    _ => None,
                }
            }
        }
    }

    pub fn neighbors(coordinate: Coordinate) -> Vec<Coordinate> {
        let mut neighbors = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                if x != 0 || y != 0 {
                    neighbors.push((coordinate.0 + x, coordinate.1 + y));
                }
            }
        }
        neighbors
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_push() {
            let mut spiral = Spiral::default();
            assert_eq!((0, 0), spiral.push(1));
            assert_eq!((1, 0), spiral.push(2));
            assert_eq!((1, 1), spiral.push(3));
        }

        #[test]
        fn test_get_by_index_is_one_based() {
            let mut spiral = Spiral::default();
            spiral.push(false);

            assert_eq!(None, spiral.get_by_index(0));
            assert_eq!(Some(&false), spiral.get_by_index(1));
        }

        #[test]
        fn test_get_by_index_empty_spiral() {
            let spiral = Spiral::<bool>::default();
            assert_eq!(None, spiral.get_by_index(1));
        }

        #[test]
        fn test_get_by_coordinate() {
            let mut spiral = Spiral::default();
            assert_eq!((0, 0), spiral.push(1));
            assert_eq!((1, 0), spiral.push(2));
            assert_eq!((1, 1), spiral.push(3));
            assert_eq!(Some(&3), spiral.get_by_coordinate(&(1, 1)));
        }

        #[test]
        fn test_get_by_coordinate_empty_spiral() {
            let spiral = Spiral::<bool>::default();
            assert_eq!(None, spiral.get_by_coordinate(&(1, 1)));
        }

        #[test]
        fn test_neighbors() {
            let mut expected = vec![
                (-1, 1),
                (0, 1),
                (1, 1),
                (-1, 0),
                (1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ];
            expected.sort();

            let mut actual = neighbors((0, 0));
            actual.sort();

            assert_eq!(expected, actual);
        }
    }
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(0, step_counter("1"));
        assert_eq!(3, step_counter("12"));
        assert_eq!(2, step_counter("23"));
        assert_eq!(31, step_counter("1024"));
    }
}
