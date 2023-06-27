// 191. Number of 1 Bits
// https://leetcode.com/problems/number-of-1-bits/

fn main() {
    println!("Hello, world!");
}

fn hammingWeight(n: u32) -> i32 {
    let bit_mask = 0b1;
    let mut num = n;
    let mut counter = 0;
    for _ in 0..32 {
        if num & bit_mask == bit_mask {
            counter += 1;
        }
        // unsigned r-shift fills with 0s, not leading bit
        num >>= 1;
    }

    counter
}

#[cfg(test)]
mod tests {
    use crate::hammingWeight;

    #[test]
    fn sample_one() {
        let input = 11;
        let expected = 3;
        let actual = hammingWeight(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = 128;
        let expected = 1;
        let actual = hammingWeight(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = 4294967293;
        let expected = 31;
        let actual = hammingWeight(input);
        assert_eq!(actual, expected);
    }
}
