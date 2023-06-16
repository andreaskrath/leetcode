// 1. Two Sum
// https://leetcode.com/problems/two-sum/

fn main() {
    println!("Hello, world!");
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for outer in 0..nums.len() {
        for inner in outer + 1..nums.len() {
            // outer + 1 from loop above ensures that outer == inner is impossible
            // as such, there is no reason to do an if check for that
            if nums[outer] + nums[inner] == target {
                return vec![outer as i32, inner as i32];
            }
        }
    }

    // No solution; should not be possible
    vec![0, 0]
}

#[cfg(test)]
mod two_sum {
    use crate::two_sum;

    #[test]
    fn sample_one() {
        let input = vec![2, 7, 11, 15];
        let target = 9;

        let expected = vec![0, 1];
        let actual = two_sum(input, target);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = vec![3, 2, 4];
        let target = 6;

        let expected = vec![1, 2];
        let actual = two_sum(input, target);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = vec![3, 3];
        let target = 6;

        let expected = vec![0, 1];
        let actual = two_sum(input, target);
        assert_eq!(actual, expected);
    }
}
