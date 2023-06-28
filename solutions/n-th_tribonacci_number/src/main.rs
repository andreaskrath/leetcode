// 1137. N-th Tribonacci Number
// https://leetcode.com/problems/n-th-tribonacci-number/

fn main() {
    println!("Hello, world!");
}

fn tribonacci(n: i32) -> i32 {
    let mut memo = vec![0, 1, 1];
    for num in 3..=n {
        let num = num as usize;
        let new_trib = memo[num - 1] + memo[num - 2] + memo[num - 3];
        memo.push(new_trib);
    }

    memo[n as usize]
}

#[cfg(test)]
mod tests {
    use crate::tribonacci;

    #[test]
    fn sample_one() {
        let input = 4;
        let expected = 4;
        let actual = tribonacci(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = 25;
        let expected = 1389537;
        let actual = tribonacci(input);
        assert_eq!(actual, expected);
    }
}
