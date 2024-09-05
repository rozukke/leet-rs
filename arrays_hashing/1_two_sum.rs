use std::collections::HashMap;

/// Complexity: O(n) => 1 iteration through input in the worst case
///
/// We iterate and store the complement of each number we come across as well as its index. If we
/// then encounter the complement in the map, we can immediately return both indexes.
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // This should realistically be tuned to expected input size
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(50);

        for (i, x) in nums.into_iter().enumerate() {
            let i = i as i32;
            if let Some(val) = map.get(&x) {
                // val is idx of the complement number, i is idx of current number
                return vec![i, *val];
            } else {
                // Store complement of each number along with the index of the number
                map.insert(target - x, i);
            }
        }

        return vec![];
    }
}
