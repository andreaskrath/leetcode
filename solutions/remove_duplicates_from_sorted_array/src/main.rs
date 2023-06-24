// 26. Remove Duplicates from Sorted Array
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

fn main() {
    println!("Hello, world!");
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use crate::remove_duplicates;

    #[test]
    fn sample_one() {
        let mut input = vec![1, 1, 2];
        let expected_vec = vec![1, 2];
        remove_duplicates(&mut input);
        assert_eq!(input, expected_vec);
        assert_eq!(input.len(), expected_vec.len());
    }

    #[test]
    fn sample_two() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let expected_vec = vec![0, 1, 2, 3, 4];
        remove_duplicates(&mut input);
        assert_eq!(input, expected_vec);
        assert_eq!(input.len(), expected_vec.len());
    }
}
