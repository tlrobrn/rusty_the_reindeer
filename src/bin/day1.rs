extern crate rusty_the_reindeer;

fn main() {
    let contents = rusty_the_reindeer::get_input()
        .expect("Must provide valid input path");

    println!("Part 1: {}", solve_captcha1(&contents));
    println!("Part 2: {}", solve_captcha2(&contents));
}

fn solve_captcha1(contents: &str) -> u32 {
    let digits = to_digits(contents);
    let length = digits.len();
    solve_captcha(digits, length, 1)
}

fn solve_captcha2(contents: &str) -> u32 {
    let digits = to_digits(contents);
    let length = digits.len();
    solve_captcha(digits, length, length / 2)
}

fn solve_captcha(digits: Vec<u32>, length: usize, step: usize) -> u32 {
    digits
        .iter()
        .enumerate()
        .fold(0, |total, (i, &n)| {
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
    contents
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}


#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn solve_captcha_works() {
        assert_eq!(3, solve_captcha1("1122"));
        assert_eq!(4, solve_captcha1("1111"));
        assert_eq!(0, solve_captcha1("1234"));
        assert_eq!(9, solve_captcha1("91212129"));
    }

    #[test]
    fn solve_captcha_2_works() {
        assert_eq!(6, solve_captcha2("1212"));
        assert_eq!(0, solve_captcha2("1221"));
        assert_eq!(4, solve_captcha2("123425"));
        assert_eq!(12,solve_captcha2( "123123"));
        assert_eq!(4, solve_captcha2("12131415"));
    }
}
