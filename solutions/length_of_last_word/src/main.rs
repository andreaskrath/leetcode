// 58. Length of Last Word
// https://leetcode.com/problems/length-of-last-word/description/

fn main() {
    println!("Hello, world!");
}

fn length_of_last_word(s: String) -> i32 {
    let mut word_len = 0;

    for ch in s.chars().rev() {
        if ch.is_ascii_whitespace() {
            if word_len > 0 {
                break;
            }
        } else {
            word_len += 1;
        }
    }

    word_len
}

#[cfg(test)]
mod tests {
    use crate::length_of_last_word;

    #[test]
    fn sample_one() {
        let input = String::from("Hello World");
        let expected = 5;
        let actual = length_of_last_word(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("   fly me   to   the moon  ");
        let expected = 4;
        let actual = length_of_last_word(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = String::from("luffy is still joyboy");
        let expected = 6;
        let actual = length_of_last_word(input);
        assert_eq!(actual, expected);
    }
}
