/// Complexity: O(n) => 1 iteration through input
///
/// We collect the character frequency from each input word, which we can then use as a key, since
/// anagrams share character frequencies. The resulting map is then simply converted to a vector.
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();

        for elem in strs {
            let mut key = [0; 26];
            for c in elem.bytes() {
                key[(c - b'a') as usize] += 1;
            }

            map.entry(key).or_insert(Vec::new()).push(elem);
        }

        map.into_values().collect()
    }
}
