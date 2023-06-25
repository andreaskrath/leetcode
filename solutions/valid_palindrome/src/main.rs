// 125. Valid Palindrome
// https://leetcode.com/problems/valid-palindrome/

fn main() {
    println!("Hello, world!");
}

fn is_palindrome(s: String) -> bool {
    let mut new_str = String::new();
    for ch in s.chars() {
        if ch.is_alphanumeric() {
            new_str.push(ch.to_ascii_lowercase());
        }
    }

    let half = new_str.len() / 2;
    new_str
        .bytes()
        .take(half)
        .eq(new_str.bytes().rev().take(half))
}

#[cfg(test)]
mod tests {
    use crate::is_palindrome;

    #[test]
    fn sample_one() {
        let input = String::from("A man, a plan, a canal: Panama");
        let result = is_palindrome(input);
        assert!(result);
    }

    #[test]
    fn sample_two() {
        let input = String::from("race a car");
        let result = is_palindrome(input);
        assert!(!result);
    }

    #[test]
    fn sample_three() {
        let input = String::from(" ");
        let result = is_palindrome(input);
        assert!(result);
    }
}
