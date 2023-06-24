// 20. Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/

fn main() {
    println!("Hello, world!");
}

fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    // somewhat of a simplified state machine (pda, in this case)
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if let Some(stack_ch) = stack.pop() {
                    if stack_ch != '(' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            ']' => {
                if let Some(stack_ch) = stack.pop() {
                    if stack_ch != '[' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            '}' => {
                if let Some(stack_ch) = stack.pop() {
                    if stack_ch != '{' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }

    // if the stack is not empty, the parentheses were not closed correctly
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use crate::is_valid;

    #[test]
    fn sample_one() {
        let input = String::from("()");
        let result = is_valid(input);
        assert!(result);
    }

    #[test]
    fn sample_two() {
        let input = String::from("()[]{}");
        let result = is_valid(input);
        assert!(result);
    }

    #[test]
    fn sample_three() {
        let input = String::from("(]");
        let result = is_valid(input);
        assert!(!result);
    }
}
