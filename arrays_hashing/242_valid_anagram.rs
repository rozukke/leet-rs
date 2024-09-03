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
