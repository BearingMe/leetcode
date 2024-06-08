use std::collections::HashMap;

// Version 01: 0ms, 2.24mb
pub fn create_loopup01() -> HashMap<char, i32> {
    let mut lookup = HashMap::<char, i32>::new();

    lookup.insert('O', 0);
    lookup.insert('I', 1);
    lookup.insert('V', 5);
    lookup.insert('X', 10);
    lookup.insert('L', 50);
    lookup.insert('C', 100);
    lookup.insert('D', 500);
    lookup.insert('M', 1000);

    lookup
}

pub fn roman_to_int01(s: String) -> i32 {
    let mut acc = 0;

    let lookup_table = create_loopup01();
    let char_set: Vec<char> = s.chars().collect();
    let char_set_length = char_set.len();

    for index in 0..char_set_length {
        let curr = char_set.get(index).unwrap_or(&'O');
        let next = char_set.get(index + 1).unwrap_or(&'O');

        let curr = lookup_table.get(curr).unwrap();
        let next = lookup_table.get(next).unwrap();

        if next > curr {
            acc -= curr;
        } else {
            acc += curr;
        }
    }

    acc
}

// Version 02: 0ms, 2.19mb
pub fn roman_to_int(s: String) -> i32 {
    let lookup: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut acc: i32 = 0;
    let mut prev: &i32 = &0;

    // inverse chart_set
    for char in s.chars().rev() {
        let curr = lookup.get(&char).unwrap();

        if curr >= prev {
            acc += curr;
            prev = curr;
            continue;
        }

        acc -= curr;
        prev = curr;
    }

    acc
}

#[cfg(test)]
mod ex013_test {
    use super::*;

    #[test]
    fn case_1() {
        let input = "III";
        let output = 3;

        let result = roman_to_int(input.to_string());

        assert_eq!(output, result);
    }

    #[test]
    fn case_2() {
        let input = "LVIII";
        let output = 58;

        let result = roman_to_int(input.to_string());

        assert_eq!(output, result);
    }

    #[test]
    fn case_3() {
        let input = "MCMXCIV";
        let output = 1994;

        let result = roman_to_int(input.to_string());

        assert_eq!(output, result);
    }
}
