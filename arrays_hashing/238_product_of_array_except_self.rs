// This one might need a bit of work to make it nicer
impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut forward = nums[0];
        let mut backward = Vec::with_capacity(len);
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
                let temp = forward * nums[i];
                nums[i] = forward * backward[i + 1];
                forward = temp;
            }
        }
        nums
    }
}
