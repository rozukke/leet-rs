/// Complexity: O(n) => Guaranteed one full iteration through input
///
/// Here we use a trick based on the fact that we know the maximum amount of numbers, and can thus
/// create an array of frequencies. Each elemen in the array represents a number of times a number can
/// appear in the input, e.g. if the number 7 appears 3 times, it is added to the vector at index
/// 3. This array is inherently sorted, meaning that we can iterate through it in reverse to take
/// the first k largest elements, which would be the k most frequent elements. We do have to
/// flatten the inside vectors to deal with values that have duplicate frequencies.
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
