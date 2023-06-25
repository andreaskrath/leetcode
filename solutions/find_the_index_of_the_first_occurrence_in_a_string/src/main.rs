// 28. Find the Index of the First Occurrence in a String
// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/

fn main() {
    println!("Hello, world!");
}

fn str_str(haystack: String, needle: String) -> i32 {
    let char_vec: Vec<char> = haystack.chars().collect();
    let target: Vec<char> = needle.chars().collect();
    
    for (index, slice) in char_vec.windows(needle.len()).enumerate() {
        if slice == target {
            return index as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use crate::str_str;

    #[test]
    fn sample_one() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");
        let expected = 0;
        let actual = str_str(haystack, needle);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let haystack = String::from("leetcode");
        let needle = String::from("leeto");
        let expected = -1;
        let actual = str_str(haystack, needle);
        assert_eq!(actual, expected);
    }
}
