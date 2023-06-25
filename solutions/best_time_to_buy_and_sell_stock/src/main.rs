// 121. Best Time to Buy and Sell Stock
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

fn main() {
    println!("Hello, world!");
}

fn max_profit(prices: Vec<i32>) -> i32 {
    // max value is 10^4 = 10000
    let mut minimum = 10001;
    let mut max_profit = 0;
    for stock in prices {
        minimum = minimum.min(stock);
        let price_today = stock - minimum;
        max_profit = max_profit.max(price_today);
    }

    max_profit
}

#[cfg(test)]
mod tests {
    use crate::max_profit;

    #[test]
    fn sample_one() {
        let input = vec![7, 1, 5, 3, 6, 4];
        let expected = 5;
        let actual = max_profit(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_two() {
        let input = vec![7, 6, 4, 3, 1];
        let expected = 0;
        let actual = max_profit(input);
        assert_eq!(actual, expected);
    }
}
