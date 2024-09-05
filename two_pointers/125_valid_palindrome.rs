/// Complexity: O(n) => Technically half an iteration through the input
///
/// Go from both ends of the string, false when any characters don't match. Pretty self
/// explanatory.
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // Use of a .map instead of casting the whole thing .to_ascii_lowercase prevents unnecessary
        // memory usage when creating a new lowercase string.
        let mut s_iter = s
            .chars()
            .filter(|&c| char::is_alphanumeric(c))
            .map(|c| c.to_ascii_lowercase());

        // Could also clone the iterator and compare with the reverse one
        while let (Some(front), Some(back)) = (s_iter.next(), s_iter.next_back()) {
            if front != back {
                return false;
            }
        }
        true
    }
}
