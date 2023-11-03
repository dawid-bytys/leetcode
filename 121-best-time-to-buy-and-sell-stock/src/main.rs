struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut current_min_price = prices[0];
        let mut current_profit = 0;

        for i in 1..prices.len() {
            current_profit = std::cmp::max(current_profit, prices[i] - current_min_price);
            current_min_price = std::cmp::min(current_min_price, prices[i]);
        }

        current_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
