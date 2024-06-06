// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.

use std::collections::HashMap;

// Version 01: 59ms
pub fn two_sum01(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let nums_enumerate: Vec<_> = nums.into_iter().enumerate().collect();

    for (x_index, x) in &nums_enumerate {
        for (y_index, y) in &nums_enumerate {
            if x_index == y_index {
                continue;
            }

            if x + y == target {
                return vec![x_index.clone() as i32, y_index.clone() as i32];
            }
        }
    }

    return vec![1, 2, 3];
}

// Version 02: 1ms
pub fn two_sum02(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::<&i32, usize>::new();

    for (x_index, x) in nums.iter().enumerate() {
        let diff = target - x;

        if let Some(seen_index) = seen.get(&diff) {
            if seen_index == &x_index {
                continue;
            }

            return vec![seen_index.clone() as i32, x_index as i32];
        }

        seen.insert(x, x_index);
    }

    vec![]
}

// Version 03: 1ms
pub fn two_sum03(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::<i32, usize>::new();

    for (x_index, x) in nums.iter().enumerate() {
        match seen.get(x) {
            Some(seen_value) => return vec![seen_value.clone() as i32, x_index as i32],
            None => {
                seen.insert(target - x, x_index);
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod ex001_test {
    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let result = two_sum03(nums, target);
        let index = vec![0, 1];

        assert_eq!(result, index);
    }

    #[test]
    fn case_2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        let result = two_sum03(nums, target);
        let index = vec![1, 2];

        assert_eq!(result, index);
    }

    #[test]
    fn case_3() {
        let nums = vec![3, 3];
        let target = 6;

        let result = two_sum03(nums, target);
        let index = vec![0, 1];

        assert_eq!(result, index);
    }

    #[test]
    fn case_4() {
        let nums = vec![3, 2, 3];
        let target = 6;

        let result = two_sum03(nums, target);
        let index = vec![0, 2];

        assert_eq!(result, index);
    }
}
