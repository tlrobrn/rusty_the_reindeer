extern crate rusty_the_reindeer;

fn main() {
    let contents = rusty_the_reindeer::get_input()
        .expect("Must provide valid input path");
    println!("Part 1: {}", solve_captcha(&contents));
    println!("Part 2: {}", solve_captcha2(&contents));
}

fn solve_captcha(contents: &str) -> u32 {
    let mut digits = to_digits(contents);

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

fn solve_captcha2(contents: &str) -> u32 {
    let digits = to_digits(contents);
    let length = digits.len();
    let step = length / 2;

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
        assert_eq!(3, solve_captcha("1122"));
        assert_eq!(4, solve_captcha("1111"));
        assert_eq!(0, solve_captcha("1234"));
        assert_eq!(9, solve_captcha("91212129"));
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
