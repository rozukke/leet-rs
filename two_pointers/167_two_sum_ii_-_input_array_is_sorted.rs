/// Complexity: O(n) => Technically half of an iteration through input
///
/// Start with the largest element on the left, then iterate through by decrementing
/// until we reach a number on the left side that can no longer fit the target, repeat until
/// found. There can be an optimization made in regards to finding a number at the back of the
/// array that can actually contain the target if the target is small.
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut back = numbers.len() - 1;
        let mut front = 0;

        // Loop will never break as a solution is guaranteed
        loop {
            let cur_sum = numbers[front] + numbers[back];
            if cur_sum == target {
                return vec![(front + 1) as i32, (back + 1) as i32];
            } else if cur_sum > target {
                back -= 1;
            } else {
                front += 1;
            }
        }
    }
}
