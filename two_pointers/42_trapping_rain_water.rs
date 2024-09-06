/// Complexity: O(n) => Maximum one full iteration through input
///
/// Very annoying problem with an annoyingly simple solution. Any single point in the input only
/// depends on the maximum on either side of it to calculate how much it can store. We can take the
/// maximum on both sides, get the smallest one, and use that as the value to 'fill' until. The
/// elegant part comes in the ability to calculate these maximums on the fly, and switch between
/// the two sides based on whichever one is smaller. What this allows to happen is to have a
/// guarantee that the other side is already greater than or equal to the current side, simplifying
/// the condition checks by a lot.
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut sum = 0;
        // Set up pointers
        let mut r = height.len() - 1;
        let mut l = 0;
        let mut max_r = height[r];
        let mut max_l = height[l];

        while l < r {
            // Update pointers to increase mininum
            if max_l < max_r {
                l += 1;
                max_l = max_l.max(height[l]);
                // max_l is guaranteed smaller or equal to max_r
                sum += max_l - height[l];
            } else {
                r -= 1;
                max_r = max_r.max(height[r]);
                sum += max_r - height[r];
            }
        }
        sum
    }
}
