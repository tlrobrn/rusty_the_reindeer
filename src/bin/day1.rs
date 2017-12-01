extern crate rusty_the_reindeer;

fn main() {
    let contents = rusty_the_reindeer::get_input().expect("Must provide valid input path");
    let digits = to_digits(&contents);

    println!("Part 1: {}", solve_captcha1(&digits));
    println!("Part 2: {}", solve_captcha2(&digits));
}

fn solve_captcha1(digits: &Vec<u32>) -> u32 {
    solve_captcha(digits, 1)
}

fn solve_captcha2(digits: &Vec<u32>) -> u32 {
    solve_captcha(digits, digits.len() / 2)
}

fn solve_captcha(digits: &Vec<u32>, step: usize) -> u32 {
    let length = digits.len();

    digits.iter().enumerate().fold(0, |total, (i, &n)| {
        let lookup = if i + step < length {
            i + step
        } else {
            i + step - length
        };

        if n == digits[lookup] {
            total + n
        } else {
            total
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
    fn solve_captcha_works() {
        assert_eq!(3, solve_captcha1(&to_digits("1122")));
        assert_eq!(4, solve_captcha1(&to_digits("1111")));
        assert_eq!(0, solve_captcha1(&to_digits("1234")));
        assert_eq!(9, solve_captcha1(&to_digits("91212129")));
    }

    #[test]
    fn solve_captcha_2_works() {
        assert_eq!(6, solve_captcha2(&to_digits("1212")));
        assert_eq!(0, solve_captcha2(&to_digits("1221")));
        assert_eq!(4, solve_captcha2(&to_digits("123425")));
        assert_eq!(12, solve_captcha2(&to_digits("123123")));
        assert_eq!(4, solve_captcha2(&to_digits("12131415")));
    }
}
