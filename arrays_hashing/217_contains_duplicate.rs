/// Complexity: O(n) => Worst case 1 full iteration
///
/// This is as simple as keeping track of what numbers we have encountered using a hash set, where
/// each insertion returns the existence of that number before it was inserted.
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut res: HashSet<i32> = HashSet::new();

        for n in nums {
            match res.insert(n) {
                false => return true,
                true => continue,
            }
        }
        false
    }
}
