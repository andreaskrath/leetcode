// 136. Single Number
// https://leetcode.com/problems/single-number/

fn main() {
    println!("Hello, world!");
}

use std::collections::HashMap;

fn single_number(nums: Vec<i32>) -> i32 {
    let mut seen_nums: HashMap<i32, u8> = HashMap::with_capacity(nums.len());
    for num in nums {
        seen_nums.entry(num).and_modify(|n| *n += 1).or_insert(1);
    }

    for (k, v) in seen_nums {
        if v == 1 {
            return k;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::single_number;

    #[test]
    fn sample_one() {
        let input = vec![2, 2, 1];
        let expected = 1;
        let actual = single_number(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = vec![4, 1, 2, 1, 2];
        let expected = 4;
        let actual = single_number(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = vec![1];
        let expected = 1;
        let actual = single_number(input);
        assert_eq!(actual, expected);
    }
}
