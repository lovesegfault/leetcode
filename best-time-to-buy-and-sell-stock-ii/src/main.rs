struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                max += prices[i] - prices[i - 1];
            }
        }
        max
    }
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut valley;
        let mut peak;
        let mut max = 0;
        while i < prices.len().saturating_sub(1) {
            while i < prices.len() - 1 && prices[i] >= prices[i + 1] {
                i += 1;
            }
            valley = prices[i];
            while i < prices.len() - 1 && prices[i] <= prices[i + 1] {
                i += 1;
            }
            peak = prices[i];
            max += peak - valley;
        }
        max
    }
}

fn main() {
    println!("Hello, world!");
}
