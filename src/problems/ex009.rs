// Given an integer x, return true if x is a
// palindrome, and false otherwise.

// Version 01: 11ms, 2.14mb
pub fn invert01(x: i32) -> Result<i32, std::num::ParseIntError> {
    let x_string = x.to_string();
    let x_chars = x_string.chars();
    let inverse_string = x_chars.into_iter().rev().collect::<String>();

    dbg!("Inverse string, {inverse_string}");

    inverse_string.parse::<i32>()
}

pub fn is_palindrome01(x: i32) -> bool {
    match invert01(x) {
        Ok(value) => return x == value,
        Err(err) => {
            dbg!(err);
            false
        }
    }
}

// Version 02: 7ms, 2.31mb
pub fn is_palindrome02(x: i32) -> bool {
    let x_string = x.to_string();
    let x_chars = x_string.chars().collect::<Vec<_>>();
    let length = x_chars.len();

    for i in 0..length {
        if x_chars[i] != x_chars[length - 1 - i] {
            return false;
        }
    }

    true
}

// Version 03: 0ms, 2.12mb
pub fn is_palindrome03(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut x_temp = x;
    let mut y = 0;

    while x_temp > 0 {
        y *= 10;

        let last_digit = x_temp % 10;
        y += last_digit;

        x_temp /= 10;
    }

    x == y
}

// Version 04: 4ms, 2.70mb
fn count_digits(n: i32) -> u32 {
    (n as f64).log10().floor() as u32 + 1
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let digit_count = count_digits(x);

    for i in 0..digit_count / 2 {
        let left = x / 10_i32.pow(i) % 10;
        let right = x / 10_i32.pow(digit_count - i - 1) % 10;

        if left != right {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod ex009_test {
    use super::*;

    #[test]
    fn case_1() {
        let input = 121;
        let output = true;

        let result = is_palindrome(input);

        assert_eq!(result, output);
    }

    #[test]
    fn case_2() {
        let input = -121;
        let output = false;

        let result = is_palindrome(input);

        assert_eq!(result, output);
    }

    #[test]
    fn case_3() {
        let input = 10;
        let output = false;

        let result = is_palindrome(input);

        assert_eq!(result, output);
    }
}
