extern crate rusty_the_reindeer;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let (part1, part2) = multi_solve_captcha(&contents);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn multi_solve_captcha(contents: &str) -> (u32, u32) {
    let digits = to_digits(contents);
    let length = digits.len();
    let half = length / 2;

    digits
        .iter()
        .enumerate()
        .fold((0, 0), |(total1, total2), (i, &n)| {
            match (
                digits[(i + 1) % length] == n,
                digits[(i + half) % length] == n,
            ) {
                (true, true) => (total1 + n, total2 + n),
                (true, false) => (total1 + n, total2),
                (false, true) => (total1, total2 + n),
                (false, false) => (total1, total2),
            }
        })
}

fn to_digits(contents: &str) -> Vec<u32> {
    contents.chars().map(|c| c.to_digit(10).unwrap()).collect()
}


#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn to_digits_returns_vec_of_digits() {
        assert_eq!(vec![1, 1, 2, 2], to_digits("1122"));
    }
    #[test]
    fn part1() {
        let (x, _) = multi_solve_captcha("1122");
        assert_eq!(3, x);
        let (x, _) = multi_solve_captcha("1111");
        assert_eq!(4, x);
        let (x, _) = multi_solve_captcha("1234");
        assert_eq!(0, x);
        let (x, _) = multi_solve_captcha("91212129");
        assert_eq!(9, x);
    }

    #[test]
    fn part2() {
        let (_, x) = multi_solve_captcha("1212");
        assert_eq!(6, x);
        let (_, x) = multi_solve_captcha("1221");
        assert_eq!(0, x);
        let (_, x) = multi_solve_captcha("123425");
        assert_eq!(4, x);
        let (_, x) = multi_solve_captcha("123123");
        assert_eq!(12, x);
        let (_, x) = multi_solve_captcha("12131415");
        assert_eq!(4, x);
    }
}
