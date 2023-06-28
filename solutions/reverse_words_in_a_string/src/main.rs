// 151. Reverse Words in a String
// https://leetcode.com/problems/reverse-words-in-a-string/

fn main() {
    println!("Hello, world!");
}

fn reverse_words(s: String) -> String {
    s.split_whitespace()
        .rfold(String::new(), |mut state, s| {
            state.reserve(s.len() + 1);
            state.push_str(s);
            state.push(' ');
            state
        })
        .trim_end()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::reverse_words;

    #[test]
    fn sample_one() {
        let input = String::from("the sky is blue");
        let expected = String::from("blue is sky the");
        let actual = reverse_words(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = String::from("  hello world  ");
        let expected = String::from("world hello");
        let actual = reverse_words(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = String::from("a good   example");
        let expected = String::from("example good a");
        let actual = reverse_words(input);
        assert_eq!(actual, expected);
    }
}
