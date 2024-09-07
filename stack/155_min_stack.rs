/// Complexity: O(1) => All accesses are constant time
///
/// To keep track of the minimum, we have a secondary stack that is updated every time a new value
/// is pushed that is smaller or equal to the minimum. On pop, we check if that value was the
/// minimum on the stack and update accordingly.
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        match self.min_stack.last() {
            Some(min_val) if *min_val >= val => {
                self.min_stack.push(val);
            }
            None => self.min_stack.push(val),
            _ => (),
        }
        self.stack.push(val)
    }

    fn pop(&mut self) -> i32 {
        let pop_val = self.stack.pop().unwrap();
        match self.min_stack.last() {
            Some(min_val) if *min_val == pop_val => {
                self.min_stack.pop();
            }
            _ => (),
        }
        pop_val
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}
