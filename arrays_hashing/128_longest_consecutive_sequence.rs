/// Complexity: O(n) => worst case of 2 full iterations
///
/// We first collect into a set for O(1) search, and then to iterate through the set to find valid
/// sequences. Sequence starts are any number i where i - 1 is not present in the set. We then count
/// up until the sequence ends, and find the maximum of all sequences.
use std::collections::HashSet;

impl Solution {
    // TODO: Radix sort solution also available
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<_> = nums.into_iter().collect();

        // If a number - 1 is not present it e is the start of a sequence
        set.iter()
            .filter(|&e| !set.contains(&(e - 1)))
            .map(|&i| (i..).take_while(|e| set.contains(e)).count())
            .max()
            .unwrap_or(0) as i32
    }
}
