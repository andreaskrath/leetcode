// 190. Reverse Bits
// https://leetcode.com/problems/reverse-bits/

fn main() {
    println!("Hello, world!");
}

// rust does feature a reverse_bits method, however that makes the challenge trivial
fn reverse_bits(x: u32) -> u32 {
    let bit_mask = 0b1;
    let mut num = x;
    let mut bin_str = String::with_capacity(32);

    for _ in 0..32 {
        if num & bit_mask == bit_mask {
            bin_str.push('1');
        } else {
            bin_str.push('0');
        }
        num >>= 1
    }

    u32::from_str_radix(bin_str.as_str(), 2).expect("failed to parse bin_str")
}

#[cfg(test)]
mod tests {
    use crate::reverse_bits;

    #[test]
    fn sample_one() {
        let input = 43261596;
        let expected = 964176192;
        let actual = reverse_bits(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = 4294967293;
        let expected = 3221225471;
        let actual = reverse_bits(input);
        assert_eq!(actual, expected);
    }
}
