/// Complexity: O(n) => One iteration through input
///
/// The crux of the problem is finding the local mininum and then using it for future comparisons.
/// We will never miss the minimum because it is constantly being updated, and the max will always
/// keep track of the largest found value. The min and max are equivalent to an if-else.
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut min = prices[0];
        let mut max = 0;

        for i in 1..prices.len() {
            min = min.min(prices[i]);
            max = max.max(prices[i] - min);
        }
        max
    }
}
