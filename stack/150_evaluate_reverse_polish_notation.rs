/// Complexity: O(n) => Maximum one iteration through input
///
/// This is supposed to be a stack based problem, but I decided to make it into a recursive one. We
/// reverse through the sequence, and parse the two expressions following any operator to get the
/// result. Since every expression is guaranteed to evaluate, the code looks quite nice.
use std::{iter::Rev, vec::IntoIter};

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut toks = tokens.into_iter().rev();
        Solution::eval(&mut toks)
    }

    fn eval(iter: &mut Rev<IntoIter<String>>) -> i32 {
        // We can be brazen since the correct number of tokens is guaranteed
        let tok = Solution::parse(iter.next().unwrap());
        match tok {
            ValueType::Number(n) => n,
            op => {
                let a = Solution::eval(iter);
                let b = Solution::eval(iter);
                Solution::calc(b, op, a)
            }
        }
    }

    fn calc(a: i32, op: ValueType, b: i32) -> i32 {
        match op {
            ValueType::Plus => a + b,
            ValueType::Minus => a - b,
            ValueType::Div => a / b,
            ValueType::Mult => a * b,
            _ => unreachable!(),
        }
    }

    fn parse(token: String) -> ValueType {
        match token.as_str() {
            "+" => ValueType::Plus,
            "-" => ValueType::Minus,
            "/" => ValueType::Div,
            "*" => ValueType::Mult,
            token if matches!(token.chars().next().unwrap(), '-' | '0'..='9') => {
                ValueType::Number(token.parse::<i32>().unwrap())
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum ValueType {
    Number(i32),
    Plus,
    Minus,
    Div,
    Mult,
}
