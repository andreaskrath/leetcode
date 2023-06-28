// 1431. Kids With the Greatest Number of Candies
// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/

fn main() {
    println!("Hello, world!");
}

fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_candies = *candies.iter().max().expect("failed to get max");
    let mut new_vec = Vec::with_capacity(candies.len());

    for candy in candies {
        let boolean = (candy + extra_candies) >= max_candies;
        new_vec.push(boolean);
    }

    new_vec
}

#[cfg(test)]
mod tests {
    use crate::kids_with_candies;

    #[test]
    fn sample_one() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        let expected = vec![true, true, true, false, true];
        let actual = kids_with_candies(candies, extra_candies);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let candies = vec![4, 2, 1, 1, 2];
        let extra_candies = 1;
        let expected = vec![true, false, false, false, false];
        let actual = kids_with_candies(candies, extra_candies);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let candies = vec![12, 1, 12];
        let extra_candies = 10;
        let expected = vec![true, false, true];
        let actual = kids_with_candies(candies, extra_candies);
        assert_eq!(actual, expected);
    }
}
