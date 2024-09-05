// TODO: This one might need a bit of work to make it nicer

/// Complexity: O(n) => 2 full iterations through input
///
/// To get the product of all values except the current one, we can keep track of the
/// multiplication in both directions of the array. For example:
///
/// input     = [1,  2,  3,  4 ]
/// forwards  = [1,  2,  6,  24]
/// backwards = [24, 24, 12, 4 ]
///
/// Notice that to get the product of input[2], we can take the surrounding values from both
/// directions and multiply them:
///
/// product(input[2]) = forwards[1] * backwards[3] = 2 * 4 = 8
///
/// We apply this by calculating the entire backwards order ahead of time, and keeping track of the
/// forward sum while we perform the final calculation to avoid an extra iteration and n memory
/// cost.
impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut forward = nums[0];
        // Calculate backwards product ahead of time
        let mut backward = Vec::with_capacity(len);
        // SAFETY: Immediately set afterwards
        unsafe {
            backward.set_len(len);
        }
        backward[len - 1] = nums[len - 1];
        for i in (0..len - 1).rev() {
            backward[i] = nums[i] * backward[i + 1];
        }

        for i in 0..len {
            if i == 0 {
                nums[i] = backward[i + 1];
            } else if i == len - 1 {
                nums[i] = forward;
            } else {
                // Keep track of forward sum as we go
                let temp = forward * nums[i];
                nums[i] = forward * backward[i + 1];
                forward = temp;
            }
        }
        nums
    }
}
