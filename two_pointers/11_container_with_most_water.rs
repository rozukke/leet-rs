impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        // Area only increases if one of the sides increases in height compared to the minimum. As
        // a result, only move up one of the sides if it is smaller than the next upcoming bound.
        // Keep track of area through these iterations to achieve maximum.
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max = -1;

        while left != right {
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }

            let area = (right - left) as i32 * height[left].min(height[right]);
            max = max.max(area);
        }
        max
    }
}
