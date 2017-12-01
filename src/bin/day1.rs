extern crate rusty_the_reindeer;

fn main() {
    let contents = rusty_the_reindeer::get_input()
        .expect("Must provide valid input path");
    println!("{}", solve_captcha(&contents));
}

fn solve_captcha(contents: &str) -> u32 {
    let mut digits: Vec<u32> = contents
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let first = digits[0];
    digits.push(first);

    digits.windows(2).fold(0, |total, numbers| {
        if numbers[0] == numbers[1] {
            total + numbers[0]
        } else {
            total
        }
    })
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn solve_captcha_works() {
        assert_eq!(3, solve_captcha("1122"));
        assert_eq!(4, solve_captcha("1111"));
        assert_eq!(0, solve_captcha("1234"));
        assert_eq!(9, solve_captcha("91212129"));
    }
}
