use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(50);

        for (i, x) in nums.into_iter().enumerate() {
            let i = i.try_into().unwrap();
            if let Some(val) = map.get(&x) {
                return vec![i, *val];
            } else {
                map.insert(target - x, i);
            }
        }

        return vec![];
    }
}
