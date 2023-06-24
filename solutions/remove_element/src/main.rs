// 27. Remove Element
// https://leetcode.com/problems/remove-element/

fn main() {
    let mut input = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let res = remove_element(&mut input, 2);
    println!("result: {:#?}", res);
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let start_len = nums.len();
    let mut remove_indicies = Vec::new();

    for (index, num) in nums.iter().enumerate() {
        if *num == val {
            remove_indicies.push(index);
        }
    }

    println!("indicies to be removed: {:#?}", remove_indicies);
    for index in remove_indicies.iter().rev() {
        nums.swap_remove(*index);
    }

    (start_len - nums.len()) as i32
}

#[cfg(test)]
mod tests {
    use crate::remove_element;

    #[test]
    fn sample_one() {
        let mut input = vec![3, 2, 2, 3];
        let expected_vec = vec![2, 2];
        remove_element(&mut input, 3);
        assert_eq!(input, expected_vec);
        assert_eq!(input.len(), expected_vec.len());
    }

    #[test]
    fn sample_two() {
        let mut input = vec![0, 1, 2, 2, 3, 0, 4, 2];
        // expected does not match sample case due to unordering
        let expected_vec = vec![0, 1, 3, 0, 4];
        remove_element(&mut input, 2);
        assert_eq!(input, expected_vec);
        assert_eq!(input.len(), expected_vec.len());
    }
}
