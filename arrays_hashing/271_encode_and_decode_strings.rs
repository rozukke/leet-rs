/// Complexity: O(n) => 1 iteration through input in both cases
///
/// We ensure that delimiters are not ever possible as part of the input strings by encoding the
/// length of each string as part of the delimiter. 'aaa' would be encoded as '3:aaa', so that
/// after taking in the value up to the ':' delimiter, we get a length value of 3 that is guaranteed to
/// contain the full string. After that, we can immediately assume that it is followed by another
/// delimiter and restart the process.
impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        // Average word length is 5, add 2 characters for delimiter and length
        let mut builder = String::with_capacity(strs.len() * 7);
        for st in strs {
            builder.push_str(&st.len().to_string());
            builder.push(':');
            builder.push_str(&st);
        }

        builder
    }

    pub fn decode(str: String) -> Vec<String> {
        let mut res = Vec::with_capacity(str.len() / 7);
        let mut iter = &str[..];

        while !iter.is_empty() {
            // Get prefix of str length
            let prefix_ct = iter.chars().take_while(|&c| c != ':').count();
            let prefix = &iter[..prefix_ct];
            // Skip delimiter
            iter = &iter[prefix_ct + 1..];

            let str_count = iter
                .chars()
                .take(usize::from_str_radix(prefix, 10).unwrap())
                .count();
            res.push(iter[..str_count].to_string());
            // Move to next
            iter = &iter[str_count..]
        }

        res
    }
}
