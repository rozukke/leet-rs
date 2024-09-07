/// Complexity: O(n) => Maximum of 1 iteration through input
///
/// Use a stack to keep track of the most recent open bracket, check that it matches eack closing
/// bracket encountered and then pop off the stack. Stack must be empty in the end to imply that
/// every bracket was closed.
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                c => {
                    if stack.pop() != Some(c) {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}
