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
