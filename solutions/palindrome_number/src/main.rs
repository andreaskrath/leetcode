// 9. Palindrome Number
// https://leetcode.com/problems/palindrome-number/

fn main() {}

fn is_palindrome(x: i32) -> bool {
    let str_num = x.to_string();
    let half = str_num.len() / 2;

    str_num
        .bytes()
        .take(half)
        .eq(str_num.bytes().rev().take(half))
}

#[cfg(test)]
mod tests {
    use crate::is_palindrome;

    #[test]
    fn sample_one() {
        let input = 121;
        assert!(is_palindrome(input));
    }

    #[test]
    fn sample_two() {
        let input = -121;
        assert!(!is_palindrome(input));
    }

    #[test]
    fn sample_three() {
        let input = 10;
        assert!(!is_palindrome(input));
    }
}
