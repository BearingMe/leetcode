use std::cmp;

// Version 01: 1ms
pub fn compare_two_words(a: String, b: String) -> String {
    let mut prefix = String::from("");
    let min_length = cmp::min(a.len(), b.len());

    if min_length == 0 {
        return prefix;
    }

    let zipped = a.chars().zip(b.chars());

    for (char_a, char_b) in zipped {
        if char_a != char_b {
            break;
        }

        prefix.push(char_a);
    }

    prefix
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].clone();
    }

    return strs
        .into_iter()
        .reduce(compare_two_words)
        .unwrap_or_default();
}

#[cfg(test)]
mod ex014_test {
    use super::*;

    #[test]
    fn case_1() {
        let input = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let output = "fl".to_string();

        let result = longest_common_prefix(input);

        assert_eq!(output, result);
    }

    #[test]
    fn case_2() {
        let input = vec![
            "dog".to_string(), //
            "racecar".to_string(),
            "car".to_string(),
        ];
        let output = "".to_string();

        let result = longest_common_prefix(input);

        assert_eq!(output, result);
    }

    #[test]
    fn case_3() {
        let input = vec!["".to_string()];
        let output = "".to_string();

        let result = longest_common_prefix(input);

        assert_eq!(output, result);
    }

    #[test]
    fn case_4() {
        let input = vec!["ab".to_string(), "a".to_string()];
        let output = "a".to_string();

        let result = longest_common_prefix(input);

        assert_eq!(output, result);
    }

    #[test]
    fn case_5() {
        let input = vec!["a".to_string()];
        let output = "a".to_string();

        let result = longest_common_prefix(input);

        assert_eq!(output, result);
    }
}
