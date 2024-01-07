struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut price = 666666;

        for p in prices.into_iter() {
            profit = std::cmp::max(profit, p - price);
            price = std::cmp::min(price, p);
        }

        profit
    }
}

pub fn run(execute: bool) {
    if !execute {
        return;
    }
    let input = vec![3, 2, 6, 5, 0, 3];
    println!("input: {:?}", &input);
    let r = Solution::max_profit(input);
    println!("result: {r}");
}
