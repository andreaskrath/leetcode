// 1768. Merge Strings Alternately
// https://leetcode.com/problems/merge-strings-alternately/

fn main() {
    println!("Hello, world!");
}

fn merge_alternately(word1: String, word2: String) -> String {
    let mut merged_str = String::new();
    let mut word1_iter = word1.chars().peekable();
    let mut word2_iter = word2.chars().peekable();

    // merging when both strings contain characters
    while word1_iter.peek().is_some() || word2_iter.peek().is_some() {
        if let Some(ch) = word1_iter.next() {
            merged_str.push(ch);
        }
        if let Some(ch) = word2_iter.next() {
            merged_str.push(ch);
        }
    }

    merged_str
}

#[cfg(test)]
mod tests {
    use crate::merge_alternately;

    #[test]
    fn sample_one() {
        let word1 = String::from("abc");
        let word2 = String::from("pqr");
        let expected = String::from("apbqcr");
        let actual = merge_alternately(word1, word2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let word1 = String::from("ab");
        let word2 = String::from("pqrs");
        let expected = String::from("apbqrs");
        let actual = merge_alternately(word1, word2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let word1 = String::from("abcd");
        let word2 = String::from("pq");
        let expected = String::from("apbqcd");
        let actual = merge_alternately(word1, word2);
        assert_eq!(actual, expected);
    }
}
