use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = vec![Vec::new(); nums.len() + 1];

        // Count frequency
        let mut map = HashMap::new();
        for i in nums {
            *map.entry(i).or_insert(0) += 1;
        }

        // Bucket sort
        map.into_iter().for_each(|kv| freq[kv.1].push(kv.0));
        // Get K top elements
        freq.into_iter()
            .rev()
            .filter(|elem| elem.len() != 0)
            .flatten()
            .take(k as usize)
            .collect()
    }
}
