use std::collections::HashSet;

impl Solution {
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
