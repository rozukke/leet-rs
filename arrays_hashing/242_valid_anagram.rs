/// Complexity: O(n) => 1 iteration through input
///
/// Instead of using a hash map, we can use a single array of length 26 so that each character can
/// be its own hash. We then go through and cancel out each matching letter from both strings so
/// that any value in the map that is not zero means that there was an unequal number of that
/// character between both strings.
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map: [i16; 26] = [0; 26];

        for (cs, ct) in s.bytes().zip(t.bytes()) {
            map[(cs - b'a') as usize] += 1;
            map[(ct - b'a') as usize] -= 1;
        }

        map.iter().all(|&c| c == 0)
    }
}
