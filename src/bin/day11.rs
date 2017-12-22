extern crate rusty_the_reindeer;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let part1 = steps(contents.trim());

    println!("Part 1: {}", part1);
}

fn steps(contents: &str) -> u64 {
    let (x, y, z): (i64, i64, i64) = contents.split(',')
        .fold((0, 0, 0), |(x, y, z), direction| {
            match direction {
                "n" => (x, y + 1, z - 1),
                "ne" => (x + 1, y, z - 1),
                "se" => (x + 1, y - 1, z),
                "s" => (x, y - 1, z + 1),
                "sw" => (x - 1, y, z + 1),
                "nw" => (x - 1, y + 1, z),
                _ => (x, y, z),
            }
        });

    x.abs().max(y.abs()).max(z.abs()) as u64
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(3, steps("ne,ne,ne"));
        assert_eq!(0, steps("ne,ne,sw,sw"));
        assert_eq!(2, steps("ne,ne,s,s"));
        assert_eq!(3, steps("se,sw,se,sw,sw"));
    }
}
