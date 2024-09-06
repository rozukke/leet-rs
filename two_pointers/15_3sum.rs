/// Complexity: O(n*log(n)) => Sorting overwrites the complexity of the two-sum algo.
///
/// By sorting, we can use the algorithm from Two Sum II (sorted input) and set the target to be
/// the next non-repeated element in the sequence. Sucks to write, but it's decently fast.
use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        // Sort to make algorithm easier
        nums.sort_unstable();

        let mut i = 0;
        while i < nums.len() - 2 {
            // Do twosum on remaining value
            let first = nums[i];
            let mut front = i + 1;
            let mut back = nums.len() - 1;

            // Move pointers to find all valid sums
            while front < back {
                let sum = first + nums[front] + nums[back];
                match sum.cmp(&0) {
                    Ordering::Less => front += 1,
                    Ordering::Greater => back -= 1,
                    Ordering::Equal => {
                        // Save result
                        res.push(vec![first, nums[front], nums[back]]);
                        // Get rid of repeated elements
                        while front < back && nums[front] == nums[front + 1] {
                            front += 1;
                        }
                        // Final iteration to go to next
                        front += 1;
                    },
                }
            }

            // Skip same number
            while i < nums.len() - 1 && nums[i] == nums[i + 1] {
                i += 1;
            }
            i += 1;
        }
        res
    }
}
