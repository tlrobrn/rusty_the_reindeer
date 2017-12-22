extern crate rusty_the_reindeer;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = final_steps(contents.trim());
    let part2 = max_steps(contents.trim());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

type Coordinate = (i64, i64, i64);

fn final_steps(contents: &str) -> u64 {
    build_path(contents).last().map_or(0, distance)
}

fn max_steps(contents: &str) -> u64 {
    build_path(contents).iter().map(distance).max().unwrap()
}

fn build_path(contents: &str) -> Vec<Coordinate> {
    contents.split(',')
        .fold(vec![(0, 0, 0)], |mut positions, direction| {
            let &(x, y, z) = positions.last().unwrap();

            let next_position = match direction {
                "n" => (x, y + 1, z - 1),
                "ne" => (x + 1, y, z - 1),
                "se" => (x + 1, y - 1, z),
                "s" => (x, y - 1, z + 1),
                "sw" => (x - 1, y, z + 1),
                "nw" => (x - 1, y + 1, z),
                _ => (x, y, z),
            };

            positions.push(next_position);
            positions
        })
}

fn distance(coordinate: &Coordinate) -> u64 {
    let &(x, y, z) = coordinate;
    x.abs().max(y.abs()).max(z.abs()) as u64
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(3, final_steps("ne,ne,ne"));
        assert_eq!(0, final_steps("ne,ne,sw,sw"));
        assert_eq!(2, final_steps("ne,ne,s,s"));
        assert_eq!(3, final_steps("se,sw,se,sw,sw"));
    }
}
