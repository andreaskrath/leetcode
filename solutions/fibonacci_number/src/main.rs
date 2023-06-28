// 509. Fibonacci Number
// https://leetcode.com/problems/fibonacci-number/

fn main() {
    println!("Hello, world!");
}

fn fib(n: i32) -> i32 {
    let mut memo = vec![0, 1];
    for num in 2..=n {
        let num = num as usize;
        let new_fib = memo[num - 1] + memo[num - 2];
        memo.push(new_fib);
    }

    memo[n as usize]
}

#[cfg(test)]
mod tests {
    use crate::fib;

    #[test]
    fn sample_one() {
        let input = 2;
        let expected = 1;
        let actual = fib(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = 3;
        let expected = 2;
        let actual = fib(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_three() {
        let input = 4;
        let expected = 3;
        let actual = fib(input);
        assert_eq!(actual, expected);
    }
}
