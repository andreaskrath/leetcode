// 169. Majority Element
// https://leetcode.com/problems/majority-element/

fn main() {
    println!("Hello, world!");
}

use std::collections::HashMap;

fn majority_element(nums: Vec<i32>) -> i32 {
    let mut numbers = HashMap::new();
    for num in nums {
        numbers.entry(num).and_modify(|n| *n += 1).or_insert(1);
    }

    let mut most_seen_key = 0;
    let mut most_seen_val = 0;
    for (k, v) in numbers {
        if v > most_seen_val {
            most_seen_key = k;
            most_seen_val = v;
        }
    }

    most_seen_key
}

#[cfg(test)]
mod tests {
    use crate::majority_element;

    #[test]
    fn sample_one() {
        let input = vec![3, 2, 3];
        let expected = 3;
        let actual = majority_element(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = vec![2, 2, 1, 1, 1, 2, 2];
        let expected = 2;
        let actual = majority_element(input);
        assert_eq!(actual, expected);
    }
}
